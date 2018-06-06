use graphics::backend::{Backend, BackendImpl};

#[derive(Debug)]
#[cfg(not(target_arch = "wasm32"))]
pub struct SurfaceData {
    pub framebuffer: u32
}

impl Drop for SurfaceData {
    fn drop(&mut self) {
        BackendImpl::destroy_surface(self);
    }
}