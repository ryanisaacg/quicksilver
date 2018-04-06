//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod button_state;
mod event;
#[cfg(feature="gamepads")] mod gamepad;
mod key;
mod keyboard;
mod mouse;

pub use self::{
    button_state::ButtonState,
    event::Event,
    key::Key,
    gamepad::{Gamepad, GamepadAxis, GamepadButton},
    keyboard::Keyboard,
    mouse::{Mouse, MouseButton}
};
#[cfg(not(target_arch="wasm32"))] pub(crate) use self::event::EventProvider;
pub(crate) use self::{
    button_state::BUTTON_STATE_LIST,
    gamepad::{GAMEPAD_AXIS_LIST, GAMEPAD_BUTTON_LIST},
    key::KEY_LIST,
    mouse::MOUSE_BUTTON_LIST
};
