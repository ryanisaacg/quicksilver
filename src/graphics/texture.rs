extern crate gl;
extern crate imagefmt;

use gl::types::*;
use geom::Rectangle;
use std::os::raw::c_void;
use std::ops::Drop;
use std::path::Path;

pub enum PixelFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    BGR = gl::BGR as isize,
    BGRA = gl::BGRA as isize 
}

pub struct Texture {
    id: u32,
    width: i32,
    height: i32
}

impl Texture {
    pub fn from_raw(data: &[u8], w: i32, h: i32, format: PixelFormat) -> Texture {
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture as *mut GLuint);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as GLint, w, h, 0, format as u32, 
                           gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Texture {
                id: texture,
                width: w,
                height: h,
            }
        }
    }

    pub fn load(path: &Path) -> Result<Texture, imagefmt::Error> {
        let data = imagefmt::read(path, imagefmt::ColFmt::RGBA)?;
        let format = match data.fmt {
            imagefmt::ColFmt::RGB => Result::Ok(PixelFormat::RGB),
            imagefmt::ColFmt::RGBA => Result::Ok(PixelFormat::RGBA),
            imagefmt::ColFmt::BGR => Result::Ok(PixelFormat::BGR),
            imagefmt::ColFmt::BGRA => Result::Ok(PixelFormat::BGRA),
            _ => Result::Err(imagefmt::Error::Unsupported("Unsupported color format of loaded image"))
        };
        Result::Ok(Texture::from_raw(data.buf.as_slice(), data.w as i32, data.h as i32, format?))
    }

    pub fn region(&self) -> TextureRegion {
        TextureRegion {
            source: self,
            region: Rectangle::new_sized(self.width as f32, self.height as f32)
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

pub struct TextureRegion<'a> {
    source: &'a Texture,
    region: Rectangle
}

impl<'a> TextureRegion<'a> {
    pub fn get_id(&self) -> u32 {
        self.source.id
    }

    pub fn get_width(&self) -> i32 {
        self.source.width
    }

    pub fn get_height(&self) -> i32 {
        self.source.height
    }

    pub fn get_region(&self) -> Rectangle {
        self.region
    }

    pub fn subregion(&self, rect: Rectangle) -> TextureRegion {
        TextureRegion {
            source: self.source,
            region: Rectangle::new(self.region.x + rect.x, self.region.y + rect.y,
                                   rect.width, rect.height)
        }
    }
}
