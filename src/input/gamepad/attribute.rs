#[cfg(not(target_arch="wasm32"))] use gilrs::{Button};

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

#[cfg(not(target_arch="wasm32"))]
impl Into<Button> for GamepadButton {
    fn into(self) -> Button {
        match self {
            GamepadButton::FaceDown => Button::South,
            GamepadButton::FaceRight => Button::East,
            GamepadButton::FaceUp => Button::North,
            GamepadButton::FaceLeft => Button::West,
            GamepadButton::ShoulderLeft => Button::LeftTrigger,
            GamepadButton::TriggerLeft => Button::LeftTrigger2,
            GamepadButton::ShoulderRight => Button::RightTrigger,
            GamepadButton::TriggerRight => Button::RightTrigger2,
            GamepadButton::Select => Button::South,
            GamepadButton::Start => Button::South,
            GamepadButton::Home => Button::Mode,
            GamepadButton::StickButtonLeft => Button::LeftThumb,
            GamepadButton::StickButtonRight => Button::RightThumb,
            GamepadButton::DpadUp => Button::DPadUp,
            GamepadButton::DpadDown => Button::DPadDown,
            GamepadButton::DpadLeft => Button::DPadLeft,
            GamepadButton::DpadRight => Button::DPadRight,
        }
    }
}

