use crate::QuicksilverError;
use crate::geom::Vector;
use super::{ColorFormat, Graphics, Image};

/// A Surface is the core struct for rendering to textures, or getting data from them.
///
/// If you want to render to a texture, [`attach`] it and then pass the surface to
/// [`Graphics::flush`].
///
/// If you want to get data from a texture, [`attach`] it and use [`Surface::screenshot`].
///
/// [`attach`]: Surface::attach
pub struct Surface(pub(crate) golem::Surface);

impl Surface {
    /// Create a Surface with an attached Image
    ///
    /// The image must not have any other references to it, or this function will return an error.
    pub fn new(gfx: &Graphics, attachment: Image) -> Result<Surface, QuicksilverError> {
        let tex = attachment
            .into_raw()
            .map_err(|_| QuicksilverError::SurfaceImageError)?;
        Ok(Surface(golem::Surface::new(&gfx.ctx, tex)?))
    }

    /// Use the attached image as the backing data for this Surface
    ///
    /// To either get the data for an image via [`Surface::screenshot`] or set it via
    /// [`Graphics::flush`], an image needs to be attached to this Surface.
    ///
    /// The image must not have any other references to it, or this function will return an error.
    ///
    /// It's generally faster to create one [`Surface`] per [`Image`], and only attach and
    /// [`detach`] when necessary.
    ///
    /// [`detach`]: Surface::detach
    pub fn attach(&mut self, attachment: Image) -> Result<(), QuicksilverError> {
        let tex = attachment
            .into_raw()
            .map_err(|_| QuicksilverError::SurfaceImageError)?;
        self.0.put_texture(tex);

        Ok(())
    }

    /// Take the Image out of this Surface
    ///
    /// To use the data that has been rendered to a Surface, its attachment has to be removed to
    /// avoid creating a loop (where the Image is both being drawn *from* and being drawn *to*.)
    pub fn detach(&mut self) -> Option<Image> {
        Some(Image::new(self.0.take_texture()?))
    }

    /// Get the pixel data of a given region of this surface
    pub fn screenshot(
        &self,
        gfx: &Graphics,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: ColorFormat,
    ) -> Vec<u8> {
        self.0.bind();
        let mut buffer = vec![0; (width * height * format.bytes_per_pixel()) as usize];
        self.0
            .get_pixel_data(x, y, width, height, format, &mut buffer[..]);
        golem::Surface::unbind(&gfx.ctx);

        buffer
    }

    /// Return the size of the attached image, or None if there is no image
    pub fn size(&self) -> Option<Vector> {
        Some(Vector {
            x: self.0.width()? as f32,
            y: self.0.height()? as f32,
        })
    }
}
