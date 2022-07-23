//! `golem` is an opinionated mostly-safe graphics API
//!
//! When possible, `golem` should make simple things safe (bind objects before acting on them, or
//! check if they're bound for objects that are expensive to bind.) However, when not possible or
//! convenient (bounds checking the indices in an element buffer, for example), `golem` provides
//! unsafe APIs with well-defined safety conditions.
//!
//! A minimal example to display a triangle:
//!
//! ```rust
//! # use golem::*;
//! # use golem::Dimension::*;
//! # fn func(ctx: &Context) -> Result<(), GolemError> {
//! let vertices = [
//!     // Position         Color
//!     -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
//!     0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
//!     0.0, 0.5,           0.0, 0.0, 1.0, 1.0
//! ];
//! let indices = [0, 1, 2];
//!
//! let mut shader = ShaderProgram::new(
//!     ctx,
//!     ShaderDescription {
//!         vertex_input: &[
//!             Attribute::new("vert_position", AttributeType::Vector(D2)),
//!             Attribute::new("vert_color", AttributeType::Vector(D4)),
//!         ],
//!         fragment_input: &[Attribute::new("frag_color", AttributeType::Vector(D4))],
//!         uniforms: &[],
//!         vertex_shader: r#" void main() {
//!         gl_Position = vec4(vert_position, 0, 1);
//!         frag_color = vert_color;
//!     }"#,
//!         fragment_shader: r#" void main() {
//!         gl_FragColor = frag_color;
//!     }"#,
//!     },
//! )?;
//!
//! let mut vb = VertexBuffer::new(ctx)?;
//! let mut eb = ElementBuffer::new(ctx)?;
//! vb.set_data(&vertices);
//! eb.set_data(&indices);
//! shader.bind();
//!
//! ctx.clear();
//! unsafe {
//!     shader.draw(&vb, &eb, 0..indices.len(), GeometryMode::Triangles)?;
//! }
//! # Ok(()) }
//! ```
//!
//! The core type of `golem` is the [`Context`], which is constructed from the [`glow Context`].
//! From the [`Context`], [`ShaderProgram`]s are created, which take in data from [`Buffer`]s. Once
//! the data is uploaded to the GPU via [`Buffer::set_data`], it can be drawn via [`ShaderProgram::draw`].
//!
//! ## Initializing
//!
//! The user is responsible for windowing and providing a valid [`glow Context`] to create a
//! [`Context`]. You can try out the [`blinds`](https://crates.io/crates/blinds) crate, which works
//! well with `golem`, but using `winit` directly or other windowing solutions like `sdl2` are also
//! options.
//!
//! ## OpenGL Versions
//! It currently is implemented via glow, and it targets OpenGL 3.2 on desktop and WebGL 1 (so it
//! should run on a wide range of hardware.) GL 3.2 is selected for maximum desktop availability,
//! and WebGL 1 is available on 97% of clients to WebGL's 75% (taken from caniuse.com at time of
//! writing.)
//!
//! [`Context`]: crate::Context
//! [`glow Context`]: glow::Context

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::fmt::{Display, Formatter, Result as FmtResult};
use alloc::string::String;

// TODO: add out-of-memory to GolemError?
// TODO: unsafe audit: check for possible GL error conditions, and track them

use glow::HasContext;

type GlTexture = <glow::Context as HasContext>::Texture;
type GlProgram = <glow::Context as HasContext>::Program;
type GlShader = <glow::Context as HasContext>::Shader;
type GlFramebuffer = <glow::Context as HasContext>::Framebuffer;
type GlBuffer = <glow::Context as HasContext>::Buffer;
type GlVertexArray = <glow::Context as HasContext>::VertexArray;

mod attribute;
mod buffer;
mod context;
mod shader;
mod surface;
mod texture;
mod uniform;

pub mod blend;
pub mod depth;

