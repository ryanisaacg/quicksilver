use { 
    Result,
    graphics::{ Background::Col, Color, GpuTriangle, Image, PixelFormat, Surface, Vertex }
};

pub(crate) trait Backend {
    type Platform;

    unsafe fn new(platform: Self::Platform, texture_mode: ImageScaleStrategy) -> Result<Self> where Self: Sized;
    unsafe fn clear(&mut self, color: Color);
    unsafe fn set_blend_mode(&mut self, blend: BlendMode);
    unsafe fn reset_blend_mode(&mut self);
    unsafe fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]) -> Result<()>;
    unsafe fn flush(&mut self);
    unsafe fn create_texture(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<ImageData> where Self: Sized;
    unsafe fn destroy_texture(data: &mut ImageData) where Self: Sized;
    unsafe fn create_surface(image: &Image) -> Result<SurfaceData> where Self: Sized;
    unsafe fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized;
    unsafe fn unbind_surface(surface: &Surface, viewport: &[i32]) where Self: Sized;
    unsafe fn destroy_surface(surface: &SurfaceData) where Self: Sized;
    unsafe fn viewport(x: i32, y: i32, width: i32, height: i32) where Self: Sized;

    unsafe fn clear_color(&mut self, color: Color, letterbox: Color) -> Result<()> {
        self.clear(letterbox);
        self.draw(&[
            Vertex::new((-1, -1), None, Col(color)),
            Vertex::new((1, -1), None, Col(color)),
            Vertex::new((1, 1), None, Col(color)),
            Vertex::new((-1, 1), None, Col(color)),
        ], &[
            GpuTriangle::new(0, [0, 1, 2], 0.0, Col(color)),
            GpuTriangle::new(0, [2, 3, 0], 0.0, Col(color))
        ])?;
        self.flush();
        Ok(())
    }
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
