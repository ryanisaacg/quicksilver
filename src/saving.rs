//! A module for saving / loading application data
//!
//! On Web, data may only be stored as a cookie in the user's browser. On the desktop, Windows,
//! macOS, and other Unix-style operating systems all have different locations where applications
//! should store data. This module allows any type that implements Serde serialize and deserialize
//! to be saved and loaded.

extern crate serde;
extern crate serde_json;

use error::QuicksilverError;
use serde::{Deserialize, Serialize};

///Save some arbitrary data to the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
///
///The appname should be some constant; this is used to name the file to place the save in on
///Desktop platforms. The profile should allow multiple saves of the same game (save slots,
///numbered saves, different players) etc.
pub fn save<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), QuicksilverError> {
    save_impl(appname, profile, data)
}

///Load some data from the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn load<T>(appname: &str, profile: &str) -> Result<T, QuicksilverError>
        where for<'de> T: Deserialize<'de> {
    load_impl(appname, profile)
}

#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch="wasm32"))]
use std::fs::File;

#[cfg(not(target_arch="wasm32"))]
fn get_save_folder(appname: &str) -> PathBuf {
    use std::env;
    let mut path = env::home_dir().unwrap();
    let location = if cfg!(windows) { "AppData" } 
        else if cfg!(target_os="macos") { "Library/Application Support" } 
        else { ".config" };
    path.push(location);
    path.push(appname);
    path
}

#[cfg(not(target_arch="wasm32"))]
fn get_save_location(appname: &str, profile: &str) -> PathBuf {
    let mut path = get_save_folder(appname);
    path.push(profile);
    path
}

#[cfg(not(target_arch="wasm32"))]
fn save_impl<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), QuicksilverError> {
    use std::fs::DirBuilder;
    DirBuilder::new().recursive(true).create(get_save_folder(appname)).unwrap();
    Ok(serde_json::to_writer(File::create(get_save_location(appname, profile)).unwrap(), data)?)
}

#[cfg(not(target_arch="wasm32"))]
fn load_impl<T>(appname: &str, profile: &str) -> Result<T, QuicksilverError> 
        where for<'de> T: Deserialize<'de> {
    Ok(serde_json::from_reader(File::open(get_save_location(appname, profile)).unwrap())?)
}

#[cfg(target_arch="wasm32")]
fn save_impl<T: Serialize>(_appname: &str, profile: &str, data: &T) -> Result<(), QuicksilverError> {
    use stdweb::web;
    let storage = web::window().session_storage();
    storage.insert(profile, serde_json::to_string(data)?.as_str()).unwrap();
    Ok(())
}

#[cfg(target_arch="wasm32")]
fn load_impl<T>(_appname: &str, profile: &str) -> Result<T, QuicksilverError>
        where for<'de> T: Deserialize<'de> {
    use stdweb::web;
    let storage = web::window().session_storage();
    match storage.get(profile) {
        Some(string) => Ok(serde_json::from_str(string.as_str())?),
        None => Err(QuicksilverError::SaveNotFound(profile.to_string()))
    }
}
