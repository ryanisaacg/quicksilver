mod attribute;
mod gamepad;
mod manager;

pub use self::{
    attribute::{GamepadAxis, GamepadButton},
    gamepad::Gamepad
};
pub(crate) use self::{
    attribute::{GAMEPAD_AXIS_LIST, GAMEPAD_BUTTON_LIST},
    manager::GamepadManager
};
