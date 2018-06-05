use std::{
    error::Error,
    fmt,
    io::Error as IOError,
};
#[cfg(not(target_arch="wasm32"))]
use std::io::ErrorKind;

#[allow(improper_ctypes)]
extern "C" {
    //Sounds
    pub fn load_sound(path: *mut i8) -> u32;
    pub fn play_sound(index: u32, volume: f32);
    pub fn set_music_track(index: u32);
    pub fn play_music();
    pub fn pause_music();
    pub fn get_music_volume() -> f32;
    pub fn set_music_volume(volume: f32);
    //Images
    pub fn load_image(name: *mut i8) -> u32; 
    pub fn get_image_id(index: u32) -> u32;
    pub fn get_image_width(index: u32) -> u32;
    pub fn get_image_height(index: u32) -> u32;
    //Asset loading
    fn ffi_asset_status(handle: u32) -> i32;
}

#[derive(Debug)]
struct WasmIOError;

impl fmt::Display for WasmIOError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for WasmIOError {
    fn description(&self) -> &str {
        "An error occurred during a file IO operation"
    }
}

pub fn asset_status(handle: u32) -> Result<bool, IOError> {
    use std::io::ErrorKind;
    match unsafe { ffi_asset_status(handle) } {
        0 => Ok(false),
        1 => Ok(true),
        2 => Err(IOError::new(ErrorKind::NotFound, Box::new(WasmIOError))),
        _ => unreachable!()
    }
}
