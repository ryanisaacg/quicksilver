extern crate glutin;

mod builder;
mod keyboard;
mod mouse;
mod state;
mod viewport;

pub use glutin::VirtualKeyCode as Key;

pub use self::builder::InputBuilder;
pub use self::keyboard::Keyboard;
pub use self::mouse::Mouse;
pub use self::state::ButtonState;
pub use self::viewport::*;

