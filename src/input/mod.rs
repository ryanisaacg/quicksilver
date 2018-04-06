//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod button;
mod event;
#[cfg(feature="gamepads")] mod gamepad;
mod key;
mod keyboard;
mod mouse;

pub use self::{
    button::{Button, ButtonState},
    event::Event,
    key::Key,
    gamepad::{Gamepad, GamepadAxis, GamepadButton},
    keyboard::Keyboard,
    mouse::{Mouse, MouseButton}
};
pub(crate) use self::{
    button::BUTTON_STATE_LIST,
    event::EventProvider,
    gamepad::{GAMEPAD_AXIS_LIST, GAMEPAD_BUTTON_LIST},
    key::KEY_LIST,
    mouse::MOUSE_BUTTON_LIST
};
