//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod boolean;
mod button;
mod event;
#[cfg(feature="gamepads")] mod gamepad;
mod key;
mod keyboard;
mod mouse;

pub use self::{
    boolean::*,
    button::{Button, ButtonState},
    event::Event,
    key::Key,
    keyboard::Keyboard,
    mouse::{Mouse, MouseButton}
};
#[cfg(feature="gamepads")] pub use self::gamepad::{Gamepad, GamepadAxis, GamepadButton};
#[cfg(feature="gamepads")] pub(crate) use self::gamepad::GamepadManager;
pub(crate) use self::key::KEY_LIST;
