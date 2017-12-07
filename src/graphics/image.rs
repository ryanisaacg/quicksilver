extern crate gl;
extern crate image;

use gl::types::*;
use geom::{Rectangle, Vector};
use std::os::raw::c_void;
use std::ops::Drop;
use std::path::Path;
use std::rc::Rc;

pub enum PixelFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    BGR = gl::BGR as isize,
    BGRA = gl::BGRA as isize,
}

pub struct ImageData {
    id: u32,
    width: i32,
    height: i32,
}

impl ImageData {
    pub fn from_raw(data: &[u8], w: i32, h: i32, format: PixelFormat) -> ImageData {
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
            ImageData {
                id: texture,
                width: w,
                height: h,
            }
        }
    }

    pub fn load(path: &Path) -> Result<ImageData, image::ImageError> {
        let img = image::open(path)?.to_rgba();
        let width = img.width() as i32;
        let height = img.height() as i32;
        Result::Ok(ImageData::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA))
    }

    pub fn image(self) -> Image {
        let region = Rectangle::newi_sized(self.width, self.height);
        Image {
            source: Rc::new(self),
            region
        }
    }
}

impl Drop for ImageData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id as *const u32);
        }
    }
}

#[derive(Clone)]
pub struct Image {
    source: Rc<ImageData>,
    region: Rectangle,
}

impl Image {
    pub(crate) fn get_id(&self) -> u32 {
        self.source.id
    }

    pub(crate) fn source_width(&self) -> i32 {
        self.source.width
    }

    pub(crate) fn source_height(&self) -> i32 {
        self.source.height
    }

    pub(crate) fn source_size(&self) -> Vector {
        Vector::newi(self.source_width(), self.source_height())
    }

    pub fn area(&self) -> Rectangle {
        self.region
    }

    pub fn subimage(&self, rect: Rectangle) -> Image {
        Image {
            source: self.source.clone(),
            region: Rectangle::new(
                self.region.x + rect.x,
                self.region.y + rect.y,
                rect.width,
                rect.height,
            ),
        }
    }
}
