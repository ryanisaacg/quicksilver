use graphics::{Color, GpuTriangle, Image, PixelFormat, Surface, Vertex};

pub(crate) trait Backend {
    fn new(texture_mode: ImageScaleStrategy) -> Self where Self: Sized;
    fn clear(&mut self, col: Color);
    fn set_blend_mode(&mut self, blend: BlendMode);
    fn reset_blend_mode(&mut self);
    fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]);
    fn flush(&mut self);
    fn create_texture(data: &[u8], width: u32, height: u32, format: PixelFormat) -> ImageData where Self: Sized;
    fn destroy_texture(data: &mut ImageData) where Self: Sized;
    fn create_surface(image: &Image) -> SurfaceData where Self: Sized;
    fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized;
    fn unbind_surface(surface: &Surface, viewport: &[i32]) where Self: Sized;
    fn destroy_surface(surface: &SurfaceData) where Self: Sized;
    fn viewport(x: i32, y: i32, width: i32, height: i32) where Self: Sized;
}

const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

mod blend_mode;
mod image_data;
mod surface_data;

pub use self::blend_mode::BlendMode;
pub use self::image_data::ImageData;
pub use self::surface_data::SurfaceData;

// Backends
#[cfg(not(target_arch="wasm32"))]
mod gl3;
#[cfg(target_arch="wasm32")]
mod webgl;

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