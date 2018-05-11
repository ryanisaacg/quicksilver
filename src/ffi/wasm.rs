use std::{
    error::Error,
    fmt,
    io::Error as IOError,
    os::raw::c_void,
};
#[cfg(not(target_arch="wasm32"))]
use std::io::ErrorKind;

#[allow(improper_ctypes)]
extern "C" {
    //Windowing
    pub fn set_show_mouse(show: bool);
    pub fn get_page_width() -> u32;
    pub fn get_page_height() -> u32;
    pub fn create_context(title: *mut i8, width: u32, height: u32);
    pub fn set_title(title: *mut i8);
    //Event data
    pub fn event_data_button() -> u32;
    pub fn event_data_state() -> u32;
    pub fn event_data_f1() -> f32;
    pub fn event_data_f2() -> f32;
    pub fn event_data_id() -> u32;
    //Gamepads
    pub fn gamepad_count() -> u32;
    pub fn gamepad_data(start: *mut c_void, id: *mut u32, buttons: *mut u32, axes: *mut f32, next: *mut c_void);
    //Saving / loading
    pub fn save_cookie(key: *const i8, val: *const i8);
    pub fn load_cookie(key: *const i8) -> *mut i8;
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
    pub fn get_image_width(index: u32) -> i32;
    pub fn get_image_height(index: u32) -> i32;
    //Arbitrary files
    pub fn load_file(name: *mut i8) -> u32;
    pub fn file_contents(handle: u32) -> *mut u8;
    pub fn file_length(handle: u32) -> u32;
    //Asset loading
    fn ffi_asset_status(handle: u32) -> i32;
    //Game loop
    pub fn set_app(app: *mut c_void);
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
