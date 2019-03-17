use crate::{
    input::ButtonState,
    lifecycle::Event,
};
#[cfg(feature = "gilrs")]
use gilrs::Button;
use std::ops::Index;

/// A queryable traditional 2-stick gamepad
#[derive(Copy, Clone, Debug)]
pub struct Gamepad {
    pub(crate) id: i32,
    pub(crate) buttons: [ButtonState; 17],
    pub(crate) axes: [f32; 4],
}

impl Gamepad {
    pub(crate) fn clear_temporary_states(&mut self) {
        for button in self.buttons.iter_mut() {
            *button =  button.clear_temporary();
        }
    }

    pub(crate) fn set_previous(&mut self, previous: &Gamepad, events: &mut Vec<Event>) {
        for button in GAMEPAD_BUTTON_LIST.iter() {
            if self[*button].is_down() != previous[*button].is_down() {
                self.buttons[*button as usize] = if self[*button].is_down() {
                    ButtonState::Pressed
                } else {
                    ButtonState::Released
                };
                events.push(Event::GamepadButton(self.id(), *button, self[*button]));
            }
        }
        for axis in GAMEPAD_AXIS_LIST.iter() {
            if self[*axis] != previous[*axis] {
                events.push(Event::GamepadAxis(self.id(), *axis, self[*axis]));
            }
        }
    }

    /// Get the ID of the gamepad
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

impl Index<GamepadAxis> for Gamepad {
    type Output = f32;

    fn index(&self, index: GamepadAxis) -> &f32 {
        &self.axes[index as usize]
    }
}

impl Index<GamepadButton> for Gamepad {
    type Output = ButtonState;

    fn index(&self, index: GamepadButton) -> &ButtonState {
        &self.buttons[index as usize]
    }
}

pub(crate) const GAMEPAD_BUTTON_LIST: &[GamepadButton] = &[
    GamepadButton::FaceDown,
    GamepadButton::FaceRight,
    GamepadButton::FaceUp,
    GamepadButton::FaceLeft,
    GamepadButton::ShoulderLeft,
    GamepadButton::TriggerLeft,
    GamepadButton::ShoulderRight,
    GamepadButton::TriggerRight,
    GamepadButton::Select,
    GamepadButton::Start,
    GamepadButton::Home,
    GamepadButton::StickButtonLeft,
    GamepadButton::StickButtonRight,
    GamepadButton::DpadUp,
    GamepadButton::DpadDown,
    GamepadButton::DpadLeft,
    GamepadButton::DpadRight,
];

const GAMEPAD_AXIS_LIST: &[GamepadAxis] = &[
    GamepadAxis::LeftStickX,
    GamepadAxis::LeftStickY,
    GamepadAxis::RightStickX,
    GamepadAxis::RightStickY,
];

#[cfg(feature = "gilrs")]
pub(crate) const GILRS_GAMEPAD_LIST: &[Button] = &[
    Button::South,
    Button::East,
    Button::North,
    Button::West,
    Button::LeftTrigger,
    Button::LeftTrigger2,
    Button::RightTrigger,
    Button::RightTrigger2,
    Button::South,
    Button::South,
    Button::Mode,
    Button::LeftThumb,
    Button::RightThumb,
    Button::DPadUp,
    Button::DPadDown,
    Button::DPadLeft,
    Button::DPadRight,
];
