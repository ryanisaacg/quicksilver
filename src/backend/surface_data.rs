use crate::backend::{Backend, BackendImpl};

#[derive(Debug)]
#[cfg(not(target_arch = "wasm32"))]
pub struct SurfaceData {
    pub framebuffer: u32
}

#[cfg(target_arch="wasm32")]
use webgl_stdweb::WebGLFramebuffer;

#[derive(Debug)]
#[cfg(target_arch="wasm32")]
pub struct SurfaceData {
    pub framebuffer: WebGLFramebuffer
}


impl Drop for SurfaceData {
    fn drop(&mut self) {
        unsafe {
            BackendImpl::destroy_surface(self);
        }
    }
}
