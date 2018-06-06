extern crate futures;
#[cfg(not(target_arch="wasm32"))]
extern crate image;

use error::QuicksilverError;
use futures::{Async, Future, Poll};
use geom::{Rectangle, Vector};
use graphics::{Backend, BackendImpl, ImageData};
use std::{
    error::Error,
    fmt,
    io::Error as IOError,
    os::raw::c_void,
    path::Path,
    rc::Rc
};
#[cfg(not(target_arch="wasm32"))]
use std::path::PathBuf;

///Pixel formats for use with loading raw images
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PixelFormat {
    /// Red, Green, and Blue
    RGB,
    /// Red, Green, Blue, and Alpha
    RGBA,
    /// Blue, Green, and Red
    BGR,
    /// Blue, Green, Red, and Alpha
    BGRA,
}

#[derive(Clone, Debug)]
///An image that can be drawn to the screen
pub struct Image {
    source: Rc<ImageData>,
    region: Rectangle,
}

impl Image {
    pub(crate) fn new(data: ImageData) -> Image {
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
        ImageLoader { 
            path: PathBuf::from(path.as_ref())
        }
    }

    fn from_ptr(data: *const c_void, width: u32, height: u32, format: PixelFormat) -> Image {
        Image::new(BackendImpl::create_texture(data, width, height, format))
    }

    pub(crate) fn new_null(width: u32, height: u32, format: PixelFormat) -> Image {
        use std::ptr::null;
        Image::from_ptr(null(), width, height, format)
    }

    ///Load an image from raw bytes
    pub fn from_raw(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Image {
        Image::from_ptr(data.as_ptr() as *const c_void, width, height, format)
    }
    
    pub(crate) fn get_id(&self) -> u32 {
        self.source.id
    }

    pub(crate) fn source_width(&self) -> u32 {
        self.source.width
    }

    pub(crate) fn source_height(&self) -> u32 {
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
    path: PathBuf,
    #[cfg(target_arch="wasm32")]
    id: u32
}

impl Future for ImageLoader {
    type Item = Image;
    type Error = QuicksilverError;
    
    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let img = image::open(&self.path)?.to_rgba();
        let width = img.width();
        let height = img.height(); 
        Ok(Async::Ready(Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA)))
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use ffi::wasm;
        Ok(match wasm::asset_status(self.id)? {
            true => Async::Ready(Image::new(ImageData {
                    id: unsafe { wasm::get_image_id(self.id) },
                    width: unsafe { wasm::get_image_width(self.id) },
                    height: unsafe { wasm::get_image_height(self.id) }
                })),
            false => Async::NotReady,
        })
    }
}

#[derive(Debug)]
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
    IOError(IOError),
    ///The image has reached its end
    ImageEnd,
}

#[doc(hidden)]
impl From<IOError> for ImageError {
    fn from(err: IOError) -> ImageError {
        ImageError::IOError(err)
    }
}

#[doc(hidden)]
#[cfg(not(target_arch="wasm32"))]
impl From<image::ImageError> for ImageError {
    fn from(img: image::ImageError) -> ImageError {
        match img {
            image::ImageError::FormatError(string) => ImageError::FormatError(string),
            image::ImageError::DimensionError => ImageError::DimensionError,
            image::ImageError::UnsupportedError(string) => ImageError::UnsupportedError(string),
            image::ImageError::UnsupportedColor(_) => ImageError::UnsupportedColor,
            image::ImageError::NotEnoughData => ImageError::NotEnoughData,
            image::ImageError::IoError(err) => ImageError::IOError(err),
            image::ImageError::ImageEnd => ImageError::ImageEnd
        }
    }
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for ImageError {
    fn description(&self) -> &str {
        match self {
            &ImageError::FormatError(ref string) => string,
            &ImageError::DimensionError => "Invalid dimensions",
            &ImageError::UnsupportedError(ref string) => string,
            &ImageError::UnsupportedColor => "Unsupported colorspace",
            &ImageError::NotEnoughData => "Not enough image data",
            &ImageError::IOError(ref err) => err.description(),
            &ImageError::ImageEnd => "Image data ended unexpectedly"
        }
    }
    
    fn cause(&self) -> Option<&Error> {
        match self {
            &ImageError::IOError(ref err) => Some(err),
            _ => None
        }
    }
}
