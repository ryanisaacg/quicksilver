use super::{ColorFormat, Graphics, PixelFormat};
use crate::geom::Vector;
use crate::QuicksilverError;

use std::cell::{Ref, RefCell};
use std::path::Path;
use std::rc::Rc;

use golem::*;

/// A 2D image, stored on the GPU
///
/// See [`Graphics::draw_image`] to draw it
#[derive(Clone)]
pub struct Image(pub(crate) Rc<RefCell<Texture>>);

impl Image {
    pub(crate) fn new(texture: Texture) -> Image {
        Image(Rc::new(RefCell::new(texture)))
    }

    /// Create an image with a given width and height
    ///
    /// Either source the data from an array of bytes, or create a blank image.
    /// `format` determines how to interpet the bytes when creating the image
    pub fn from_raw(
        gfx: &Graphics,
        data: Option<&[u8]>,
        width: u32,
        height: u32,
        format: PixelFormat,
    ) -> Result<Image, GolemError> {
        let mut texture = Texture::new(&gfx.ctx)?;
        texture.set_image(data, width, height, format);

        Ok(Image::new(texture))
    }

    /// Create an image from an encoded image format
    ///
    /// JPEG and PNG are supported
    pub fn from_encoded_bytes(gfx: &Graphics, raw: &[u8]) -> Result<Image, QuicksilverError> {
        let img = image::load_from_memory(raw)?.to_rgba();
        let width = img.width();
        let height = img.height();
        Ok(Image::from_raw(
            gfx,
            Some(img.into_raw().as_slice()),
            width,
            height,
            PixelFormat::RGBA,
        )?)
    }

    /// Load an image from a file at the given path
    ///
    /// JPEG and PNG file formats are supported
    pub async fn load(gfx: &Graphics, path: impl AsRef<Path>) -> Result<Image, QuicksilverError> {
        let file_contents = platter::load_file(path).await?;
        Image::from_encoded_bytes(gfx, file_contents.as_slice())
    }

    /// Replace the backing data for the image, or create a blank image
    pub fn set_data(&mut self, data: Option<&[u8]>, width: u32, height: u32, color: ColorFormat) {
        self.0.borrow_mut().set_image(data, width, height, color);
    }

    /// Set the data for some region of this image, without clearing it
    pub fn set_sub_data(
        &self,
        data: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: ColorFormat,
    ) {
        self.raw().set_subimage(data, x, y, width, height, color);
    }

    pub(crate) fn raw(&self) -> Ref<Texture> {
        self.0.borrow()
    }

    pub(crate) fn ptr_eq(&self, other: &Image) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// Get the size of the image
    pub fn size(&self) -> Vector {
        Vector {
            x: self.raw().width() as f32,
            y: self.raw().height() as f32,
        }
    }

    /// Determine how the texture should scale down
    ///
    /// Only textures with a power-of-2 size support mipmaps, all others will return errors
    pub fn set_minification(&self, min: TextureFilter) -> Result<(), QuicksilverError> {
        Ok(self.raw().set_minification(min)?)
    }

    /// Determine how the texture should scale up
    ///
    /// Attempting to use a mipmap filter for magnification will result in an error
    pub fn set_magnification(&self, max: TextureFilter) -> Result<(), QuicksilverError> {
        Ok(self.raw().set_magnification(max)?)
    }

    /// Determine how the texture is wrapped horizontally
    ///
    /// Only textures with a power-of-2 size support texture wrapping, all others must ClampToEdge
    /// or will return an error
    pub fn set_wrap_h(&self, wrap: TextureWrap) -> Result<(), QuicksilverError> {
        Ok(self.raw().set_wrap_h(wrap)?)
    }

    /// Determine how the texture is wrapped vertically
    ///
    /// Only textures with a power-of-2 size support texture wrapping, all others must ClampToEdge
    /// or will return an error
    pub fn set_wrap_v(&self, wrap: TextureWrap) -> Result<(), QuicksilverError> {
        Ok(self.raw().set_wrap_v(wrap)?)
    }

    pub(crate) fn into_raw(self) -> Result<Texture, Rc<RefCell<Texture>>> {
        Ok(Rc::try_unwrap(self.0)?.into_inner())
    }
}
