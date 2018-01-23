#[cfg(not(target_arch="wasm32"))]
extern crate image;

use gl;
use asset::{Loadable, LoadingAsset};
#[cfg(target_arch="wasm32")]
use asset::LoadingHandle;
use geom::{Rectangle, Vector, Transform};
use graphics::{Canvas, View};
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

    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> Result<Image, ImageError> {
        let img = match image::open(path) {
            Ok(img) => img,
            Err(err) => return Err(From::from(err))
        }.to_rgba();
        let width = img.width() as i32;
        let height = img.height() as i32;
        Ok(Image::from_raw(img.into_raw().as_slice(), width, height, PixelFormat::RGBA))
    }
    
    #[cfg(target_arch="wasm32")]
    fn load_impl<P: AsRef<Path>>(path: P) -> u32 {
        use std::ffi::CString;
        unsafe { load_image(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
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

impl Loadable for Image {
    type Error = ImageError;

    #[cfg(target_arch="wasm32")]
    fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Self> {
        LoadingAsset::Loading(LoadingHandle(Image::load_impl(path)))
    }
    
    #[cfg(not(target_arch="wasm32"))]
    fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Self> {
        match Image::load_impl(path) {
            Ok(val) => LoadingAsset::Loaded(val),
            Err(err) => LoadingAsset::Errored(err)
        }
    }

    #[cfg(target_arch="wasm32")]
    fn parse_result(handle: LoadingHandle, loaded: bool, errored: bool) -> LoadingAsset<Self> {
        if loaded {
            if errored {
                LoadingAsset::Errored(ImageError::IoError)
            } else {
                LoadingAsset::Loaded(Image::new(ImageData {
                    id: unsafe { get_image_id(handle.0) },
                    width: unsafe { get_image_width(handle.0) },
                    height: unsafe { get_image_height(handle.0) }
                }))
            }
        } else {
            LoadingAsset::Loading(handle)
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

struct SurfaceData {
    framebuffer: u32
}

impl Drop for SurfaceData {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffer(self.framebuffer) };
    }
}

#[derive(Clone)]
///A possible render target that can be drawn to the screen
pub struct Surface {
    image: Image,
    data: Rc<SurfaceData>,
}

impl Surface {
    ///Create a new surface with a given width and height
    pub fn new(width: i32, height: i32) -> Surface {
        let image = Image::from_raw(&[], width, height, PixelFormat::RGBA);
        let surface = SurfaceData {
            framebuffer: unsafe { gl::GenFramebuffer() }
        };
        unsafe { 
            gl::BindFramebuffer(gl::FRAMEBUFFER, surface.framebuffer);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, image.source.id, 0);
            gl::DrawBuffers(1, &gl::COLOR_ATTACHMENT0 as *const u32);
        }
        Surface {
            image,
            data: Rc::new(surface)
        }
    }

    ///Render data to the surface
    ///
    ///Do not attempt to use the surface or its image within the function, because it is undefined behavior
    pub fn render_to<F>(&self, func: F, canvas: &mut Canvas) where F: FnOnce(&mut Canvas) {
        let viewport = &mut [0, 0, 0, 0];
        let view = canvas.view();
        unsafe {
            gl::GetIntegerv(gl::VIEWPORT, viewport.as_mut_ptr());
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.data.framebuffer);
            gl::Viewport(0, 0, self.image.source_width(), self.image.source_height());
            canvas.set_view(View::new_transformed(self.image.area(), Transform::scale(Vector::newi(1, -1))));
        }
        func(canvas);
        canvas.backend.flush();
        canvas.set_view(view);
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); 
            gl::Viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
        }
    }

    ///Get a reference to the Image that contains the data drawn to the Surface
    pub fn image(&self) -> &Image {
        &self.image
    }
}
