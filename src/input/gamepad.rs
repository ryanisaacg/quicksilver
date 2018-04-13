#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
extern crate gilrs;

use input::{ButtonState, Event};
use std::ops::Index;

/// A queryable traditional 2-stick gamepad
#[derive(Clone)]
pub struct Gamepad {
    id: u32,
    buttons: [ButtonState; 17],
    axes: [f32; 4],
}

impl Gamepad {
    pub(crate) fn new(id: u32) -> Gamepad {
        Gamepad { id, buttons: [ButtonState::NotPressed; 17], axes: [0.0; 4] }
    }

    pub(crate) fn clear_temporary_states(&mut self) {
        for button in self.buttons.iter_mut() {
            *button =  button.clear_temporary();
        }
    }

    pub(crate) fn set_axis(&mut self, axis: GamepadAxis, val: f32) {
        self.axes[axis as usize] = val;
    }

    pub(crate) fn set_button(&mut self, button: GamepadButton, val: ButtonState) {
        self.buttons[button as usize] = val;
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
    pub fn id(&self) -> u32 {
        self.id
    }
}

pub(crate) struct GamepadProvider {
    #[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
    gilrs: gilrs::Gilrs
}

impl GamepadProvider {
    pub fn new() -> GamepadProvider {
        GamepadProvider {
            #[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
            gilrs: gilrs::Gilrs::new().unwrap()
        }
    }

    pub fn provide_gamepads(&mut self, buffer: &mut Vec<Gamepad>) {
        self.provide_gamepads_impl(buffer);
    }

    #[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
    fn provide_gamepads_impl(&mut self, buffer: &mut Vec<Gamepad>) {
        while let Some(ev) = self.gilrs.next_event() {
            self.gilrs.update(&ev);
        }
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
        buffer.extend(self.gilrs.gamepads().map(|(id, gamepad)| {
            let id = id as u32;
            
            let axes = [
                axis_value(gamepad.axis_data(Axis::LeftStickX)),
                axis_value(gamepad.axis_data(Axis::LeftStickY)),
                axis_value(gamepad.axis_data(Axis::RightStickX)),
                axis_value(gamepad.axis_data(Axis::RightStickY))
            ];

            let mut buttons = [ButtonState::NotPressed; 17];
            for i in 0..GAMEPAD_BUTTON_LIST.len() {
                let button = GAMEPAD_BUTTON_LIST[i];
                let value = match gamepad.button_data(button.into()) {
                    Some(ref data) => data.is_pressed(),
                    None => false
                };
                let state = if value { ButtonState::Pressed } else { ButtonState::Released };
                buttons[button as usize] = state;
            }

            Gamepad { id, axes, buttons }
        }));
    }

    #[cfg(target_arch="wasm32")]
    fn provide_gamepads_impl(&self, buffer: &mut Vec<Gamepad>) {
        use std::os::raw::c_void;
        use ffi::wasm;
        buffer.push(Gamepad::new(0));
        buffer.push(Gamepad::new(0));
        unsafe {
            let gamepad_count = wasm::gamepad_count() as usize;
            buffer.reserve(gamepad_count);
            let start = &mut buffer[0] as *mut Gamepad as *mut c_void;
            let id_ptr = &mut buffer[0].id as *mut u32;
            let button_ptr = &mut buffer[0].buttons[0] as *mut ButtonState as *mut u32;
            let axis_ptr = &mut buffer[0].axes[0] as *mut f32;
            let next_id = &mut buffer[1] as *mut Gamepad as *mut c_void;
            wasm::gamepad_data(start, id_ptr, button_ptr, axis_ptr, next_id);
            buffer.set_len(gamepad_count);
            if gamepad_count < 2 {
                buffer.truncate(2 - gamepad_count);
            }
        }
    }

    #[cfg(any(target_os="macos", not(feature = "gamepads")))]
    fn provide_gamepads_impl(&self, buffer: &mut Vec<Gamepad>) {
        //Inentionally a no-op
    }
}

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

#[repr(u32)]
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

pub const GAMEPAD_BUTTON_LIST: &[GamepadButton] = &[
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

pub const GAMEPAD_AXIS_LIST: &[GamepadAxis] = &[
    GamepadAxis::LeftStickX,
    GamepadAxis::LeftStickY,
    GamepadAxis::RightStickX,
    GamepadAxis::RightStickY,
];

#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]impl Into<gilrs::Button> for GamepadButton {
    fn into(self) -> gilrs::Button {
        use gilrs::Button;
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
