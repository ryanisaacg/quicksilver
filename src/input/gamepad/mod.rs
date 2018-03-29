mod attribute;
mod gamepad;
mod manager;

pub use self::{
    attribute::{GamepadAxis, GamepadButton},
    gamepad::Gamepad
};
pub(crate) use self::manager::GamepadManager;