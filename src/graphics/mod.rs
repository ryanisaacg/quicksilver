//! A module to draw 2D graphics in a window
//! 
//! It also includes image loading

mod backend;
mod camera;
mod canvas;
mod color;
mod image;
mod resize;
mod window;
pub(crate) use self::backend::{Backend, Vertex};
pub use self::camera::Camera;
pub use self::canvas::Canvas;
pub use self::color::Color;
pub use self::image::{Image, ImageError, PixelFormat};
pub use self::resize::ResizeStrategy;
pub use self::window::{Window, WindowBuilder};
