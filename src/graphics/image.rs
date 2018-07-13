extern crate futures;
extern crate image;

use Result;
use error::QuicksilverError;
use file::FileLoader;
use futures::{Async, Future, Poll};
use geom::{Rectangle, Transform, Vector};
use graphics::{Backend, BackendImpl, DrawAttributes, Drawable, GpuTriangle, ImageData, Vertex, Window};
use std::{
    error::Error,
    fmt,
    io::Error as IOError,
    path::Path,
    rc::Rc
};

///Pixel formats for use with loading raw images
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum PixelFormat {
    /// Red, Green, and Blue
    RGB,
    /// Red, Green, Blue, and Alpha
    RGBA
}

#[derive(Clone, Debug)]
///An image that can be drawn to the screen
pub struct Image {
    source: Rc<ImageData>,
    region: Rectangle,
}

/// A future for loading images
pub struct ImageLoader(FileLoader);

impl Future for ImageLoader {
    type Item = Image;
    type Error = QuicksilverError;
    
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(match self.0.poll()? {
            Async::Ready(data) => Async::Ready({
                let img = image::load_from_memory(data.as_slice())?.to_rgba();
                let width = img.width();
                let height = img.height(); 
                Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA)?
            }),
            Async::NotReady => Async::NotReady
        })
    }
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
        ImageLoader(FileLoader::load(path))
    }

    pub(crate) fn new_null(width: u32, height: u32, format: PixelFormat) -> Result<Image> {
        Image::from_raw(&[], width, height, format)
    }

    ///Load an image from raw bytes
    pub fn from_raw(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<Image> {
        Ok(unsafe {
            Image::new(BackendImpl::create_texture(data, width, height, format)?)
        })
    }

    #[cfg(target_arch="wasm32")]
    pub(crate) fn data(&self) -> &ImageData {
        &self.source
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

#[derive(Debug)]
///An error generated while loading an image
pub enum ImageError {
    /// There was an error decoding the bytes of the image
    DecodingError(image::ImageError),
    ///There was some error reading the image file
    IOError(IOError)
}

#[doc(hidden)]
impl From<IOError> for ImageError {
    fn from(err: IOError) -> ImageError {
        ImageError::IOError(err)
    }
}

#[doc(hidden)]
impl From<image::ImageError> for ImageError {
    fn from(img: image::ImageError) -> ImageError {
        ImageError::DecodingError(img)
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
            &ImageError::DecodingError(ref err) => err.description(),
            &ImageError::IOError(ref err) => err.description(),
        }
    }
    
    fn cause(&self) -> Option<&Error> {
        match self {
            &ImageError::DecodingError(ref err) => Some(err),
            &ImageError::IOError(ref err) => Some(err),
        }
    }
}

impl Drawable for Image {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        let area = self.area();
        let trans = Transform::translate(area.size() / 2)
            * params.transform
            * Transform::translate(-area.size() / 2)
            * Transform::scale(area.size());
        let recip_size = self.source_size().recip();
        let normalized_pos = area.top_left().times(recip_size);
        let normalized_size = area.size().times(recip_size);
        let vertices = &[
            Vertex::new_textured(trans * Vector::zero(), normalized_pos + Vector::zero().times(normalized_size), params.color),
            Vertex::new_textured(trans * Vector::x(), normalized_pos +  Vector::x().times(normalized_size), params.color),
            Vertex::new_textured(trans * Vector::one(), normalized_pos +  Vector::one().times(normalized_size), params.color),
            Vertex::new_textured(trans * Vector::y(), normalized_pos + Vector::y().times(normalized_size), params.color),
        ];
        let triangles = &[
            GpuTriangle::new_textured([0, 1, 2], params.z, self.clone()),
            GpuTriangle::new_textured([2, 3, 0], params.z, self.clone())
        ];
        window.add_vertices(vertices.iter().cloned(), triangles.iter().cloned());
    }
}