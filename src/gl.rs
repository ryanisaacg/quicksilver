extern crate gl;

#[cfg(not(target_arch="wasm32"))]
pub use self::gl::*;


#[cfg(target_arch="wasm32")]
pub use gl::{BGR, BGRA, CLAMP_TO_EDGE, LINEAR, RGB, RGBA, TEXTURE_2D, 
    TEXTURE_MIN_FILTER, TEXTURE_MAG_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNSIGNED_BYTE};
use std::os::raw::c_void;
#[cfg(target_arch="wasm32")]
extern "C" {
    pub fn BindTexture(textype: u32, index: u32);
    pub fn DeleteTextures(index: i32, data: *const u32);
    pub fn GenerateMipmap(enum_type: u32);
    pub fn GenTextures(index: i32, data: *mut u32);
    pub fn TexImage2D(
        target: u32, 
        level: i32, 
        internalformat: i32, 
        width: i32, 
        height: i32, 
        border: i32, 
        format: u32, 
        type_: u32, 
        pixels: *const c_void
    );
    pub fn TexParameteri(textype: u32, filtertype: u32, value: i32);
    pub fn Viewport(x: i32, y: i32, w: i32, h: i32);
}
