use graphics::backend::{Backend, BackendImpl};

#[derive(Debug)]
#[cfg(not(target_arch = "wasm32"))]
pub struct ImageData {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_arch = "wasm32")]
use webgl_stdweb::WebGLTexture;

#[derive(Debug)]
#[cfg(target_arch = "wasm32")]
pub struct ImageData {
    pub data: WebGLTexture,
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl Drop for ImageData {
    fn drop(&mut self) {
        unsafe {
            BackendImpl::destroy_texture(self);
        }
    }
}
