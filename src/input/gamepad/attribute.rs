use gilrs::{Button};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// The axes a gamepad can report
pub enum GamepadAxis {
    /// The horizontal tilt of the left stick
    LeftStickX = 0,
    /// The vertical tilt of the left stick
    LeftStickY = 1,
    /// The horizontal tilt of the right stick
    RightStickX = 2,
    /// The vertical tilt of the right stick
    RightStickY = 3
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// A button on a gamepad
pub enum GamepadButton {
    /// The bottom face button
    ///
    /// This would be X on a Playstation controller, 
    /// or A on an XBOX controller
    FaceDown,
    /// The right face button
    ///
    /// This would be O on a Playstation controller,
    /// or B on an XBOX controller
    FaceRight,
    /// The left face button
    ///
    /// This would be Square on a Playstation controller,
    /// or X on an XBOX controller
    FaceLeft,
    /// The top face button
    ///
    /// This would be Triangle on a Playstation controller,
    /// or Y on an XBOX controller
    FaceUp,
    /// The shoulder button on the left of the controller
    ShoulderLeft,
    /// The shoulder button on the right of the controller
    ShoulderRight,
    /// The trigger button on the left of the controller
    TriggerLeft,
    /// The trigger button on the right of the controller
    TriggerRight,
    /// The left-most of the center buttons
    Select,
    /// The right-most of the center buttons
    Start,
    /// The button press that pushing in the left stick causes
    StickButtonLeft,
    /// The button press that pushing in the right stick causes
    StickButtonRight,
    /// The up button on the dpad
    DpadUp,
    /// The down button on the dpad
    DpadDown,
    /// The left button on the dpad
    DpadLeft,
    /// The right button on the dpad
    DpadRight,
    /// The middle of the center buttons
    Home
}

impl From<Button> for GamepadButton {
    fn from(other: Button) -> GamepadButton {
        match other {
            Button::South => GamepadButton::FaceDown,
            Button::East => GamepadButton::FaceRight,
            Button::North => GamepadButton::FaceUp,
            Button::West => GamepadButton::FaceLeft,
            Button::C => GamepadButton::ShoulderLeft,
            Button::Z => GamepadButton::TriggerLeft,
            Button::LeftTrigger => GamepadButton::ShoulderLeft,
            Button::LeftTrigger2 => GamepadButton::TriggerLeft,
            Button::RightTrigger => GamepadButton::ShoulderRight,
            Button::RightTrigger2 => GamepadButton::TriggerRight,
            Button::Select => GamepadButton::Select,
            Button::Start => GamepadButton::Start,
            Button::Mode => GamepadButton::Home,
            Button::LeftThumb => GamepadButton::StickButtonLeft,
            Button::RightThumb => GamepadButton::StickButtonRight,
            Button::DPadUp => GamepadButton::DpadUp,
            Button::DPadDown => GamepadButton::DpadDown,
            Button::DPadLeft => GamepadButton::DpadLeft,
            Button::DPadRight => GamepadButton::DpadRight,
            Button::Unknown => unreachable!(),
        }
    }
}
