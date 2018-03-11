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
pub use self::animation::Animation;
pub use self::atlas::{Atlas, AtlasError, AtlasItem, AtlasLoader};
pub(crate) use self::backend::{Backend, Vertex};
pub use self::backend::BlendMode;
pub use self::color::Color;
pub use self::draw_call::DrawCall;
#[cfg(feature="fonts")] pub use self::font::{Font, FontLoader};
pub use self::image::{Image, ImageError, ImageLoader, PixelFormat};
pub use self::resize::ResizeStrategy;
pub use self::surface::Surface;
pub use self::view::View;
pub use self::window::{ImageScaleStrategy, Window, WindowBuilder};
