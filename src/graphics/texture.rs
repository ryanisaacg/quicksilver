extern crate gl;

use gl::types::*;
use geom::Rectangle;
use std::os::raw::c_void;

pub enum PixelFormat {
    RGB = gl::RGB as isize,
    RGBA = gl::RGBA as isize,
    BGR = gl::BGR as isize,
    BGRA = gl::BGRA as isize 
}
//TODO: texture vs textureregion
pub struct Texture {
    pub id: u32,
    width: i32,
    height: i32,
    region: Rectangle
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
                region: Rectangle::new_sized(w as f32, h as f32)
            }
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

