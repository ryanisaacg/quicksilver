//! A module for saving / loading application data
//!
//! On Web, data may only be stored as a cookie in the user's browser. On the desktop, Windows,
//! macOS, and other Unix-style operating systems all have different locations where applications
//! should store data. This module allows any type that implements Serde serialize and deserialize
//! to be saved and loaded.

use serde::{Deserialize, Serialize};
use serde_json::{self, Error as SerdeError};
use std::{
    error::Error,
    fmt,
    io::Error as IOError
};

/// Save some arbitrary data to the given profile using Serde
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
///
/// The appname should be some constant; this is used to name the file to place the save in on
/// desktop platforms. The profile should allow multiple saves of the same game (save slots,
/// numbered saves, different players) etc.
///
/// The example shows how to round-trip some data. Note that for [load](fn.load.html) you must
/// explicitly specify the type of the data, this is because the struct is not passed as a
/// parameter to `load` so Rust cannot infer the type.
///
/// ```
/// # use quicksilver::saving::{save, load};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct Player {
///     name: String,
///     score: u32
/// }
///
/// let player1 = Player { name: "Bob".to_string(), score: 21 };
/// save("mygame", "player1", &player1).expect("Could not save Player 1");
///
/// let player2 = Player { name: "Alice".to_string(), score: 200 };
/// save("mygame", "player2", &player2).expect("Could not save Player 2");
///
/// // Now reload.
/// let player1 = load::<Player>("mygame", "player1").expect("Could not load Player 1");
/// let player2 = load::<Player>("mygame", "player2").expect("Could not load Player 2");
/// ```
pub fn save<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), SaveError> {
    save_impl(appname, profile, data)
}


/// Save some raw bytes to the given profile
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
///
/// The appname should be some constant; this is used to name the file to place the save in on
/// desktop platforms. The profile should allow multiple saves of the same game (save slots,
/// numbered saves, different players) etc.
pub fn save_raw(appname: &str, profile: &str, data: &[u8]) -> Result<(), SaveError> {
    save_raw_impl(appname, profile, data)
}

/// Load some data from the given profile using Serde
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
///
/// See [save](fn.save.html) for an example of saving and then loading some data.
pub fn load<T>(appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    load_impl(appname, profile)
}

/// Load some raw bytes from the given profile
///
/// Different platforms may have different save locations: on the Web, data is saved in local
/// storage, on the desktop, it is stored in some appropriate home-directory folder.
pub fn load_raw(appname: &str, profile: &str) -> Result<Vec<u8>, SaveError> {
    load_raw_impl(appname, profile)
}

#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch="wasm32"))]
use std::fs::File;
#[cfg(not(target_arch="wasm32"))]
use std::io::{Read, Write};


#[cfg(not(target_arch="wasm32"))]
fn get_save_folder(appname: &str) -> Result<PathBuf, SaveError> {
    let mut path = ::dirs::data_dir().ok_or(SaveError::SaveLocationNotFound)?;
    path.push(appname);
    Ok(path)
}

#[cfg(not(target_arch="wasm32"))]
fn get_save_location(appname: &str, profile: &str) -> Result<PathBuf, SaveError> {
    let mut path = get_save_folder(appname)?;
    path.push(profile);
    Ok(path)
}

#[cfg(not(target_arch="wasm32"))]
fn save_impl<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), SaveError> {
    use std::fs::DirBuilder;
    DirBuilder::new().recursive(true).create(get_save_folder(appname)?)?;
    Ok(serde_json::to_writer(File::create(get_save_location(appname, profile)?)?, data)?)
}

#[cfg(not(target_arch="wasm32"))]
fn save_raw_impl(appname: &str, profile: &str, data: &[u8]) -> Result<(), SaveError> {
    use std::fs::DirBuilder;
    DirBuilder::new().recursive(true).create(get_save_folder(appname)?)?;
    Ok(File::create(get_save_location(appname, profile)?)?.write_all(data)?)
}

#[cfg(not(target_arch="wasm32"))]
fn load_impl<T>(appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    Ok(serde_json::from_reader(File::open(get_save_location(appname, profile)?)?)?)
}

#[cfg(not(target_arch="wasm32"))]
fn load_raw_impl(appname: &str, profile: &str) -> Result<Vec<u8>, SaveError> {
    let mut buf = Vec::new();
    File::open(get_save_location(appname, profile)?)?.read_to_end(&mut buf)?;
    Ok(buf)
}

#[cfg(target_arch="wasm32")]
fn save_impl<T: Serialize>(_appname: &str, profile: &str, data: &T) -> Result<(), SaveError> {
    use stdweb::web;
    let storage = web::window().local_storage();
    match storage.insert(profile, serde_json::to_string(data)?.as_str()) {
        Ok(()) => Ok(()),
        Err(_) => Err(SaveError::SaveWriteFailed)
    }
}

#[cfg(target_arch="wasm32")]
fn save_raw_impl(_appname: &str, profile: &str, data: &[u8]) -> Result<(), SaveError> {
    use stdweb::web;
    use base64::encode;
    let storage = web::window().local_storage();
    match storage.insert(profile, encode(data).as_str()) {
        Ok(()) => Ok(()),
        Err(_) => Err(SaveError::SaveWriteFailed)
    }
}

#[cfg(target_arch="wasm32")]
fn load_impl<T>(_appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    use stdweb::web;
    let storage = web::window().local_storage();
    match storage.get(profile) {
        Some(string) => Ok(serde_json::from_str(string.as_str())?),
        None => Err(SaveError::SaveNotFound(profile.to_string()))
    }
}

#[cfg(target_arch="wasm32")]
fn load_raw_impl(_appname: &str, profile: &str) -> Result<Vec<u8>, SaveError> {
    use stdweb::web;
    use base64::decode;
    let storage = web::window().local_storage();
    match storage.get(profile) {
        Some(string) => decode(string.as_str()).map_err(|_| SaveError::DecodeError),
        None => Err(SaveError::SaveNotFound(profile.to_string()))
    }
}

#[derive(Debug)]
/// An error that can occur during a save or load operation
pub enum SaveError {
    /// Some serialization failed during save or load
    SerdeError(SerdeError),
    /// Save string is failed to decode (web-specific)
    DecodeError,
    /// Some IO failed during save or load
    IOError(IOError),
    /// The user has no home directory so no save or load location can be established
    SaveLocationNotFound,
    /// The save cannot be written (web-specific)
    SaveWriteFailed,
    /// The save profile with the given name was not found
    ///
    /// On desktop this will more likely be reported as an IO error, but on web it will be a
    /// SaveNotFound
    SaveNotFound(String)
}

impl From<SerdeError> for SaveError {
    fn from(err: SerdeError) -> SaveError {
        SaveError::SerdeError(err)
    }
}

impl From<IOError> for SaveError {
    fn from(err: IOError) -> SaveError {
        SaveError::IOError(err)
    }
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SaveError {
    fn description(&self) -> &str {
        match self {
            SaveError::SerdeError(err) => err.description(),
            SaveError::DecodeError => "Save is not valid base64 string",
            SaveError::IOError(err) => err.description(),
            SaveError::SaveWriteFailed => "The save could not be written to local storage",
            SaveError::SaveLocationNotFound => "The current user has no home directory",
            SaveError::SaveNotFound(_) => "The given save profile was not found"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match self {
            SaveError::SerdeError(err) => Some(err),
            SaveError::IOError(err) => Some(err),
            SaveError::SaveLocationNotFound
                | SaveError::SaveWriteFailed
                | SaveError::SaveNotFound(_)
                | SaveError::DecodeError => None
        }
    }
}
