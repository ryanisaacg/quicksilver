use {
    Result,
    backend::{Backend, BackendImpl, ImageData},
    error::QuicksilverError,
    file::load_file,
    futures::{Future, future},
    geom::{Rectangle, Transform, Vector},
    image,
    std::{
        error::Error,
        fmt,
        io::Error as IOError,
        path::Path,
        rc::Rc
    }
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

impl Image {
    pub(crate) fn new(data: ImageData) -> Image {
        let region = Rectangle::new_sized((data.width, data.height));
        Image {
            source: Rc::new(data),
            region
        }
    }
   
    /// Start loading a texture from a given path
    pub fn load<P: AsRef<Path>>(path: P) -> impl Future<Item = Image, Error = QuicksilverError> {
        load_file(path)
            .map(|data| {
                let img = image::load_from_memory(data.as_slice())?.to_rgba();
                let width = img.width();
                let height = img.height(); 
                Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA)
            })
            .and_then(future::result)
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

    ///The area of the source image this subimage takes up
    pub fn area(&self) -> Rectangle {
        self.region
    }

    ///Find a subimage of a larger image
    pub fn subimage(&self, rect: Rectangle) -> Image {
        Image {
            source: self.source.clone(),
            region: Rectangle::new(
                (
                    self.region.pos.x + rect.pos.x,
                    self.region.pos.y + rect.pos.y
                ),
                (
                    rect.width(),
                    rect.height()
                )
            )
        }
    }
    
    /// Create a projection matrix for a given region onto the Image
    pub fn projection(&self, region: Rectangle) -> Transform {
        let source_size: Vector = (self.source_width(), self.source_height()).into();
        let recip_size = source_size.recip();
        let normalized_pos = self.region.top_left().times(recip_size);
        let normalized_size = self.region.size().times(recip_size);
        Transform::translate(normalized_pos)
            * Transform::scale(normalized_size)
            * Transform::scale(region.size().recip())
            * Transform::translate(-region.top_left())
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
    
    fn cause(&self) -> Option<&dyn Error> {
        match self {
            &ImageError::DecodingError(ref err) => Some(err),
            &ImageError::IOError(ref err) => Some(err),
        }
    }
}
