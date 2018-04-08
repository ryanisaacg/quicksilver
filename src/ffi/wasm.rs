use std::os::raw::c_void;
use std::io::ErrorKind as IOError;

#[allow(improper_ctypes)]
extern "C" {
    //Windowing
    pub fn set_show_mouse(show: bool);
    pub fn get_page_width() -> u32;
    pub fn get_page_height() -> u32;
    pub fn create_context(title: *mut i8, width: u32, height: u32);
    pub fn set_title(title: *mut i8);
    //Mouse input
    pub fn get_mouse_x() -> f32;
    pub fn get_mouse_y() -> f32;
    pub fn pump_mouse_queue() -> i32;
    pub fn mouse_scroll_clear();
    pub fn mouse_scroll_type() -> u32;
    pub fn mouse_scroll_x() -> f32;
    pub fn mouse_scroll_y() -> f32;
    //Keyboard input
    pub fn pump_key_queue() -> i32;
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
    //Gamepads
    pub fn gamepads_update();
    pub fn gamepads_length() -> u32;
    pub fn gamepads_id(index: u32) -> u32;
    pub fn gamepad_axis(id: u32, axis: u32) -> f32;
    pub fn gamepad_button(id: u32, button: u32) -> bool;
    //Asset loading
    fn ffi_asset_status(handle: u32) -> i32;
    //Game loop
    pub fn set_app(app: *mut c_void);
}

pub fn asset_status(handle: u32) -> Result<bool, IOError> {
    use std::io::ErrorKind;
    match unsafe { ffi_asset_status(handle) } {
        0 => Ok(false),
        1 => Ok(true),
        2 => Err(ErrorKind::NotFound),
        _ => unreachable!()
    }
}
