//! A collection of polling input structures
//!
//! The Keyboard is indexed by Keys, allowing polling of a button state. The Mouse tracks the
//! standard three buttons, the mouse wheel, and the mouse position. 

mod key;
mod keyboard;
mod mouse;
mod state;

pub use self::key::Key;
pub use self::keyboard::Keyboard;
pub use self::mouse::{Mouse, MouseButton};
pub use self::state::ButtonState;

/// A unified button input for mouse and keyboard
pub enum UnifiedButton {
    /// A mouse button
    Mouse(MouseButton),
    /// A keyboard key
    Keyboard(Key)
}
