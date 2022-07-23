//! An optional module to store the current state of input devices
//!
//! While the least error-prone way of handling input is to process events as they come in and
//! update your application's state accordingly, sometimes it is convenient or ergonomic to refer
//! to the global state of the input devices. The [`EventCache`] and [`CachedEventStream`] are
//! designed to make this easy and avoid some non-obvious pitfalls.
use crate::{
    Event, EventStream, GamepadAxis, GamepadButton, GamepadId, Key, MouseButton, PointerId,
};

use enum_map::EnumMap;
use mint::Vector2;
use rustc_hash::FxHashMap;

/// A wrapper around [`EventStream`] and [`EventCache`] for convenience
///
/// This is entirely equivalent to using a normal [`EventStream`] and passing all of its events
/// into an [`EventCache`]
pub struct CachedEventStream {
    events: EventStream,
    cache: EventCache,
}

impl CachedEventStream {
    pub fn new(events: EventStream) -> CachedEventStream {
        CachedEventStream {
            events,
            cache: EventCache::new(),
        }
    }

    /// See [`EventStream::next_event`]
    pub async fn next_event(&mut self) -> Option<Event> {
        let event = self.events.next_event().await;
        if let Some(ev) = &event {
            self.cache.process_event(ev);
        }

        event
    }

    pub fn cache(&self) -> &EventCache {
        &self.cache
    }
}

/// A struct that stores all input event values
///
/// While this is mostly takes care of book keeping necessary to store input event state, it also
/// handles cases like the window losing focus (which should completely reset input state).
///
/// It is capable of tracking individual [`pointer`]s and [`gamepad`]s, as well as the global
/// [`key`] state and [`mouse`].
///
/// [`pointer`]: EventCache::pointer
/// [`gamepad`]: EventCache::gamepad
/// [`key`]: EventCache::key
/// [`mouse`]: EventCache::mouse
#[derive(Default)]
pub struct EventCache {
    keys: EnumMap<Key, bool>,
    global_pointer: PointerState,
    pointers: FxHashMap<PointerId, PointerState>,
    gamepads: FxHashMap<GamepadId, GamepadState>,
}

impl EventCache {
    pub fn new() -> EventCache {
        EventCache::default()
    }

    /// Take an event and update the internal state to reflect it
    pub fn process_event(&mut self, event: &Event) {
        use Event::*;
        match event {
            KeyboardInput(ev) => {
                self.keys[ev.key()] = ev.is_down();
            }
            PointerEntered(ev) => self.ensure_pointer_exists(*ev.pointer()),
            PointerLeft(ev) => self.ensure_pointer_exists(*ev.pointer()),
            PointerMoved(ev) => {
                let pointer = *ev.pointer();
                self.ensure_pointer_exists(pointer);
                self.global_pointer.location = ev.location();
                self.pointers
                    .get_mut(&pointer)
                    .expect("Internal error: pointer failed to exist")
                    .location = ev.location();
            }
            PointerInput(ev) => {
                let pointer = *ev.pointer();
                self.ensure_pointer_exists(pointer);
                self.global_pointer
                    .process_button(ev.button(), ev.is_down());
                self.pointers
                    .get_mut(&pointer)
                    .expect("Internal error: pointer failed to exist")
                    .process_button(ev.button(), ev.is_down());
            }
            GamepadConnected(ev) => self.ensure_gamepad_exists(ev.gamepad().clone()),
            GamepadDisconnected(ev) => self.ensure_gamepad_exists(ev.gamepad().clone()),
            GamepadButton(ev) => {
                let gamepad = ev.gamepad();
                self.ensure_gamepad_exists(gamepad.clone());
                self.gamepads
                    .get_mut(gamepad)
                    .expect("Internal error: gamepad failed to exist")
                    .buttons[ev.button()] = ev.is_down();
            }
            GamepadAxis(ev) => {
                let gamepad = ev.gamepad();
                self.ensure_gamepad_exists(gamepad.clone());
                self.gamepads
                    .get_mut(gamepad)
                    .expect("Internal error: gamepad failed to exist")
                    .axes[ev.axis()] = ev.value();
            }
            FocusChanged(ev) if !ev.is_focused() => {
                self.clear();
            }
            _ => (),
        }
    }

    fn ensure_pointer_exists(&mut self, id: PointerId) {
        self.pointers.insert(id, PointerState::default());
    }

    fn ensure_gamepad_exists(&mut self, id: GamepadId) {
        self.gamepads.insert(id, GamepadState::default());
    }

    /// Clear all of the state
    pub fn clear(&mut self) {
        self.keys.clear();
        self.global_pointer.clear();
        self.pointers.clear();
        self.gamepads.clear();
    }

    /// Check if a given key is down
    pub fn key(&self, key: Key) -> bool {
        self.keys[key]
    }

    /// The state of the global mouse
    ///
    /// Under a system with touch input or with multiple cursors, this may report erratic results.
    /// The state here is tracked for every pointer event, regardless of pointer ID.
    pub fn mouse(&self) -> &PointerState {
        &self.global_pointer
    }

    /// The state of the given pointer
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn pointer(&self, id: &PointerId) -> Option<&PointerState> {
        self.pointers.get(id)
    }

    /// The pointer ID and values that have been tracked
    pub fn pointers(&self) -> impl Iterator<Item = (&PointerId, &PointerState)> {
        self.pointers.iter()
    }

    /// The state of the given gamepad
    pub fn gamepad(&self, id: &GamepadId) -> Option<&GamepadState> {
        self.gamepads.get(id)
    }

    /// The gamepad ID and values that have been tracked
    pub fn gamepads(&self) -> impl Iterator<Item = (&GamepadId, &GamepadState)> {
        self.gamepads.iter()
    }
}

pub struct PointerState {
    left: bool,
    right: bool,
    middle: bool,
    location: Vector2<f32>,
    other: FxHashMap<u16, bool>,
}

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

    pub fn other(&self, button: u16) -> bool {
        self.other.get(&button).copied().unwrap_or(false)
    }

    pub fn location(&self) -> Vector2<f32> {
        self.location
    }

    fn clear(&mut self) {
        self.left = false;
        self.right = false;
        self.middle = false;
        self.other.clear();
    }

    fn process_button(&mut self, button: MouseButton, is_down: bool) {
        match button {
            MouseButton::Left => self.left = is_down,
            MouseButton::Right => self.right = is_down,
            MouseButton::Middle => self.middle = is_down,
            MouseButton::Other(idx) => {
                self.other.insert(idx, is_down);
            }
        }
    }
}

impl Default for PointerState {
    fn default() -> PointerState {
        PointerState {
            left: false,
            right: false,
            middle: false,
            location: Vector2 { x: 0.0, y: 0.0 },
            other: FxHashMap::default(),
        }
    }
}

#[derive(Default)]
pub struct GamepadState {
    buttons: EnumMap<GamepadButton, bool>,
    axes: EnumMap<GamepadAxis, f32>,
}

impl GamepadState {
    pub fn button(&self, button: GamepadButton) -> bool {
        self.buttons[button]
    }

    pub fn axis(&self, axis: GamepadAxis) -> f32 {
        self.axes[axis]
    }
}
