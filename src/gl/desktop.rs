#![allow(non_snake_case)]

extern crate gl;

pub use self::gl::*;

pub unsafe fn DeleteTexture(id: u32) {
    gl::DeleteTextures(1, &id as *const u32);
}

pub unsafe fn DeleteBuffer(buffer: u32) {
    gl::DeleteBuffers(1, &buffer as *const u32);
}

pub unsafe fn DeleteVertexArray(array: u32) {
    gl::DeleteVertexArrays(1, &array as *const u32);
}

pub unsafe fn GenBuffer() -> u32 {
    let mut buffer = 0;
    gl::GenBuffers(1, &mut buffer as *mut u32);
    buffer
}

pub unsafe fn GenTexture() -> u32 {
    let mut texture = 0;
    gl::GenTextures(1, &mut texture as *mut u32);
    texture
}

pub unsafe fn GenVertexArray() -> u32 {
    let mut array = 0;
    gl::GenVertexArrays(1, &mut array as *mut u32);
    array
}

pub unsafe fn ShaderSource(shader: u32, string: *const i8) {
    use std::ptr::null;
    gl::ShaderSource(shader, 1, &(string) as *const *const i8, null());
}

