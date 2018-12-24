use crate::backend::{Backend, instance};

#[derive(Debug)]
pub struct ImageData {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}

impl Drop for ImageData {
    fn drop(&mut self) {
        unsafe { instance().destroy_texture(self) };
    }
}
