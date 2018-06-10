//! A module for saving / loading application data
//!
//! On Web, data may only be stored as a cookie in the user's browser. On the desktop, Windows,
//! macOS, and other Unix-style operating systems all have different locations where applications
//! should store data. This module allows any type that implements Serde serialize and deserialize
//! to be saved and loaded.

extern crate serde;
extern crate serde_json;

use serde_json::Error as SerdeError;

use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
    io::Error as IOError
};

///Save some arbitrary data to the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
///
///The appname should be some constant; this is used to name the file to place the save in on
///Desktop platforms. The profile should allow multiple saves of the same game (save slots,
///numbered saves, different players) etc.
pub fn save<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), SaveError> {
    save_impl(appname, profile, data)
}

///Load some data from the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn load<T>(appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    load_impl(appname, profile)
}

#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch="wasm32"))]
use std::fs::File;

#[cfg(not(target_arch="wasm32"))]
fn get_home() -> Result<PathBuf, SaveError> {
    use std::env;
    match env::home_dir() {
        Some(path) => Ok(path),
        None => Err(SaveError::SaveLocationNotFound)
    }
}

#[cfg(not(target_arch="wasm32"))]
fn get_save_folder(appname: &str) -> Result<PathBuf, SaveError> {
    let mut path = get_home()?;
    let location = if cfg!(windows) { "AppData" } 
        else if cfg!(target_os="macos") { "Library/Application Support" } 
        else { ".config" };
    path.push(location);
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
fn load_impl<T>(appname: &str, profile: &str) -> Result<T, SaveError> 
        where for<'de> T: Deserialize<'de> {
    Ok(serde_json::from_reader(File::open(get_save_location(appname, profile)?)?)?)
}

#[cfg(target_arch="wasm32")]
use std::ffi::CString;

#[cfg(target_arch="wasm32")]
fn save_impl<T: Serialize>(_appname: &str, profile: &str, data: &T) -> Result<(), SaveError> {
    use ffi::wasm;
    let key = CString::new(profile).unwrap().into_raw();
    let val = CString::new(serde_json::to_string(data)?).unwrap().into_raw();
    unsafe { wasm::save_cookie(key, val) };
    Ok(())
}

#[cfg(target_arch="wasm32")]
fn load_impl<T>(_appname: &str, profile: &str) -> Result<T, SaveError>
        where for<'de> T: Deserialize<'de> {
    use ffi::wasm;
    let key = CString::new(profile).unwrap().into_raw();
    let string = unsafe { CString::from_raw(wasm::load_cookie(key)) }.into_string().unwrap();
    Ok(serde_json::from_str(string.as_str())?)
}

#[derive(Debug)]
/// An error that can occur during a save or load operation
pub enum SaveError {
    /// Some serialization failed during save or load
    SerdeError(SerdeError),
    /// Some IO failed during save or load
    IOError(IOError),
    /// The user has no home directory so no save or load location can be established
    SaveLocationNotFound
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
            SaveError::IOError(err) => err.description(),
            SaveError::SaveLocationNotFound => "The current user has no home directory"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            SaveError::SerdeError(err) => Some(err),
            SaveError::IOError(err) => Some(err),
            SaveError::SaveLocationNotFound => None
        }
    }
}