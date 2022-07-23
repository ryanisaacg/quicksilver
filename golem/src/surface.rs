use crate::*;

/// A framebuffer that allows render-to-texture
pub struct Surface {
    pub(crate) ctx: Context,
    pub(crate) id: GlFramebuffer,
    pub(crate) texture: Option<Texture>,
}

impl Surface {
    /// Create a new Surface to render to, backed by the given texture
    pub fn new(ctx: &Context, texture: Texture) -> Result<Surface, GolemError> {
        let ctx = Context(ctx.0.clone());
        let gl = &ctx.0.gl;
        let id = unsafe { gl.create_framebuffer() }?;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(id));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture.id),
                0,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, *ctx.0.current_surface.borrow());
        }

        Ok(Surface {
            ctx,
            id,
            texture: Some(texture),
        })
    }

    /// Check if a texture is attached to this Surface
    ///
    /// Textures can be attached via [`Surface::put_texture`] and removed via
    /// [`Surface::take_texture`].
    pub fn has_texture(&self) -> bool {
        self.texture.is_some()
    }

    /// Check if this surface is bound to be operated on
    ///
    /// Call [`Surface::bind`] to bind the surface, which is required to render to it or to call
    /// [`Surface::get_pixel_data`]
    pub fn is_bound(&self) -> bool {
        match *self.ctx.0.current_surface.borrow() {
            Some(surface) => self.id == surface,
            None => false,
        }
    }

    /// Remove the texture from the Surface to operate on it
    ///
    /// Until another texture is added via [`Surface::put_texture`], operations on the Surface will
    /// panic.
    pub fn take_texture(&mut self) -> Option<Texture> {
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                None,
                0,
            );
        }
        self.texture.take()
    }

    /// Put a texture into the Surface to operate on
    pub fn put_texture(&mut self, texture: Texture) {
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture.id),
                0,
            );
        }
        self.texture = Some(texture);
    }

    /// Borrow the texture the Surface is holding
    ///
    /// # Safety
    ///
    /// The texture can be used and referenced while it is bound and while it is not bound.
    /// However, it is undefined behavior to form a 'texture loop.' If a Surface is actively bound,
    /// the texture cannot be used in the rendering pipeline. It is important to only ever render
    /// to the Surface *or* use its texture, not both.
    pub unsafe fn borrow_texture(&self) -> Option<&Texture> {
        self.texture.as_ref()
    }

    /// Set the current render target to this surface
    ///
    /// Also necessary for operations like [`Surface::get_pixel_data`]
    pub fn bind(&self) {
        assert!(
            self.has_texture(),
            "The surface had no attached image when bind was called"
        );
        *self.ctx.0.current_surface.borrow_mut() = Some(self.id);
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
        }
    }

    /// Unbind the surface and set the render target to the screen
    pub fn unbind(ctx: &Context) {
        *ctx.0.current_surface.borrow_mut() = None;
        let gl = &ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    /// Get the pixel data and write it to a buffer
    ///
    /// The surface must be bound first, see [`Surface::bind`].
    ///
    /// The ColorFormat determines how many bytes each pixel is: 3 bytes for RGB and 4 for RGBA. The
    /// slice needs have a length of `(width - x) * (height - y) * ColorFormat size`.
    pub fn get_pixel_data(
        &self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: ColorFormat,
        data: &mut [u8],
    ) {
        assert!(
            self.is_bound(),
            "The surface wasn't bound when get_pixel_data was called"
        );
        assert!(
            self.has_texture(),
            "The surface had no attached image when get_pixel_data was called"
        );
        let bytes_per_pixel = format.bytes_per_pixel();
        let length = (width * height * bytes_per_pixel) as usize;
        assert!(
            data.len() >= length,
            "The buffer was not large enough to hold the data"
        );
        let format = format.gl_format();
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.id));
            gl.read_pixels(
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                glow::UNSIGNED_BYTE,
                data,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }

    /// Get the width of the inner texture, or None if there is no texture
    pub fn width(&self) -> Option<u32> {
        self.texture.as_ref().map(|tex| tex.width())
    }

    /// Get the height of the inner texture, or None if there is no texture
    pub fn height(&self) -> Option<u32> {
        self.texture.as_ref().map(|tex| tex.height())
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_framebuffer(self.id);
        }
    }
}
