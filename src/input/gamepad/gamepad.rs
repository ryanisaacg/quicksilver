#[cfg(not(target_arch="wasm32"))] extern crate gilrs;

use input::{ButtonState, GamepadAxis, GamepadButton};
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    ops::Index
};

/// A queryable traditional 2-stick gamepad
#[derive(Clone)]
pub struct Gamepad {
    id: u32,
    buttons: [ButtonState; 17],
    axes: [f32; 4],
}

impl Gamepad {
    #[cfg(not(any(target_arch="wasm32", target_os="macos")))]
    pub(crate) fn new((id, _pad): (usize, &gilrs::Gamepad)) -> Gamepad {
        use gilrs::{
            Axis,
            ev::state::AxisData
        };
        fn axis_value(data: Option<&AxisData>) -> f32 {
            match data {
                Some(ref data) => data.value(),
                None => 0.0
            }
        }

        let id = id as u32;
        
        let axes = [
            axis_value(_pad.axis_data(Axis::LeftStickX)),
            axis_value(_pad.axis_data(Axis::LeftStickY)),
            axis_value(_pad.axis_data(Axis::RightStickX)),
            axis_value(_pad.axis_data(Axis::RightStickY))
        ];

        let mut buttons = [ButtonState::NotPressed; 17];
        for i in 0..ALL_BUTTONS.len() {
            let button = ALL_BUTTONS[i];
            let value = match _pad.button_data(button.into()) {
                Some(ref data) => data.is_pressed(),
                None => false
            };
            let state = if value { ButtonState::Pressed } else { ButtonState::Released };
            buttons[button as usize] = state;
        }
        
        Gamepad { id, axes, buttons }
    }

    #[cfg(target_arch="wasm32")]
    pub(crate) fn new(id: u32) -> Gamepad {
        use ffi::wasm;

        let axes = unsafe{[
            wasm::gamepad_axis(id, GamepadAxis::LeftStickX as u32),
            wasm::gamepad_axis(id, GamepadAxis::LeftStickY as u32),
            wasm::gamepad_axis(id, GamepadAxis::RightStickX as u32),
            wasm::gamepad_axis(id, GamepadAxis::RightStickY as u32)
        ]};

        let mut buttons = [ButtonState::NotPressed; 17];
        for i in 0..ALL_BUTTONS.len() {
            let button = ALL_BUTTONS[i];
            let value = unsafe { wasm::gamepad_button(id, button as u32) };
            let state = if value { ButtonState::Pressed } else { ButtonState::Released };
            buttons[button as usize] = state;
        }

        Gamepad { id, buttons, axes }          
    }

    pub(crate) fn set_previous(&mut self, _previous: Gamepad) {
        #[cfg(not(target_os="macos"))]
        for i in 0..ALL_BUTTONS.len() {
            let button = ALL_BUTTONS[i];
            let gamepad_button: GamepadButton = button.into();
            let state = self.buttons[gamepad_button as usize];
            self.buttons[gamepad_button as usize] = _previous[gamepad_button].update(state);
        }
    }

    /// Get the ID of the gamepad
    pub fn id(&self) -> u32 {
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

#[cfg(not(target_os="macos"))]
const ALL_BUTTONS: &[GamepadButton] = &[
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
