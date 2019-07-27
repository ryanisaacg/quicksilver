//! A module to draw 2D graphics in a window
//!  It also includes image loading

mod atlas;
mod blend_mode;
mod color;
mod drawable;
#[cfg(feature="fonts")] mod font;
#[cfg(feature="lyon")] mod lyon;
mod image;
mod image_scale_strategy;
mod mesh;
mod resize;
mod surface;
mod vertex;
mod view;

pub use self::{
    atlas::{Atlas, AtlasError, AtlasItem},
    blend_mode::BlendMode,
    color::Color,
    drawable::{Background, Drawable},
    image::{Image, ImageError, PixelFormat},
    image_scale_strategy::ImageScaleStrategy,
    mesh::Mesh,
    resize::ResizeStrategy,
    surface::Surface,
    vertex::{Vertex, GpuTriangle},
    view::View,
};
#[cfg(feature="fonts")] pub use self::font::{Font, FontStyle};
#[cfg(feature="lyon")] pub use self::lyon::ShapeRenderer;

