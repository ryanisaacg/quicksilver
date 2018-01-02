extern crate gl;

#[cfg(not(target_arch="wasm32"))]
pub use self::gl::*;

#[cfg(not(target_arch="wasm32"))]
pub unsafe fn DeleteTexture(id: u32) {
    gl::DeleteTextures(1, &id as *const u32);
}

#[cfg(not(target_arch="wasm32"))]
pub unsafe fn DeleteBuffer(buffer: u32) {
    gl::DeleteBuffers(1, &buffer as *const u32);
}

#[cfg(not(target_arch="wasm32"))]
pub unsafe fn DeleteVertexArray(array: u32) {
    gl::DeleteVertexArrays(1, &array as *const u32);
}

#[cfg(not(target_arch="wasm32"))]
pub unsafe fn GenBuffer() -> u32 {
    let mut buffer = 0;
    gl::GenBuffers(1, &mut buffer as *mut u32);
    buffer
}

#[cfg(not(target_arch="wasm32"))]
pub unsafe fn GenTexture() -> u32 {
    let mut texture = 0;
    gl::GenTextures(1, &mut texture as *mut u32);
    texture
}


#[cfg(not(target_arch="wasm32"))]
pub unsafe fn GenVertexArray() -> u32 {
    let mut array = 0;
    gl::GenVertexArrays(1, &mut array as *mut u32);
    array
}


#[cfg(target_arch="wasm32")]
pub use self::gl::{RGBA, DEPTH_BUFFER_BIT, ONE_MINUS_SRC_ALPHA, TEXTURE_MAG_FILTER, TRUE, UNSIGNED_INT, BLEND, FRAGMENT_SHADER, VERTEX_SHADER, LINEAR, RGB, STREAM_DRAW, ARRAY_BUFFER, TEXTURE_MIN_FILTER, ELEMENT_ARRAY_BUFFER, TRIANGLES, FALSE, BGRA, BGR, TEXTURE_WRAP_T, UNSIGNED_BYTE, COLOR_BUFFER_BIT, FLOAT, TEXTURE_WRAP_S, INVALID_VALUE, TEXTURE, COMPILE_STATUS, SRC_ALPHA, CLAMP_TO_EDGE, TEXTURE_2D, TEXTURE0};
#[cfg(target_arch="wasm32")]
use std::os::raw::c_void;
#[cfg(target_arch="wasm32")]
extern "C" {
    pub fn ActiveTexture(tex: u32);
    pub fn AttachShader(program: u32, shader: u32);
    pub fn Clear(buffer: u32);
    pub fn ClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn CompileShader(shader: u32);
    pub fn CreateShader(shader_type: u32) -> u32;
    pub fn CreateProgram() -> u32;
    pub fn BindBuffer(target: u32, buffer: u32);
    pub fn BindTexture(textype: u32, index: u32);
    pub fn BindVertexArray(array: u32);
    pub fn BlendFunc(src: u32, dst: u32);
    pub fn BufferData(target: u32, size: isize, data: *const c_void, usage: u32);
    pub fn BufferSubData(target: u32, offset: i32, size: isize, data: *const c_void);
    pub fn DeleteBuffer(buffer: u32);
    pub fn DeleteProgram(index: u32);
    pub fn DeleteShader(index: u32);
    pub fn DeleteTexture(index: u32);
    pub fn DeleteVertexArray(array: u32);
    pub fn DrawElements(mode: u32, count: i32, elem_type: u32, indices: *const c_void);
    pub fn Enable(feature: u32);
    pub fn EnableVertexAttribArray(index: u32);
    pub fn GenBuffer() -> u32;
    pub fn GenerateMipmap(enum_type: u32);
    pub fn GenTexture() -> u32;
    pub fn GenVertexArray() -> u32;
    pub fn GetAttribLocation(program: u32, name: *const i8) -> i32;
    pub fn GetError() -> u32;
    pub fn GetShaderInfoLog(shader: u32, max_length: isize, length: *mut i32, log: *mut i8);
    pub fn GetShaderiv(shader: u32, name: u32, params: *mut i32);
    pub fn GetUniformLocation(program: u32, name: *const i8) -> i32;
    pub fn LinkProgram(shader: u32);
    pub fn ShaderSource(shader: u32, count: isize, string: *const *const i8, length: *const i32);
    pub fn TexParameteri(textype: u32, filtertype: u32, value: i32);
    pub fn Uniform1i(location: i32, index: u32);
    pub fn UseProgram(program: u32);
    pub fn VertexAttribPointer(index: u32, size: i32, attr_type: u32, norm: u8, stride: i32, ptr: *const c_void);
    pub fn Viewport(x: i32, y: i32, w: i32, h: i32);
}
