use {
    Result,
    input::Gamepad,
};
#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
use {
    gilrs::{self, Button},
    input::{ButtonState, GAMEPAD_BUTTON_LIST},
};
#[cfg(target_arch = "wasm32")]
use {
    input::ButtonState,
    stdweb::web::Gamepad as WebGamepad,
};


pub struct GamepadProvider {
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

#[cfg(all(not(any(target_arch="wasm32", target_os="macos")), feature = "gamepads"))]
const GILRS_GAMEPAD_LIST: &[Button] = &[
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
