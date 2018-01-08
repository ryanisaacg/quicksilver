//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. Viewports allow points to be
//! converted between world and screen coordinates, which is useful for mice.

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
