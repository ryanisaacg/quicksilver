//! Read events / input state
//!
//! The main struct for this module is [`Input`], which is provided by the [`run`] method to your
//! app. [`Input`] allows you to read events from the user, which can range from [`window resizes`]
//! to [`key presses`].
//!
//! There is also an optional feature (enabled by default) to cache input for user convenience.
//! This allows you to write quick expressions like `if input.key_down(Key::W)` rather than having
//! to write code to handle the event when it comes in.
//!
//! [`run`]: crate::run::run
//! [`window resizes`]: ResizedEvent
//! [`key presses`]: KeyboardEvent

use crate::geom::Vector;

pub use blinds::event::{
    FocusChangedEvent, GamepadAxisEvent, GamepadButtonEvent, GamepadConnectedEvent,
    GamepadDisconnectedEvent, KeyboardEvent, ModifiersChangedEvent, PointerEnteredEvent,
    PointerInputEvent, PointerLeftEvent, ReceivedCharacterEvent, ScaleFactorChangedEvent,
    ScrollDelta,
};
#[cfg(feature = "event-cache")]
use blinds::event_cache::EventCache;
/// The button and axis values of a gamepad
#[cfg(feature = "event-cache")]
pub use blinds::event_cache::GamepadState;
pub use blinds::{GamepadAxis, GamepadButton, GamepadId, Key, MouseButton, PointerId};

/// The source of events and input device state
pub struct Input {
    source: blinds::EventStream,
    #[cfg(feature = "event-cache")]
    cache: EventCache,
}

impl Input {
    pub(crate) fn new(source: blinds::EventStream) -> Input {
        Input {
            source,
            #[cfg(feature = "event-cache")]
            cache: EventCache::new(),
        }
    }

    /// Retrieve the next event from the environment, or wait until there is one
    ///
    /// If an event has occured since this method was last called, it will be return as
    /// `Some(event)`. Once all events have been handled, `None` will be returned. At this point you
    /// should run any update or drawing logic in your app. When this method is called after it
    /// returns `None`, it will yield control back to the environment until your app should run
    /// again.
    pub async fn next_event(&mut self) -> Option<Event> {
        while let Some(ev) = self.source.next_event().await {
            #[cfg(feature = "event-cache")]
            self.cache.process_event(&ev);
            // If there is an event, it might not be something Quicksilver can process.
            // If it's not, skip this and get the next event
            if let Some(ev) = conv(ev) {
                return Some(ev);
            }
        }

        // We didn't have any Some events before we hit a None
        None
    }
}

#[cfg(feature = "event-cache")]
impl Input {
    /// Check if a given key is down
    pub fn key_down(&self, key: Key) -> bool {
        self.cache.key(key)
    }

    /// The state of the global mouse
    ///
    /// Under a system with touch input or with multiple cursors, this may report erratic results.
    /// The state here is tracked for every pointer event, regardless of pointer ID.
    pub fn mouse(&self) -> PointerState {
        self.cache.mouse().into()
    }

    /// The state of the given pointer
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn pointer(&self, id: &PointerId) -> Option<PointerState> {
        self.cache.pointer(id).map(|p| p.into())
    }

    /// The pointer ID and values that have been tracked
    pub fn pointers(&self) -> impl Iterator<Item = (&PointerId, PointerState)> {
        self.cache.pointers().map(|(id, p)| (id, p.into()))
    }

    /// The state of the given gamepad
    pub fn gamepad(&self, id: &GamepadId) -> Option<&GamepadState> {
        self.cache.gamepad(id)
    }

    /// The gamepad ID and values that have been tracked
    pub fn gamepads(&self) -> impl Iterator<Item = (&GamepadId, &GamepadState)> {
        self.cache.gamepads()
    }
}

/// The buttons and location of a given pointer
#[cfg(feature = "event-cache")]
pub struct PointerState {
    left: bool,
    right: bool,
    middle: bool,
    location: Vector,
}

#[cfg(feature = "event-cache")]
impl PointerState {
    pub fn left(&self) -> bool {
        self.left
    }

    pub fn right(&self) -> bool {
        self.right
    }

    pub fn middle(&self) -> bool {
        self.middle
    }

    pub fn location(&self) -> Vector {
        self.location
    }
}

#[cfg(feature = "event-cache")]
impl From<&blinds::event_cache::PointerState> for PointerState {
    fn from(ps: &blinds::event_cache::PointerState) -> PointerState {
        PointerState {
            left: ps.left(),
            right: ps.right(),
            middle: ps.middle(),
            location: ps.location().into(),
        }
    }
}

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
    size: Vector,
}

impl ResizedEvent {
    /// The new size of the window
    pub fn size(&self) -> Vector {
        self.size
    }
}

#[derive(Clone, Debug)]
/// See [`Event::PointerMoved`]
///
/// [`Event::PointerMoved`]: crate::input::Event::PointerMoved
pub struct PointerMovedEvent {
    id: PointerId,
    location: Vector,
}

impl PointerMovedEvent {
    pub fn pointer(&self) -> &PointerId {
        &self.id
    }

    /// The logical location of the pointer, relative to the top-left of the window
    pub fn location(&self) -> Vector {
        self.location
    }
}

fn conv(ev: blinds::Event) -> Option<Event> {
    use Event::*;
    Some(match ev {
        blinds::Event::Resized(x) => Resized(ResizedEvent {
            size: x.logical_size().into(),
        }),
        blinds::Event::ScaleFactorChanged(x) => ScaleFactorChanged(x),
        blinds::Event::FocusChanged(x) => FocusChanged(x),
        blinds::Event::ReceivedCharacter(x) => ReceivedCharacter(x),
        blinds::Event::KeyboardInput(x) => KeyboardInput(x),
        blinds::Event::PointerEntered(x) => PointerEntered(x),
        blinds::Event::PointerLeft(x) => PointerLeft(x),
        blinds::Event::PointerMoved(x) => PointerMoved(PointerMovedEvent {
            id: *x.pointer(),
            location: x.location().into(),
        }),
        blinds::Event::PointerInput(x) => PointerInput(x),
        blinds::Event::ScrollInput(x) => ScrollInput(x),
        blinds::Event::ModifiersChanged(x) => ModifiersChanged(x),
        blinds::Event::GamepadConnected(x) => GamepadConnected(x),
        blinds::Event::GamepadDisconnected(x) => GamepadDisconnected(x),
        blinds::Event::GamepadButton(x) => GamepadButton(x),
        blinds::Event::GamepadAxis(x) => GamepadAxis(x),
        _ => return None,
    })
}
