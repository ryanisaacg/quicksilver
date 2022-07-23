use mint::Vector2;
mod convert;
mod gamepad;
mod keyboard;
mod pointer;

pub(crate) use self::convert::*;
pub use self::gamepad::*;
pub use self::keyboard::*;
pub use self::pointer::*;

#[derive(Clone, Debug)]
#[non_exhaustive]
/// An indicator something has changed or input has been dispatched
pub enum Event {
    /// The size of the window has changed, see [`Window::size`]
    ///
    /// [`Window::size`]: crate::Window::size
    Resized(ResizedEvent),
    /// The scale factor of the window has changed, see [`Window::scale_factor`]
    ///
    /// [`Window::scale_factor`]: crate::Window::scale_factor
    ScaleFactorChanged(ScaleFactorChangedEvent),
    /// The window has gained operating system focus (true), or lost it (false)
    FocusChanged(FocusChangedEvent),
    /// The user typed a character, used for text input
    ///
    /// Don't use keyboard events for text! Depending on how the user's operating system and
    /// keyboard layout are configured, different keys may produce different Unicode characters.
    ReceivedCharacter(ReceivedCharacterEvent),
    /// A key has been pressed, released, or held down
    ///
    /// Operating systems often have key repeat settings that cause duplicate events to be
    /// generated for a single press.
    KeyboardInput(KeyboardEvent),
    /// A pointer entered the window
    PointerEntered(PointerEnteredEvent),
    /// A pointer has exited the window
    PointerLeft(PointerLeftEvent),
    /// A pointer has a new position, relative to the window's top-left
    PointerMoved(PointerMovedEvent),
    /// A button on a pointer, likely a mouse, has produced an input
    PointerInput(PointerInputEvent),
    /// The mousewheel has scrolled, either in lines or pixels (depending on the input method)
    ScrollInput(ScrollDelta),
    /// The keyboard modifiers (e.g. shift, alt, ctrl) have changed
    ModifiersChanged(ModifiersChangedEvent),
    /// A gamepad has been connected
    GamepadConnected(GamepadConnectedEvent),
    /// A gamepad has been disconnected
    GamepadDisconnected(GamepadDisconnectedEvent),
    /// A gamepad button has been pressed or released
    GamepadButton(GamepadButtonEvent),
    /// A gamepad axis has changed its value
    GamepadAxis(GamepadAxisEvent),
}

#[derive(Clone, Debug)]
/// See [`Event::Resized`]
pub struct ResizedEvent {
    pub(crate) size: Vector2<f32>,
}

impl ResizedEvent {
    /// The new logical size of the window, taking into account DPI
    pub fn logical_size(&self) -> Vector2<f32> {
        self.size
    }
}

#[derive(Clone, Debug)]
/// See [`Event::ScaleFactorChanged`]
pub struct ScaleFactorChangedEvent {
    pub(crate) scale: f32,
}

impl ScaleFactorChangedEvent {
    pub fn scale_factor(&self) -> f32 {
        self.scale
    }
}

#[derive(Clone, Debug)]
/// See [`Event::FocusChanged`]
pub struct FocusChangedEvent {
    pub(crate) focus: bool,
}

impl FocusChangedEvent {
    pub fn is_focused(&self) -> bool {
        self.focus
    }
}

#[derive(Clone, Debug)]
/// See [`Event::ReceivedCharacter`]
pub struct ReceivedCharacterEvent {
    pub(crate) chr: char,
}

impl ReceivedCharacterEvent {
    /// The character entered by the user
    pub fn character(&self) -> char {
        self.chr
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// A change in the event modifiers like shift, control, alt, or 'logo'
///
/// See [`Event::ModifiersChanged`]
pub struct ModifiersChangedEvent {
    shift: bool,
    ctrl: bool,
    alt: bool,
    logo: bool,
}

impl ModifiersChangedEvent {
    pub fn shift(self) -> bool {
        self.shift
    }

    pub fn ctrl(self) -> bool {
        self.ctrl
    }

    pub fn alt(self) -> bool {
        self.alt
    }

    /// Windows, Command, etc.
    pub fn logo(self) -> bool {
        self.logo
    }
}
