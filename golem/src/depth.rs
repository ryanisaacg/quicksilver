//! Various options to specify the depth of view volume
//! and how pixels are determined to occlude each other

/// The state of depth test settings
///
/// Depth values (how far from the camera a pixel is) will be linearly mapped from `-1.0..1.0`
/// (normalized depth coordinates, NDC) to `range_near..range_far` (window depth coordinates).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DepthTestMode {
    /// How to compare each incoming pixel's depth value to one present in the depth buffer
    ///
    /// Default is `DepthTestFunction::Less`.
    pub function: DepthTestFunction,
    /// Specifies the mapping of the near clipping plane to window coordinates
    ///
    /// Default is `0.0f32`.
    pub range_near: f32,
    /// Specifies the mapping of the far clipping plane to window coordinates
    ///
    /// Default is `1.0f32`.
    pub range_far: f32,
    /// Specifies whether the depth buffer is enabled for writing
    ///
    /// Default is `true`, i.e. "writing is enabled".
    ///
    /// Making the depth buffer read-only is useful for situations where you still want
    /// depth tests to occur, but don't want to overwrite the values already in the depth buffer;
    /// for example, common way of rendering scenes with a mix of opaque and translucent objects
    /// is to render opaque ones first, then disable depth mask and render translucent ones
    /// from back to front.
    pub depth_mask: bool,
}

impl Default for DepthTestMode {
    fn default() -> Self {
        Self {
            function: DepthTestFunction::default(),
            range_near: 0.0,
            range_far: 1.0,
            depth_mask: true,
        }
    }
}

/// Function used to compare each incoming pixel depth value with the depth value
/// present in the depth buffer
///
/// "Depth value" is, roughly, how far away the pixel is from the camera; you'll almost always
/// want `DepthTestFunction::Less` (which is the default).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum DepthTestFunction {
    /// Pixel is never drawn
    Never,
    /// Pixel is drawn if it's depth value is less than the stored one
    Less,
    /// Pixel is drawn if it's depth value is equal to the stored one
    Equal,
    /// Pixel is drawn if it's depth value is less than or equal to the stored one
    LessOrEqual,
    /// Pixel is drawn if it's depth value is greater than the stored one
    Greater,
    /// Pixel is drawn if it's depth value is not equal to the stored one
    NotEqual,
    /// Pixel is drawn if it's depth value is greater than or equal to the stored one
    GreaterOrEqual,
    /// Pixel is always drawn
    Always,
}

impl Default for DepthTestFunction {
    fn default() -> Self {
        DepthTestFunction::Less
    }
}

impl DepthTestFunction {
    #[allow(clippy::wrong_self_convention)] // TODO maybe this should be addressed properly.
    pub(crate) fn to_gl(self) -> u32 {
        use DepthTestFunction::*;
        match self {
            Never => glow::NEVER,
            Less => glow::LESS,
            Equal => glow::EQUAL,
            LessOrEqual => glow::LEQUAL,
            Greater => glow::GREATER,
            NotEqual => glow::NOTEQUAL,
            GreaterOrEqual => glow::GEQUAL,
            Always => glow::ALWAYS,
        }
    }
}
