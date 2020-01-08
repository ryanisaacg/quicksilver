use super::{Graphics, PixelFormat};
use crate::geom::Vector;
use crate::QuicksilverError;

use std::path::Path;
use std::rc::Rc;

/// A 2D image, stored on the GPU
///
/// See [`Graphics::draw_image`] to draw it
#[derive(Clone)]
pub struct Image {
    tex: Rc<golem::Texture>,
}

impl Image {
    fn new(texture: golem::Texture) -> Image {
        Image {
            tex: Rc::new(texture),
        }
    }

    /// Create an image from an array of bytes
    ///
    /// `format` determines how to interpet the bytes when creating the image
    pub fn from_raw(
        gfx: &Graphics,
        data: &[u8],
        width: u32,
        height: u32,
        format: PixelFormat,
    ) -> Result<Image, QuicksilverError> {
        Ok(Image::new(gfx.create_image(data, width, height, format)?))
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
            img.into_raw().as_slice(),
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

    pub(crate) fn raw(&self) -> &golem::Texture {
        &self.tex
    }

    pub(crate) fn ptr_eq(&self, other: &Image) -> bool {
        Rc::ptr_eq(&self.tex, &other.tex)
    }

    /// Get the size of the image
    pub fn size(&self) -> Vector {
        Vector {
            x: self.raw().width() as f32,
            y: self.raw().height() as f32,
        }
    }
}
