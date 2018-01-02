#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

mod keyboard;
mod mouse;
mod state;
mod viewport;

#[cfg(not(target_arch="wasm32"))]
pub use glutin::VirtualKeyCode as Key;

pub use self::keyboard::Keyboard;
pub use self::mouse::Mouse;
pub use self::state::ButtonState;
pub use self::viewport::{Viewport, ViewportBuilder};
