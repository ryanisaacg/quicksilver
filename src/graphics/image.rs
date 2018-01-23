#[cfg(not(target_arch="wasm32"))]
extern crate image;

use gl;
use asset::{Asset, LoadingAsset};
use geom::{Rectangle, Vector};
#[cfg(target_arch="wasm32")]
use std::os::raw::c_char;
use std::ops::Drop;
use std::path::Path;
use std::rc::Rc;

#[cfg(target_arch="wasm32")]
extern "C" {
    fn load_image(string: *mut c_char) -> u32;
    fn get_image_id(index: u32) -> u32;
    fn get_image_width(index: u32) -> i32;
    fn get_image_height(index: u32) -> i32;
}

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
        let region = Rectangle::newi_sized(data.width, data.height);
        Image {
            source: Rc::new(data),
            region
        }
    }
   
    /// Start loading a texture from a given path
    pub fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Image> {
        Image::load_impl(path)
    }
    
    #[cfg(target_arch="wasm32")]
    fn load_impl<P: AsRef<Path>>(path: P) -> LoadingAsset<Self> {
        use std::ffi::CString;
        LoadingAsset::Loading(unsafe { load_image(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) })
    }
    
    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> LoadingAsset<Self> {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(err) => return LoadingAsset::Errored(err.into())
        }.to_rgba();
        let width = img.width() as i32;
        let height = img.height() as i32;
        LoadingAsset::Loaded(Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA))
    }


    ///Load an image from raw bytes
    #[cfg(not(target_arch="wasm32"))]
    pub fn from_raw(data: &[u8], width: i32, height: i32, format: PixelFormat) -> Image {
        use std::os::raw::c_void;
        let id = unsafe {
            let texture = gl::GenTexture();
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, format as u32, 
                           gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            texture
        };
        Image::new(ImageData { id, width, height })
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

impl Asset for Image {
    type Loading = u32;
    type Error = ImageError;

    #[cfg(not(target_arch="wasm32"))]
    fn update(_: u32) -> LoadingAsset<Self> {
        unreachable!();
    }

    #[cfg(target_arch="wasm32")]
    fn update(loading: u32) -> LoadingAsset<Self> {
        extern "C" {
            fn is_texture_loaded(handle: u32) -> bool;
            fn is_texture_errored(handle: u32) -> bool;
        }
        if unsafe { is_texture_loaded(loading) } {
            if unsafe { is_texture_errored(loading) } {
                LoadingAsset::Errored(ImageError::IoError)
            } else {
                LoadingAsset::Loaded(Image::new(ImageData {
                    id: unsafe { get_image_id(loading) },
                    width: unsafe { get_image_width(loading) },
                    height: unsafe { get_image_height(loading) }
                }))
            } 
        } else {
            LoadingAsset::Loading(loading)
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
