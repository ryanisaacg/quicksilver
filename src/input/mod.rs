//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod key;
mod keyboard;
mod mouse;
mod state;

pub use self::key::Key;
pub use self::keyboard::Keyboard;
pub use self::mouse::Mouse;
pub use self::state::ButtonState;
