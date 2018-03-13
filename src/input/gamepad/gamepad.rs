extern crate gilrs;

use gilrs::{Axis, Button};
use gilrs::ev::state::{AxisData, ButtonData};
use input::{ButtonState, GamepadAxis, GamepadButton};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::Index;

/// A queryable traditional 2-stick gamepad
#[derive(Clone)]
pub struct Gamepad {
    id: usize,
    buttons: [ButtonState; 17],
    axes: [f32; 4],
}

fn axis_value(data: Option<&AxisData>) -> f32 {
    match data {
        Some(ref data) => data.value(),
        None => 0.0
    }
}

fn button_value(data: Option<&ButtonData>) -> bool {
    match data {
        Some(ref data) => data.is_pressed(),
        None => false
    }
}

impl Gamepad {
    /// Construct a new gamepad
    pub(crate) fn new(data: (usize, &gilrs::Gamepad)) -> Gamepad {
        let (id, pad) = data;
        
        let axes = [
            axis_value(pad.axis_data(Axis::LeftStickX)),
            axis_value(pad.axis_data(Axis::LeftStickY)),
            axis_value(pad.axis_data(Axis::RightStickX)),
            axis_value(pad.axis_data(Axis::RightStickY))
        ];

        let mut buttons = [ButtonState::NotPressed; 17];
        for i in 0..ALL_BUTTONS.len() {
            let button = ALL_BUTTONS[i];
            let value = button_value(pad.button_data(button));
            let state = if value { ButtonState::Pressed } else { ButtonState::Released };
            let gamepad_button: GamepadButton = button.into();
            buttons[gamepad_button as usize] = state;
        }
        
        Gamepad {
            id,
            buttons,
            axes
        }
    }

    pub(crate) fn set_previous(&mut self, previous: Gamepad) {
        for i in 0..ALL_BUTTONS.len() {
            let button = ALL_BUTTONS[i];
            let gamepad_button: GamepadButton = button.into();
            let state = self.buttons[gamepad_button as usize];
            self.buttons[gamepad_button as usize] = previous[gamepad_button].update(state);
        }
    }

    /// Get the ID of the gamepad
    pub fn id(&self) -> usize {
        self.id
    }
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

impl PartialOrd for Gamepad {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Gamepad {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialEq for Gamepad {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Gamepad {}

const ALL_BUTTONS: &[Button] = &[
    Button::South,
    Button::East,
    Button::North,
    Button::West,
    Button::C,
    Button::Z,
    Button::LeftTrigger,
    Button::LeftTrigger2,
    Button::RightTrigger,
    Button::RightTrigger2,
    Button::Select,
    Button::Start,
    Button::Mode,
    Button::LeftThumb,
    Button::RightThumb,
    Button::DPadUp,
    Button::DPadDown,
    Button::DPadLeft,
    Button::DPadRight
];