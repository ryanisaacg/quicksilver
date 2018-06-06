use graphics::{Color, GpuTriangle, Image, ImageData, PixelFormat, Surface, SurfaceData, Vertex};
use std::os::raw::c_void;

pub(crate) trait Backend {
    fn new(texture_mode: ImageScaleStrategy) -> Self where Self: Sized;
    fn clear(&mut self, col: Color);
    fn set_blend_mode(&mut self, blend: BlendMode);
    fn reset_blend_mode(&mut self);
    fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]);
    fn flush(&mut self);
    fn create_texture(data: *const c_void, width: u32, height: u32, format: PixelFormat) -> ImageData where Self: Sized;
    fn destroy_texture(data: &mut ImageData) where Self: Sized;
    fn create_surface(image: &Image) -> SurfaceData where Self: Sized;
    fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized;
    fn unbind_surface(surface: &Surface, viewport: &[i32]) where Self: Sized;
    fn destroy_surface(surface: &SurfaceData) where Self: Sized;
    fn viewport(x: i32, y: i32, width: i32, height: i32) where Self: Sized;
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
pub(crate) type BackendImpl = GL3Backend;
#[cfg(target_arch="wasm32")]
pub(crate) type BackendImpl = WebGLBackend;

/// The way the images should change when drawn at a scale
#[repr(u32)]
#[derive(Debug)]
pub enum ImageScaleStrategy {
    /// The image should attempt to preserve each pixel as accurately as possible
    Pixelate,
    /// The image should attempt to preserve the overall picture by blurring
    Blur
}