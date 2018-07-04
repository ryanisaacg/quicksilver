//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The
//! Mouse tracks the standard three buttons, the mouse wheel, and the mouse
//! position.

mod button_state;
mod event;
mod gamepad;
mod key;
mod keyboard;
mod mouse;

pub(crate) const LINES_TO_PIXELS: f32 = 15.0;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use self::event::EventProvider;
pub use self::{
    button_state::ButtonState,
    event::Event,
    gamepad::{Gamepad, GamepadAxis, GamepadButton},
    key::Key,
    keyboard::Keyboard,
    mouse::{Mouse, MouseButton},
};
pub(crate) use self::{gamepad::GamepadProvider, key::KEY_LIST};
