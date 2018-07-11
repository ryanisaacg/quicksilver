#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
extern crate gilrs;

#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
use gilrs::Button;

#[cfg(target_arch="wasm32")]
use stdweb::web::Gamepad as WebGamepad;

use {
    Result,
    input::{ButtonState, Event},
    std::ops::Index
};

/// A queryable traditional 2-stick gamepad
#[derive(Copy, Clone, Debug)]
pub struct Gamepad {
    id: i32,
    buttons: [ButtonState; 17],
    axes: [f32; 4],
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

pub(crate) struct GamepadProvider {
    #[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
    gilrs: gilrs::Gilrs
}

impl GamepadProvider {
    pub fn new() -> Result<GamepadProvider> {
        Ok(GamepadProvider {
            #[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
            gilrs: gilrs::Gilrs::new()?
        })
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
            let id = id as i32;

            let axes = [
                axis_value(gamepad.axis_data(Axis::LeftStickX)),
                axis_value(gamepad.axis_data(Axis::LeftStickY)),
                axis_value(gamepad.axis_data(Axis::RightStickX)),
                axis_value(gamepad.axis_data(Axis::RightStickY))
            ];

            let mut buttons = [ButtonState::NotPressed; 17];
            for i in 0..GAMEPAD_BUTTON_LIST.len() {
                let button = GAMEPAD_BUTTON_LIST[i];
                let value = match gamepad.button_data(GILRS_GAMEPAD_LIST[i]) {
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
        buffer.extend(WebGamepad::get_all()
            .iter()
            .filter_map(|pad| match pad {
                &Some(ref pad) => Some(pad),
                &None => None
            })
            .map(|pad| {
                let id = pad.index() as i32;
                let mut axes = [0.0; 4];
                let in_axes = pad.axes();
                for i in 0..axes.len().min(in_axes.len()) {
                    axes[i] = in_axes[i] as f32;
                }
                let mut buttons = [ButtonState::NotPressed; 17];
                let in_buttons = pad.buttons();
                for i in 0..buttons.len().min(in_buttons.len()) {
                    buttons[i] = if in_buttons[i].pressed() {
                            ButtonState::Pressed
                        } else {
                            ButtonState::Released
                        };
                }
                Gamepad { id, buttons, axes }
            }));
    }

    #[cfg(any(target_os="macos", not(feature = "gamepads")))]
    fn provide_gamepads_impl(&self, _buffer: &mut Vec<Gamepad>) {
        //Inentionally a no-op
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

const GAMEPAD_BUTTON_LIST: &[GamepadButton] = &[
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

#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
const GILRS_GAMEPAD_LIST: &[gilrs::Button] = &[
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

const GAMEPAD_AXIS_LIST: &[GamepadAxis] = &[
    GamepadAxis::LeftStickX,
    GamepadAxis::LeftStickY,
    GamepadAxis::RightStickX,
    GamepadAxis::RightStickY,
];

