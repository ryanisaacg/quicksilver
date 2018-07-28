//! A module to draw 2D graphics in a window
//!  It also includes image loading

mod animation;
mod atlas;
mod backend;
mod color;
mod drawable;
#[cfg(feature="fonts")] mod font;
mod image;
#[cfg(feature="immi")] mod immi;
mod mesh;
mod resize;
mod surface;
mod vertex;
mod view;
mod window;

pub use self::{
    animation::Animation,
    atlas::{Atlas, AtlasError, AtlasItem},
    backend::{BlendMode, ImageScaleStrategy},
    color::Color,
    drawable::{Background, Drawable},
    image::{Image, ImageError, PixelFormat},
    mesh::Mesh,
    resize::ResizeStrategy,
    surface::Surface,
    vertex::{Vertex, GpuTriangle},
    view::View,
    window::{Window, WindowBuilder}
};
#[cfg(feature = "fonts")] pub use self::font::{Font, FontStyle};
#[cfg(feature = "immi")] pub use self::immi::ImmiRender;
pub(crate) use self::backend::{Backend, BackendImpl, ImageData, SurfaceData};

