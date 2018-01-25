extern crate gl;


pub use self::gl::{RGBA, DEPTH_BUFFER_BIT, ONE_MINUS_SRC_ALPHA, TEXTURE_MAG_FILTER, TRUE, UNSIGNED_INT, BLEND, FRAGMENT_SHADER, FRAMEBUFFER, VERTEX_SHADER, LINEAR, RGB, STREAM_DRAW, ARRAY_BUFFER, TEXTURE_MIN_FILTER, ELEMENT_ARRAY_BUFFER, TRIANGLES, FALSE, BGRA, BGR, TEXTURE_WRAP_T, UNSIGNED_BYTE, COLOR_BUFFER_BIT, FLOAT, TEXTURE_WRAP_S, INVALID_VALUE, TEXTURE, COMPILE_STATUS, SRC_ALPHA, CLAMP_TO_EDGE, TEXTURE_2D, TEXTURE0, VIEWPORT, COLOR_ATTACHMENT0, NEAREST};

use std::os::raw::c_void;

#[allow(non_snake_case)]
extern "C" {
    pub fn ActiveTexture(tex: u32);
    pub fn AttachShader(program: u32, shader: u32);
    pub fn Clear(buffer: u32);
    pub fn ClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn CompileShader(shader: u32);
    pub fn CreateShader(shader_type: u32) -> u32;
    pub fn CreateProgram() -> u32;
    pub fn BindBuffer(target: u32, buffer: u32);
    pub fn BindFramebuffer(target: u32, buffer: u32);
    pub fn BindTexture(textype: u32, index: u32);
    pub fn BindVertexArray(array: u32);
    pub fn BlendFunc(src: u32, dst: u32);
    pub fn BufferData(target: u32, size: isize, data: *const c_void, usage: u32);
    pub fn BufferSubData(target: u32, offset: i32, size: isize, data: *const c_void);
    pub fn DeleteBuffer(buffer: u32);
    pub fn DeleteFramebuffer(index: u32);
    pub fn DeleteProgram(index: u32);
    pub fn DeleteShader(index: u32);
    pub fn DeleteTexture(index: u32);
    pub fn DeleteVertexArray(array: u32);
    pub fn DrawBuffer(buffer: u32);
    pub fn DrawElements(mode: u32, count: i32, elem_type: u32, indices: *const c_void);
    pub fn Enable(feature: u32);
    pub fn EnableVertexAttribArray(index: u32);
    pub fn FramebufferTexture(target: u32, attachment: u32, texture: u32, level: u32);
    pub fn GenBuffer() -> u32;
    pub fn GenerateMipmap(texture: u32);
    pub fn GenFramebuffer() -> u32;
    pub fn GenTexture() -> u32;
    pub fn GenVertexArray() -> u32;
    pub fn GetAttribLocation(program: u32, name: *const i8) -> i32;
    pub fn GetShaderInfoLog(shader: u32, max_length: isize, length: *mut i32, log: *mut i8);
    pub fn GetShaderiv(shader: u32, name: u32, params: *mut i32);
    pub fn GetViewport(target: *mut i32);
    pub fn GetUniformLocation(program: u32, name: *const i8) -> i32;
    pub fn LinkProgram(shader: u32);
    pub fn ShaderSource(shader: u32, string: *const i8);
    pub fn TexImage2D(target: u32, level: i32, internal: i32, width: i32, height: i32, border: i32, format: u32, textype: u32, data: *const c_void);
    pub fn TexParameteri(target: u32, param: u32, pname: i32);
    pub fn Uniform1i(location: i32, index: u32);
    pub fn UseProgram(program: u32);
    pub fn VertexAttribPointer(index: u32, size: i32, attr_type: u32, norm: u8, stride: i32, ptr: *const c_void);
    pub fn Viewport(x: i32, y: i32, w: i32, h: i32);
}
