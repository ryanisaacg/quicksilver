//! A module to draw 2D graphics in a window
//! 
//! It also includes image loading

mod animation;
mod backend;
mod canvas;
mod color;
mod image;
mod resize;
mod surface;
mod view;
mod window;
pub use self::animation::Animation;
pub(crate) use self::backend::{Backend, Vertex};
pub use self::backend::BlendMode;
pub use self::canvas::Canvas;
pub use self::color::Color;
pub use self::image::{Image, ImageError, ImageLoader, PixelFormat};
pub use self::resize::ResizeStrategy;
pub use self::surface::Surface;
pub use self::view::View;
pub use self::window::{Window, WindowBuilder};
