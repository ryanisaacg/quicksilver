mod attribute;
mod gamepad;
mod manager;

pub use self::attribute::{GamepadAxis, GamepadButton};
pub use self::gamepad::Gamepad;
pub(crate) use self::manager::GamepadManager;