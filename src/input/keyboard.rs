extern crate glutin;

use input::{ButtonState, Key};
use std::ops::Index;

#[derive(Copy)]
pub struct Keyboard {
    keys: [ButtonState; 256],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { keys: [ButtonState::NotPressed; 256] }
    }

    pub fn process_event(&mut self, event: &glutin::KeyboardInput) {
        if let Some(keycode) = event.virtual_keycode {
            let index = keycode as usize;
            let previous_state = self.keys[index];
            self.keys[index] = match event.state {
                glutin::ElementState::Pressed => {
                    if previous_state.is_down() {
                        ButtonState::Held
                    } else {
                        ButtonState::Pressed
                    }
                }
                glutin::ElementState::Released => {
                    if previous_state.is_down() {
                        ButtonState::Released
                    } else {
                        ButtonState::NotPressed
                    }
                }
            };
        }
    }

    pub fn clear_temporary_states(&mut self) {
        for index in 0..self.keys.len() {
            self.keys[index] = self.keys[index].clear_temporary();
        }
    }
}

impl Clone for Keyboard {
    fn clone(&self) -> Keyboard {
        *self
    }
}

impl Index<Key> for Keyboard {
    type Output = ButtonState;

    fn index(&self, index: Key) -> &ButtonState {
        &self.keys[index as usize]
    }
}
