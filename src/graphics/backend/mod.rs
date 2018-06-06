use graphics::{Color, GpuTriangle, Vertex};

pub trait Backend {
    fn new(texture_mode: ImageScaleStrategy) -> Self where Self: Sized;
    fn clear(&mut self, col: Color);
    fn set_blend_mode(&mut self, blend: BlendMode);
    fn reset_blend_mode(&mut self);
    fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]);
    fn flush(&mut self);
}

const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

mod blend_mode;
#[cfg(not(target_arch="wasm32"))]
mod gl3;
#[cfg(target_arch="wasm32")]
mod webgl;

pub use self::blend_mode::BlendMode;
#[cfg(not(target_arch="wasm32"))]
pub use self::gl3::GL3Backend;
#[cfg(target_arch="wasm32")]
pub use self::webgl::WebGLBackend;

#[cfg(not(target_arch="wasm32"))]
pub type BackendImpl = GL3Backend;
#[cfg(target_arch="wasm32")]
pub type BackendImpl = WebGLBackend;

/// The way the images should change when drawn at a scale
#[repr(u32)]
#[derive(Debug)]
pub enum ImageScaleStrategy {
    /// The image should attempt to preserve each pixel as accurately as possible
    Pixelate,
    /// The image should attempt to preserve the overall picture by blurring
    Blur
}