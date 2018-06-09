use graphics::backend::{Backend, BackendImpl};

#[derive(Debug)]
#[cfg(not(target_arch = "wasm32"))]
pub struct SurfaceData {
    pub framebuffer: u32
}

#[cfg(target_arch="wasm32")]
use webgl_stdweb::WebGLFrambuffer;

#[derive(Debug)]
#[cfg(target_arch="wasm32")]
pub struct SurfaceData {
    pub framebuffer: WebGLFrambuffer
}


impl Drop for SurfaceData {
    fn drop(&mut self) {
        BackendImpl::destroy_surface(self);
    }
}