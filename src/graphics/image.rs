#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(target_arch="wasm32")]
use bridge;

use gl;
use geom::{Rectangle, Vector};
use std::ffi::CString;
use std::io;
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

struct ImageData {
    id: u32,
    width: i32,
    height: i32,
}

impl Drop for ImageData {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTexture(self.id);
        }
    }
}

#[derive(Clone)]
pub struct Image {
    source: Rc<ImageData>,
    region: Rectangle,
}

impl Image {
    #[cfg(not(target_arch="wasm32"))]
    pub fn from_raw(data: &[u8], width: i32, height: i32, format: PixelFormat) -> Image {
        let id = unsafe {
            let texture = gl::GenTexture();
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_EDGE as i32,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, format as u32, 
                           gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            texture
        };
        Image {
            source: Rc::new(ImageData { id, width, height }),
            region: Rectangle::newi_sized(width, height)
        }
    }

    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> Result<Image, ImageError> {
        let img = image::open(path)?.to_rgba();
        let width = img.width() as i32;
        let height = img.height() as i32;
        Ok(Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA))
    }
    
    #[cfg(target_arch="wasm32")]
    //TODO: create an image error that works across wasm and native
    fn load_impl<P: AsRef<Path>>(path: P) -> Result<Image, ImageError> {
        unsafe { bridge::load_image(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw());}
        let id = unsafe { bridge::get_image_id() };
        let width = unsafe { bridge::get_image_width() } ;
        let height = unsafe { bridge::get_image_height() };
        Ok(Image { 
            source: Rc::new(ImageData {
                id,
                width,
                height
            }),
            region: Rectangle::newi_sized(width, height)
        })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Image, ImageError> {
        Self::load_impl(path)
    }
        
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

pub enum ImageError {
    FormatError(String),
    DimensionError,
    UnsupportedError(String),
    UnsupportedColor,
    NotEnoughData,
    IoError(io::Error),
    ImageEnd,
}

#[cfg(not(target_arch="wasm32"))]
impl From<image::ImageError> for ImageError {
    fn from(img: image::ImageError) -> ImageError {
        match img {
            image::ImageError::FormatError(string) => ImageError::FormatError(string),
            image::ImageError::DimensionError => ImageError::DimensionError,
            image::ImageError::UnsupportedError(string) => ImageError::UnsupportedError(string),
            image::ImageError::UnsupportedColor(_) => ImageError::UnsupportedColor,
            image::ImageError::NotEnoughData => ImageError::NotEnoughData,
            image::ImageError::IoError(err) => ImageError::IoError(err),
            image::ImageError::ImageEnd => ImageError::ImageEnd
        }
    }
}
