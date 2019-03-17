//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod button_state;
mod gamepad;
mod key;
mod keyboard;
mod mouse;
mod mouse_cursor;

pub(crate) const LINES_TO_PIXELS: f32 = 15.0;

pub use self::{
    button_state::ButtonState,
    key::Key,
    gamepad::{Gamepad, GamepadAxis, GamepadButton},
    keyboard::Keyboard,
    mouse::{Mouse, MouseButton},
    mouse_cursor::MouseCursor
};
pub(crate) use self::key::KEY_LIST;
pub(crate) use self::gamepad::GAMEPAD_BUTTON_LIST;
#[cfg(feature = "gilrs")]
pub(crate) use self::gamepad::GILRS_GAMEPAD_LIST;
