use super::*;
use core::num::NonZeroU32;

/// An image stored on the GPU
pub struct Texture {
    ctx: Context,
    pub(crate) id: GlTexture,
    width: u32,
    height: u32,
    mipmap: bool,
}

impl Texture {
    /// Create a new, empty texture
    pub fn new(ctx: &Context) -> Result<Texture, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_texture()? };
        let tex = Texture {
            ctx,
            id,
            width: 0,
            height: 0,
            mipmap: false,
        };
        tex.set_minification(TextureFilter::Linear)
            .expect("Linear textures don't require mip-maps");

        Ok(tex)
    }

    /// Mark the texture as active, allowing it to be used in shaders
    ///
    /// To use the texture in a shader, supply the same number as the `bind_point` to a
    /// [`UniformValue::Int`], matching a [`Uniform`] with a [`UniformType::Sampler2D`].
    ///
    /// The value 0 is reserved by `golem`, so it cannot be passed to this function.
    pub fn set_active(&self, bind_point: NonZeroU32) {
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.active_texture(glow::TEXTURE0 + bind_point.get());
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.active_texture(glow::TEXTURE0);
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Set the image data associated with this texture
    ///
    /// `width` and `height` must be less than the maximum texture size of the
    /// GPU, given by [`glow::MAX_TEXTURE_SIZE`]. If they are both powers of 2, mipmaps will be
    /// generated. If they aren't, mipmaps will be unavailable.
    ///
    /// If 'data' is None, the image will be created with no data at the given dimensions.
    /// If it is Some, it needs to be at least as long as `width * height *
    /// [`color.bytes_per_pixel`])
    ///
    /// [`color.bytes_per_pixel`]: ColorFormat::bytes_per_pixel
    pub fn set_image(&mut self, data: Option<&[u8]>, width: u32, height: u32, color: ColorFormat) {
        assert!(width > 0, "The texture width was 0",);
        assert!(height > 0, "The texture width was 0",);
        assert!(
            width < glow::MAX_TEXTURE_SIZE,
            "The texture width was bigger than the maximum size"
        );
        assert!(
            height < glow::MAX_TEXTURE_SIZE,
            "The texture height was bigger than the maximum size"
        );
        if let Some(data) = data {
            assert!(
                data.len() >= (width * height * color.bytes_per_pixel()) as usize,
                "The texture data wasn't big enough for the width, height, and format supplied"
            );
        }
        self.width = width;
        self.height = height;

        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA,
        };
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                format as i32,
                width as i32,
                height as i32,
                0,
                format,
                glow::UNSIGNED_BYTE,
                data,
            );
            if width & (width - 1) == 0 && height & (height - 1) == 0 {
                gl.generate_mipmap(glow::TEXTURE_2D);
                self.mipmap = true;
            } else {
                self.mipmap = false;
                self.set_wrap_h(TextureWrap::ClampToEdge)
                    .expect("The texture wrap ClampToEdge is always valid");
                self.set_wrap_v(TextureWrap::ClampToEdge)
                    .expect("The texture wrap ClampToEdge is always valid");
            }
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    /// Set a region of the texture data
    ///
    /// The data provided must be enough to cover `width * height * [`color.bytes_per_pixel()`]`.
    /// Also, the region must be within the texture's bounds.
    ///
    /// [`color.bytes_per_pixel()`]: ColorFormat::bytes_per_pixel
    pub fn set_subimage(
        &self,
        data: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: ColorFormat,
    ) {
        assert!(
            x + width <= self.width,
            "The region over-ran the width of the texture"
        );
        assert!(
            y + height <= self.height,
            "The region over-ran the height of the texture"
        );
        let format = match color {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA,
        };
        let required_data_len = width * height * color.bytes_per_pixel();
        assert!(data.len() >= required_data_len as usize);
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_sub_image_2d_u8_slice(
                glow::TEXTURE_2D,
                0,
                x as i32,
                y as i32,
                width as i32,
                height as i32,
                format,
                glow::UNSIGNED_BYTE,
                Some(data),
            );
            gl.generate_mipmap(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    fn set_texture_param(&self, param: u32, value: i32) {
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, Some(self.id));
            gl.tex_parameter_i32(glow::TEXTURE_2D, param, value);
        }
    }

    /// Determine how the texture should scale down
    pub fn set_minification(&self, min: TextureFilter) -> Result<(), GolemError> {
        if !self.mipmap && min.uses_mipmap() {
            Err(GolemError::MipMapsUnavailable)
        } else {
            self.set_texture_param(glow::TEXTURE_MIN_FILTER, min.to_gl());
            Ok(())
        }
    }

    /// Determine how the texture should scale up
    pub fn set_magnification(&self, max: TextureFilter) -> Result<(), GolemError> {
        if max.uses_mipmap() {
            Err(GolemError::MipMapsUnavailable)
        } else {
            self.set_texture_param(glow::TEXTURE_MAG_FILTER, max.to_gl());
            Ok(())
        }
    }

    /// Determine how the texture is wrapped horizontally
    pub fn set_wrap_h(&self, wrap: TextureWrap) -> Result<(), GolemError> {
        if !self.mipmap && wrap != TextureWrap::ClampToEdge {
            Err(GolemError::IllegalWrapOption)
        } else {
            self.set_texture_param(glow::TEXTURE_WRAP_S, wrap.to_gl());
            Ok(())
        }
    }

    /// Determine how the texture is wrapped vertically
    pub fn set_wrap_v(&self, wrap: TextureWrap) -> Result<(), GolemError> {
        if !self.mipmap && wrap != TextureWrap::ClampToEdge {
            Err(GolemError::IllegalWrapOption)
        } else {
            self.set_texture_param(glow::TEXTURE_WRAP_T, wrap.to_gl());
            Ok(())
        }
    }
}

