//! A module to draw 2D graphics in a window
//! 
//! It also includes image loading

mod animation;
mod atlas;
mod backend;
mod color;
mod draw_call;
#[cfg(feature="fonts")] mod font;
mod image;
mod resize;
mod surface;
mod view;
mod window;
pub use self::{
    animation::Animation,
    atlas::{Atlas, AtlasError, AtlasItem, AtlasLoader},
    backend::BlendMode,
    color::Color,
    draw_call::DrawCall,
    image::{Image, ImageError, ImageLoader, PixelFormat},
    resize::ResizeStrategy,
    surface::Surface,
    view::View,
    window::{ImageScaleStrategy, Window, WindowBuilder}
};
#[cfg(feature="fonts")] pub use self::font::{Font, FontLoader};
pub(crate) use self::backend::{Backend, Vertex};
