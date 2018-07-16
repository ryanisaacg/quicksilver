//! A module to draw 2D graphics in a window
//!  It also includes image loading

mod animation;
mod atlas;
mod backend;
mod color;
mod drawable;
#[cfg(feature="fonts")] mod font;
mod image;
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
    drawable::{DrawAttributes, Drawable},
    image::{Image, ImageError, PixelFormat},
    resize::ResizeStrategy,
    surface::Surface,
    vertex::{Vertex, GpuTriangle},
    view::View,
    window::{Window, WindowBuilder}
};
#[cfg(feature="fonts")] pub use self::font::{Font, FontStyle};
pub(crate) use self::backend::{Backend, BackendImpl, ImageData, SurfaceData};
