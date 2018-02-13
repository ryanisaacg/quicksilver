//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod boolean;
mod button;
mod key;
mod keyboard;
mod mouse;

pub use self::boolean::*;
pub use self::button::{Button, ButtonState};
pub use self::key::Key;
pub(crate) use self::key::KEY_LIST;
pub use self::keyboard::Keyboard;
pub use self::mouse::{Mouse, MouseButton};
