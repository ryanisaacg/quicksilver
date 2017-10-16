extern crate glutin;

mod keyboard;
mod state;
mod mouse;

pub use glutin::VirtualKeyCode as Key;

pub use self::keyboard::{Keyboard};
pub use self::state::ButtonState;
pub use self::mouse::Mouse;
