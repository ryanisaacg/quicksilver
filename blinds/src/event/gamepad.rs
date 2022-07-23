use std::cmp::Ordering;

#[derive(Clone, Debug)]
/// See [`Event::GamepadConnected`]
///
/// [`Event::GamepadConnected`]: crate::event::Event::GamepadConnected
pub struct GamepadConnectedEvent(pub(crate) GamepadId);

impl GamepadConnectedEvent {
    pub fn gamepad(&self) -> &GamepadId {
        &self.0
    }
}

#[derive(Clone, Debug)]
/// See [`Event::GamepadDisconnected`]
///
/// [`Event::GamepadDisconnected`]: crate::event::Event::GamepadDisconnected
pub struct GamepadDisconnectedEvent(pub(crate) GamepadId);

impl GamepadDisconnectedEvent {
    pub fn gamepad(&self) -> &GamepadId {
        &self.0
    }
}

#[derive(Clone, Debug)]
/// See [`Event::GamepadButton`]
///
/// [`Event::GamepadButton`]: crate::event::Event::GamepadButton
pub struct GamepadButtonEvent {
    pub(crate) id: GamepadId,
    pub(crate) button: GamepadButton,
    pub(crate) is_down: bool,
    pub(crate) is_repeat: bool,
}

impl GamepadButtonEvent {
    /// Which gamepad generated the event
    pub fn gamepad(&self) -> &GamepadId {
        &self.id
    }

    pub fn button(&self) -> GamepadButton {
        self.button
    }

    /// If the button is now down, either repeating or down for the first time
    pub fn is_down(&self) -> bool {
        self.is_down
    }

    /// If this event is a repeat of a previous down event
    pub fn is_repeat(&self) -> bool {
        self.is_repeat
    }
}

#[derive(Clone, Debug)]
/// See [`Event::GamepadAxis`]
///
/// [`Event::GamepadAxis`]: crate::event::Event::GamepadAxis
pub struct GamepadAxisEvent {
    pub(crate) id: GamepadId,
    pub(crate) axis: GamepadAxis,
    pub(crate) value: f32,
}

impl GamepadAxisEvent {
    /// Which gamepad generated the event
    pub fn gamepad(&self) -> &GamepadId {
        &self.id
    }

    pub fn axis(&self) -> GamepadAxis {
        self.axis
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
/// A unique ID for a gamepad that persists after the device is unplugged
pub struct GamepadId(
    #[cfg(feature = "gilrs")] pub(crate) gilrs::GamepadId,
    #[cfg(not(feature = "gilrs"))] usize,
);

impl PartialOrd for GamepadId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GamepadId {
    fn cmp(&self, other: &Self) -> Ordering {
        let a: usize = self.0.into();
        let b: usize = other.0.into();
        a.cmp(&b)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
/// A button on a standard (d-pad, 2-stick, 4-button, 4-trigger) gamepad
pub enum GamepadButton {
    Start,
    Select,

    /// The north face button.
    ///
    /// * Nintendo: X
    /// * Playstation: Triangle
    /// * XBox: Y
    North,
    /// The south face button.
    ///
    /// * Nintendo: B
    /// * Playstation: X
    /// * XBox: A
    South,
    /// The east face button.
    ///
    /// * Nintendo: A
    /// * Playstation: Circle
    /// * XBox: B
    East,
    /// The west face button.
    ///
    /// * Nintendo: Y
    /// * Playstation: Square
    /// * XBox: X
    West,

    /// The left stick was pressed in as a button
    LeftStick,
    /// The right stick was pressed in as a button
    RightStick,

    LeftTrigger,
    RightTrigger,

    LeftShoulder,
    RightShoulder,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
/// The stick axes of a gamepad
pub enum GamepadAxis {
    LeftStickX,
    LeftStickY,

    RightStickX,
    RightStickY,
}

#[cfg(feature = "gilrs")]
pub(crate) fn convert_gilrs_button(event: gilrs::ev::Button) -> Option<GamepadButton> {
    use gilrs::ev::Button::*;
    Some(match event {
        South => GamepadButton::South,
        East => GamepadButton::East,
        North => GamepadButton::North,
        West => GamepadButton::West,
        LeftTrigger => GamepadButton::LeftShoulder,
        LeftTrigger2 => GamepadButton::LeftShoulder,
        RightTrigger => GamepadButton::RightShoulder,
        RightTrigger2 => GamepadButton::RightTrigger,
        Select => GamepadButton::Select,
        Start => GamepadButton::Start,
        LeftThumb => GamepadButton::LeftStick,
        RightThumb => GamepadButton::RightStick,
        DPadUp => GamepadButton::DPadUp,
        DPadDown => GamepadButton::DPadDown,
        DPadLeft => GamepadButton::DPadLeft,
        DPadRight => GamepadButton::DPadRight,

        C | Z | Unknown | Mode => return None,
    })
}

#[cfg(feature = "gilrs")]
pub(crate) fn convert_gilrs_axis(axis: gilrs::ev::Axis) -> Option<GamepadAxis> {
    use gilrs::ev::Axis::*;

    Some(match axis {
        LeftStickX => GamepadAxis::LeftStickX,
        LeftStickY => GamepadAxis::LeftStickY,
        RightStickX => GamepadAxis::RightStickX,
        RightStickY => GamepadAxis::RightStickY,

        LeftZ | RightZ | DPadX | DPadY | Unknown => return None,
    })
}
