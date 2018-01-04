#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

mod key;
mod keyboard;
mod mouse;
mod state;
mod viewport;

pub use self::key::Key;
pub use self::keyboard::Keyboard;
pub use self::mouse::Mouse;
pub use self::state::ButtonState;
pub use self::viewport::{Viewport, ViewportBuilder};
