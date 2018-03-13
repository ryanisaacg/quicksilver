//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod boolean;
mod button;
#[cfg(feature="gamepads")] mod gamepad;
mod key;
mod keyboard;
mod mouse;

pub use self::boolean::*;
pub use self::button::{Button, ButtonState};
#[cfg(feature="gamepads")] pub(crate) use self::gamepad::GamepadManager;
#[cfg(feature="gamepads")] pub use self::gamepad::{Gamepad, GamepadAxis, GamepadButton};
pub use self::key::Key;
pub(crate) use self::key::KEY_LIST;
pub use self::keyboard::Keyboard;
pub use self::mouse::{Mouse, MouseButton};
