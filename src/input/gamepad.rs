use geom::Vector;
use input::ButtonState;
use std::ops::Index;

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

/// A queryable traditional 2-stick gamepad
pub struct Gamepad {
    buttons: [ButtonState; 18],
    left: Vector,
    right: Vector
}

impl Gamepad {
    /// The left stick's value, normalized on the range [-1.0, 1.0]
    pub fn left(&self) -> Vector {
        self.left
    }

    /// The right stick's value, normalized on the range [-1.0, 1.0]
    pub fn right(&self) -> Vector {
        self.right
    }
}

impl Index<GamepadButton> for Gamepad {
    type Output = ButtonState;

    fn index(&self, index: GamepadButton) -> &ButtonState {
        &self.buttons[index as usize]
    }
}