/// How textures should scale when being drawn at non-native sizes
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TextureFilter {
    /// Smooth out the texture samples as the texture stretches or squashes
    ///
    /// This is best for textures you want to blur as they scale
    Linear,
    /// Pick the nearest texture sample as the texture stretches or squashes
    ///
    /// This is best for textures you want to pixelate as they scale
    Nearest,
    /// Use the mipmap, and take the nearest sample from the nearest mipmap
    NearestMipmapNearest,
    /// Use the mipmap, and take an averaged sample from the nearest mipmap
    LinearMipmapNearest,
    /// Use the mipmap, and take the nearest sample from averaged layers of the mipmap
    NearestMipmapLinear,
    /// Use the mipmap, and take an averaged sample from averaged layers of the mipmap
    LinearMipmapLinear,
}

impl TextureFilter {
    pub(crate) fn to_gl(self) -> i32 {
        match self {
            TextureFilter::Linear => glow::LINEAR as i32,
            TextureFilter::Nearest => glow::NEAREST as i32,
            TextureFilter::NearestMipmapNearest => glow::NEAREST_MIPMAP_NEAREST as i32,
            TextureFilter::NearestMipmapLinear => glow::NEAREST_MIPMAP_LINEAR as i32,
            TextureFilter::LinearMipmapNearest => glow::LINEAR_MIPMAP_NEAREST as i32,
            TextureFilter::LinearMipmapLinear => glow::LINEAR_MIPMAP_LINEAR as i32,
        }
    }

    /// If this texture filter uses texture mipmaps
    ///
    /// Mipmaps are only available for power-of-two textures, and only available for minification
    pub fn uses_mipmap(self) -> bool {
        !matches!(self, TextureFilter::Linear | TextureFilter::Nearest)
    }
}

/// How the texture should wrap if a sample is outside the edge
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TextureWrap {
    /// Repeat as though the texture was endlessly tiled
    Repeat,
    /// Choose the closest sample in the texture
    ClampToEdge,
    /// Repeat as though the texture was endlessly tiled, but flipping each time
    MirroredRepeat,
}

impl TextureWrap {
    pub(crate) fn to_gl(self) -> i32 {
        match self {
            TextureWrap::Repeat => glow::REPEAT as i32,
            TextureWrap::ClampToEdge => glow::CLAMP_TO_EDGE as i32,
            TextureWrap::MirroredRepeat => glow::MIRRORED_REPEAT as i32,
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_texture(self.id);
        }
    }
}
