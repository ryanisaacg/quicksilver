extern crate gl;
extern crate image;

use gl::types::*;
use geom::{Rectangle, Vector};
use std::os::raw::c_void;
use std::ops::Drop;
use std::path::Path;

pub enum PixelFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    BGR = gl::BGR as isize,
    BGRA = gl::BGRA as isize,
}

pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
}

impl Texture {
    pub fn from_raw(data: &[u8], w: i32, h: i32, format: PixelFormat) -> Texture {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as GLint,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                w,
                h,
                0,
                format as u32,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Texture {
                id: texture,
                width: w,
                height: h,
            }
        }
    }

    pub(crate) fn load(path: &Path) -> Result<Texture, image::ImageError> {
        let img = image::open(path)?.to_rgba();
        let width = img.width() as i32;
        let height = img.height() as i32;
        Result::Ok(Texture::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA))
    }

    pub fn region(&self) -> TextureRegion {
        TextureRegion {
            id: self.id,
            width: self.width,
            height: self.height,
            region: Rectangle::newi_sized(self.width, self.height),
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id as *const u32);
        }
    }
}

#[derive(Clone, Copy)]
pub struct TextureRegion {
    id: u32,
    width: i32,
    height: i32,
    region: Rectangle,
}

impl TextureRegion {
    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn source_width(&self) -> i32 {
        self.width
    }

    pub fn source_height(&self) -> i32 {
        self.height
    }

    pub fn source_size(&self) -> Vector {
        Vector::newi(self.source_width(), self.source_height())
    }

    pub fn get_region(&self) -> Rectangle {
        self.region
    }

    pub fn subregion(&self, rect: Rectangle) -> TextureRegion {
        TextureRegion {
            id: self.id,
            width: self.width,
            height: self.height,
            region: Rectangle::new(
                self.region.x + rect.x,
                self.region.y + rect.y,
                rect.width,
                rect.height,
            ),
        }
    }
}
