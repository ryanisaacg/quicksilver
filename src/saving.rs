//! A module for saving / loading application data
//!
//! On Web, data may only be stored as a cookie in the user's browser. On the desktop, Windows,
//! macOS, and other Unix-style operating systems all have different locations where applications
//! should store data. This module allows any type that implements Serde serialize and deserialize
//! to be saved and loaded.

extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Error;

///Save some arbitrary data to the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
///
///The appname should be some constant; this is used to name the file to place the save in on
///Desktop platforms. The profile should allow multiple saves of the same game (save slots,
///numbered saves, different players) etc.
pub fn save<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), Error> {
    save_impl(appname, profile, data)
}

///Load some data from the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn load<T>(appname: &str, profile: &str) -> Result<T, Error>
        where for<'de> T: Deserialize<'de> {
    load_impl(appname, profile)
}

#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;
#[cfg(not(target_arch="wasm32"))]
use std::fs::File;

#[cfg(not(target_arch="wasm32"))]
fn get_save_location(appname: &str, profile: &str) -> PathBuf{
    use std::env;
    let mut path = env::home_dir().unwrap();
    let location = if cfg!(windows) { "AppData" } 
        else if cfg!(target_os="macos") { "Library/Application Support" } 
        else { ".config" };
    path.push(location);
    path.push(appname);
    path.push(profile);
    path
}

#[cfg(not(target_arch="wasm32"))]
fn save_impl<T: Serialize>(appname: &str, profile: &str, data: &T) -> Result<(), Error> {
    serde_json::to_writer(File::create(get_save_location(appname, profile)).unwrap(), data)
}

#[cfg(not(target_arch="wasm32"))]
fn load_impl<T>(appname: &str, profile: &str) -> Result<T, Error> 
        where for<'de> T: Deserialize<'de> {
    serde_json::from_reader(File::open(get_save_location(appname, profile)).unwrap())
}

#[cfg(target_arch="wasm32")]
use std::ffi::CString;

#[cfg(target_arch="wasm32")]
fn save_impl<T: Serialize>(_appname: &str, profile: &str, data: &T) -> Result<(), Error> {
    extern "C" {
        fn save_cookie(key: *const i8, val: *const i8);
    }
    let key = CString::new(profile).unwrap().into_raw();
    let val = CString::new(serde_json::to_string(data)?).unwrap().into_raw();
    unsafe { save_cookie(key, val) };
    Ok(())
}

#[cfg(target_arch="wasm32")]
fn load_impl<T>(_appname: &str, profile: &str) -> Result<T, Error>
        where for<'de> T: Deserialize<'de> {
    extern "C" {
        fn get_cookie_length(key: *const i8) -> usize;
        fn load_cookie(key: *const i8, val: *mut i8);
    }
    let key = CString::new(profile).unwrap().into_raw();
    let length = unsafe { get_cookie_length(key) };
    let buffer = Vec::with_capacity(length).as_mut_slice().as_mut_ptr();
    unsafe { load_cookie(key, buffer) };
    let string = unsafe { CString::from_raw(buffer) }.into_string().unwrap();
    serde_json::from_str(string.as_str())
}
