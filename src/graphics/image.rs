extern crate futures;
#[cfg(not(target_arch="wasm32"))]
extern crate image;

use ffi::gl;
use futures::{Async, Future, Poll};
use geom::{Rectangle, Vector};
use std::ops::Drop;
use std::os::raw::c_void;
use std::path::Path;
use std::rc::Rc;

///Pixel formats for use with loading raw images
pub enum PixelFormat {
    /// Red, Green, and Blue
    RGB = gl::RGB as isize,
    /// Red, Green, Blue, and Alpha
    RGBA = gl::RGBA as isize,
    /// Blue, Green, and Red
    BGR = gl::BGR as isize,
    /// Blue, Green, Red, and Alpha
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
///An image that can be drawn to the screen
pub struct Image {
    source: Rc<ImageData>,
    region: Rectangle,
}

impl Image {
    fn new(data: ImageData) -> Image {
        let region = Rectangle::new_sized(data.width, data.height);
        Image {
            source: Rc::new(data),
            region
        }
    }
   
    /// Start loading a texture from a given path
    pub fn load<P: AsRef<Path>>(path: P) -> ImageLoader {
        Image::load_impl(path)
    }
    
    #[cfg(target_arch="wasm32")]
    fn load_impl<P: AsRef<Path>>(path: P) -> ImageLoader {
        use std::ffi::CString;
        use ffi::wasm;
        ImageLoader {
            id: unsafe { wasm::load_image(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
        }
    }
    
    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> ImageLoader {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(err) => return ImageLoader { image: Err(err.into()) }
        }.to_rgba();
        let width = img.width() as i32;
        let height = img.height() as i32;
        ImageLoader {
            image: Ok(Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA))
        }
    }

    fn from_ptr(data: *const c_void, width: i32, height: i32, format: PixelFormat) -> Image {
        unsafe {
            let id = gl::GenTexture();
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, format as u32, 
                           gl::UNSIGNED_BYTE, data);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            Image::new(ImageData { id, width, height })
        }
    }

    pub(crate) fn new_null(width: i32, height: i32, format: PixelFormat) -> Image {
        use std::ptr::null;
        Image::from_ptr(null(), width, height, format)
    }

    ///Load an image from raw bytes
    pub fn from_raw(data: &[u8], width: i32, height: i32, format: PixelFormat) -> Image {
        Image::from_ptr(data.as_ptr() as *const c_void, width, height, format)
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
        Vector::new(self.source_width(), self.source_height())
    }

    ///The area of the source image this subimage takes up
    pub fn area(&self) -> Rectangle {
        self.region
    }

    ///Find a subimage of a larger image
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

/// A future for loading images
pub struct ImageLoader { 
    #[cfg(not(target_arch="wasm32"))]
    image: Result<Image, ImageError>,
    #[cfg(target_arch="wasm32")]
    id: u32
}

impl Future for ImageLoader {
    type Item = Image;
    type Error = ImageError;
    
    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<Image, ImageError> {
        Ok(Async::Ready(self.image.clone()?))
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<Image, ImageError> {
        use ffi::wasm;
        if unsafe { wasm::is_texture_loaded(self.id) } {
            if unsafe { wasm::is_texture_errored(self.id) } {
                Err(ImageError::IoError)
            } else {
                Ok(Async::Ready(Image::new(ImageData {
                    id: unsafe { wasm::get_image_id(self.id) },
                    width: unsafe { wasm::get_image_width(self.id) },
                    height: unsafe { wasm::get_image_height(self.id) }
                })))
            } 
        } else {
            Ok(Async::NotReady)
        }

    }
}

#[derive(Clone, Debug)]
///An error generated while loading an image
pub enum ImageError {
    ///There was an error in the image format
    FormatError(String),
    ///The image dimensions were invalid
    DimensionError,
    ///The image format is unsupported
    UnsupportedError(String),
    ///The color type is not supported
    UnsupportedColor,
    ///The image data ends too early
    NotEnoughData,
    ///There was some error reading the image file
    IoError,
    ///The image has reached its end
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
            image::ImageError::IoError(_) => ImageError::IoError,
            image::ImageError::ImageEnd => ImageError::ImageEnd
        }
    }
}