pub use self::attribute::{Attribute, AttributeType};
pub use self::buffer::{Buffer, ElementBuffer, VertexBuffer};
pub use self::context::Context;
pub use self::shader::{ShaderDescription, ShaderProgram};
pub use self::surface::Surface;
pub use self::texture::{Texture, TextureFilter, TextureWrap};
pub use self::uniform::{Uniform, UniformType, UniformValue};

pub use glow;

pub(crate) enum Position {
    Input,
    Output,
}

/// Used to determine whether shader uniforms are ints or floats
pub enum NumberType {
    Int,
    Float,
}

/// How a pixel's color is laid out in memory
pub enum ColorFormat {
    /// One red pixel byte, followed by one blue, and one green
    RGB,
    /// One red, blue, green, then alpha (transparency)
    RGBA,
}

impl ColorFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            ColorFormat::RGBA => 4,
            ColorFormat::RGB => 3,
        }
    }

    fn gl_format(&self) -> u32 {
        match self {
            ColorFormat::RGB => glow::RGB,
            ColorFormat::RGBA => glow::RGBA,
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
/// The dimensionality of a vector or matrix shader input
///
/// D2 indicates a Vector2 or Matrix2x2, etc.
pub enum Dimension {
    D2 = 2,
    D3 = 3,
    D4 = 4,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
/// The GeometryMode determines how the data is drawn during [`ShaderProgram::draw`]
pub enum GeometryMode {
    /// Each element forms a single point
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1), (2), (3), (4), (5), (6)]`
    Points,
    /// Each pair of elements forms a thin line
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1, 2), (3, 4), (5, 6)]`
    Lines,
    /// Each pair of elements forms a chain of lines
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6)]`
    LineStrip,
    /// Each pair of elements forms a chain of lines, connected to the original
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 1)]`
    LineLoop,
    /// Each trio of elements forms a distinct triangle
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1, 2, 3), (4, 5, 6)]`
    Triangles,
    /// Each trio of elements forms a triangle, with the next vertex taking the previous two
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1, 2, 3), (2, 3, 4), (3, 4, 5), (4, 5, 6)]`
    TriangleStrip,
    /// The first elements forms the center of a fan, with each pair of vertices forming a triangle
    ///
    /// `[1, 2, 3, 4, 5, 6] -> [(1, 2, 3), (1, 3, 4), (1, 4, 5), (1, 5, 6)]`
    TriangleFan,
}

#[derive(Debug)]
/// The library's error conditions
pub enum GolemError {
    /// The OpenGL Shader compilation failed, with the given error message
    ///
    /// This may be during vertex-time, fragment-time, or link-time
    ShaderCompilationError(String),
    /// Some general error bubbling up from the GL context
    ContextError(String),
    /// An attempt was made to bind to an illegal uniform
    NoSuchUniform(String),
    /// An operation was performed on a shader that wasn't bound
    ///
    /// Shader operations include setting uniforms and drawing
    NotCurrentProgram,
    /// A texture filter requiring mipmaps was used when mipmaps were unavailable
    ///
    /// Mipmaps are only available for minification, and only for power-of-two sized textures (2x2, 4x4, etc.)
    MipMapsUnavailable,
    /// A wrap option was set for a Texture that isn't available
    ///
    /// Texture repeats are currently only supported for power-of-2 sized textures (2x2, 4x4, etc.)
    IllegalWrapOption,
}

impl From<String> for GolemError {
    fn from(other: String) -> Self {
        GolemError::ContextError(other)
    }
}

impl Display for GolemError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            GolemError::ShaderCompilationError(e) => write!(fmt, "Shader compilation: {}", e),
            GolemError::ContextError(e) => write!(fmt, "OpenGL: {}", e),
            GolemError::NoSuchUniform(e) => write!(fmt, "Illegal uniform: {}", e),
            GolemError::NotCurrentProgram => write!(fmt, "Shader program not bound"),
            GolemError::MipMapsUnavailable => write!(fmt, "Mipmaps are unavailable"),
            GolemError::IllegalWrapOption => write!(fmt, "An illegal texture wrap"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GolemError {}
