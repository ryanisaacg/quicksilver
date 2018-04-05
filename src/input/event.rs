use input::{ButtonState, GamepadAxis, GamepadButton, Key, MouseButton};
use geom::Vector;

/// An input event
pub enum Event {
    /// The application has been closed
    Closed,
    /// The application has gained focus
    Focused,
    /// The application has lost focus
    Unfocused,
    /// A key has changed its button state
    Key(Key, ButtonState),
    /// The mouse has been moved to a position
    MouseMoved(Vector),
    /// The mouse has entered the window
    MouseEntered,
    /// The mouse has exited the window
    MouseExited,
    /// The mouse wheel has been scrolled by a vector
    MouseWheel(Vector),
    /// A mouse button has changed its button state
    MouseButton(MouseButton, ButtonState),
    /// A gamepad axis has changed its state
    GamepadAxis(u32, GamepadAxis, f32),
    /// A gamepad button has changed its state
    GamepadButton(u32, GamepadButton, ButtonState),
    /// A gamepad has been connected
    GamepadConnected(u32),
    /// A gamepad has been disconnected
    GamepadDisconnected(u32)
}