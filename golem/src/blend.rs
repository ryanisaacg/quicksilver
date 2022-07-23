//! Various options to specify how to combine pixels with what they draw over
//!
//! GPU blending follows the equation:
//!
//! `Output = Operation(SourceFunction(Source), DestFunction(Destination))`
//!
//! The `Operation` is controlled by [`BlendEquation`], and the
//! `SourceFunction` and `DestFunction` are controlled by [`BlendFunction`]. Both of these are
//! combined to form a [`BlendMode`], which can be applied by [`Context::set_blend_mode`].
//!
//! The default [`BlendMode`] produces the result of:
//!
//! `Output = Source.Alpha * Source + (1 - Source.Alpha) * Destination`
//!
//! and looks like:
//!
//! ```no_run
//! # use golem::blend::{
//! #   BlendMode, BlendInput, BlendFactor, BlendChannel, BlendOperation, BlendEquation,
//! #   BlendFunction,
//! # };
//! # fn test() -> BlendMode {
//! BlendMode {
//!     equation: BlendEquation::Same(BlendOperation::Add),
//!     function: BlendFunction::Same {
//!         source: BlendFactor::Color {
//!             input: BlendInput::Source,
//!             channel: BlendChannel::Alpha,
//!             is_inverse: false,
//!         },
//!         destination: BlendFactor::Color {
//!             input: BlendInput::Source,
//!             channel: BlendChannel::Alpha,
//!             is_inverse: true,
//!         },
//!     },
//!     global_color: [0.0; 4]
//! }
//! # }
//! ```
//!
//! For more information, see the documentation for the individual Blend enums.
//!
//! [`Context::set_blend_mode`]: crate::Context::set_blend_mode

/// The state of the blend pipeline
///
/// See [`Context::set_blend_mode`]
///
/// [`Context::set_blend_mode`]: crate::Context::set_blend_mode
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BlendMode {
    /// How to combine the source and destination
    pub equation: BlendEquation,
    /// How to transform the inputs to the `equation`
    pub function: BlendFunction,
    /// A color in blending that's neither the source nor destination, specified as [R, G, B, A]
    ///
    /// This provides the value for [`BlendInput::GlobalBlend`]
    pub global_color: [f32; 4],
}

impl Default for BlendMode {
    fn default() -> BlendMode {
        BlendMode {
            equation: BlendEquation::default(),
            function: BlendFunction::default(),
            global_color: [0.0; 4],
        }
    }
}

/// How to combine the values when blending
///
/// Almost all the time you'll want `BlendEquation::Same(BlendOperation::Add)`, but there are cases
/// where other blend equatiosn come in handy.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlendEquation {
    /// Apply the same equation to the color and alpha of the blended pixels
    Same(BlendOperation),
    /// Apply a different equation to the color and alpha channel of the blended pixels
    Separate {
        color: BlendOperation,
        alpha: BlendOperation,
    },
}

impl Default for BlendEquation {
    fn default() -> BlendEquation {
        BlendEquation::Same(BlendOperation::Add)
    }
}

/// The operation to apply to the pixels during blending
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlendOperation {
    /// Output = Source + Destination
    Add,
    /// Output = Source - Destination
    Subtract,
    /// Output = Destination - Source
    ReverseSubtract,
    /// Output = Max(Source, Destination)
    Max,
    /// Output = Min(Source, Destination)
    Min,
}

impl BlendOperation {
    pub(crate) fn to_gl(self) -> u32 {
        use BlendOperation::*;
        match self {
            Add => glow::FUNC_ADD,
            Subtract => glow::FUNC_SUBTRACT,
            ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            Max => glow::MAX,
            Min => glow::MIN,
        }
    }
}

/// The blend function controls how the source and destination are transformed
///
/// Before being passed to the [`BlendEquation`], the source and destination are multiplied by the
/// value determined by `BlendFunction`.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlendFunction {
    /// Use the same [`BlendFactor`] on the color and alpha channels
    Same {
        source: BlendFactor,
        destination: BlendFactor,
    },
    /// Use different [`BlendFactor`]s on the color and alpha channels
    Separate {
        source_color: BlendFactor,
        source_alpha: BlendFactor,
        destination_color: BlendFactor,
        destination_alpha: BlendFactor,
    },
}

impl Default for BlendFunction {
    fn default() -> BlendFunction {
        BlendFunction::Same {
            source: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: false,
            },
            destination: BlendFactor::Color {
                input: BlendInput::Source,
                channel: BlendChannel::Alpha,
                is_inverse: true,
            },
        }
    }
}

/// The various coefficients to multiply the color inputs by
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlendFactor {
    Zero,
    One,
    Color {
        /// Which source of color
        input: BlendInput,
        /// Which channel of the `input` to use
        channel: BlendChannel,
        /// Whether to use (1 - Value) instead of Value
        is_inverse: bool,
    },
}

impl BlendFactor {
    pub(crate) fn to_gl(self) -> u32 {
        use BlendChannel::*;
        use BlendFactor::{Color as Col, One, Zero};
        use BlendInput::*;

        match self {
            Zero => glow::ZERO,
            One => glow::ONE,
            Col {
                input: Source,
                channel: Color,
                is_inverse: false,
            } => glow::SRC_COLOR,
            Col {
                input: Source,
                channel: Color,
                is_inverse: true,
            } => glow::ONE_MINUS_SRC_COLOR,
            Col {
                input: Source,
                channel: Alpha,
                is_inverse: false,
            } => glow::SRC_ALPHA,
            Col {
                input: Source,
                channel: Alpha,
                is_inverse: true,
            } => glow::ONE_MINUS_SRC_ALPHA,
            Col {
                input: Destination,
                channel: Color,
                is_inverse: false,
            } => glow::DST_COLOR,
            Col {
                input: Destination,
                channel: Color,
                is_inverse: true,
            } => glow::ONE_MINUS_DST_COLOR,
            Col {
                input: Destination,
                channel: Alpha,
                is_inverse: false,
            } => glow::DST_ALPHA,
            Col {
                input: Destination,
                channel: Alpha,
                is_inverse: true,
            } => glow::ONE_MINUS_DST_ALPHA,
            Col {
                input: GlobalBlend,
                channel: Color,
                is_inverse: false,
            } => glow::CONSTANT_COLOR,
            Col {
                input: GlobalBlend,
                channel: Color,
                is_inverse: true,
            } => glow::ONE_MINUS_CONSTANT_COLOR,
            Col {
                input: GlobalBlend,
                channel: Alpha,
                is_inverse: false,
            } => glow::CONSTANT_ALPHA,
            Col {
                input: GlobalBlend,
                channel: Alpha,
                is_inverse: true,
            } => glow::ONE_MINUS_CONSTANT_ALPHA,
        }
    }
}

/// A color to pull from when blending
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlendInput {
    /// The pixel that is being drawn
    Source,
    /// The pixel that is being replaced
    Destination,
    /// The color supplied to [`BlendMode::global_color`]
    GlobalBlend,
}

/// Which part of the [`BlendInput`] to pull from
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BlendChannel {
    /// The RGB component when using separate functions, or RGBA otherwise
    Color,
    /// The Alpha component
    Alpha,
}
