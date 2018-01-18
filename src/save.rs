extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Error;

///Save some arbitrary data to the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn save<T: Serialize>(profile: &str, data: T) {
    //todo: save the data
}

#[cfg(not(target_arch="wasm32"))]
fn save_impl<T: Serialize>(profile: &str, data: T) -> Result<(), Error> {

}

#[cfg(target_arch="wasm32")]
fn save_impl<T: Serialize>(profile: &str, data: T) -> Result<(), Error> {
    extern "C" {
        fn save_cookie(key: *const i8, val: *const i8);
    }
    use std::ffi::CString;
    let key = CString::new(profile).unwrap().into_raw();
    let val = CString::new(serde_json::to_string(&data)?).unwrap().into_raw();
    unsafe { save_cookie(key, val) };
    Ok(())
}

///Load some data from the given profile
///
///Different platforms may have different save locations: on the Web, data is saved in cookies, on
///the desktop, it is stored in some appropriate home-directory folder.
pub fn load<T: Deserialize>(profile: &str) -> Result<T, Error> {
    //todo: load the data
}

#[cfg(target_arch="wasm32")]
fn load_impl<T: Deserialize>(profile: &str) -> Result<T, Error> {
    extern "C" {
        fn get_cookie_length(key: *const i8) -> usize;
        fn load_cookie(key: *const i8, val: *mut i8);
    }
    let key = CString::new(profile).unwrap().into_raw();
    let length = unsafe { get_cookie_length(key) };
    let buffer = Vec::with_capacity(length).as_mut_slice().as_ptr();
    unsafe { load_cookie(key, buffer) };
    let string = CString::from_raw(buffer).into_string().unwrap();
    serde_json::from_str(string)
}
