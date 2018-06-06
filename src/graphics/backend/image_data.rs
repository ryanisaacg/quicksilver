use graphics::backend::{Backend, BackendImpl};

#[derive(Debug)]
#[cfg(not(target_arch = "wasm32"))]
pub struct ImageData {
    pub(crate) id: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl Drop for ImageData {
    fn drop(&mut self) {
        BackendImpl::destroy_texture(self);
    }
}
