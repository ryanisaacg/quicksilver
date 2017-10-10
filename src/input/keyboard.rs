extern crate glutin;

use input::State;
use std::ops::Index;

#[derive(Copy)]
pub struct Keyboard {
    keys: [State; 256]
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [State::NotPressed; 256]
        }
    }

    pub fn process_event(&mut self, event: &glutin::KeyboardInput) {
        let index = event.scancode as usize;
        let previous_state = self.keys[index];
        self.keys[index] = match event.state {
            glutin::ElementState::Pressed => 
                if previous_state.is_down() { State::Held } else { State::Pressed },
            glutin::ElementState::Released => 
                if previous_state.is_down() { State::Released } else { State::NotPressed }
        };
    }

    pub fn clear_temporary_states(&mut self) {
        for index in 0..self.keys.len() {
            self.keys[index] = match self.keys[index] {
                State::Pressed => State::Held,
                State::Released => State::NotPressed,
                _ => self.keys[index]
            };
        }
    }
}

impl Clone for Keyboard {
    fn clone(&self) -> Keyboard {
        *self
    }
}

impl Index<u32> for Keyboard {
    type Output = State;
    
    fn index(&self, index: u32) -> &State {
        &self.keys[index as usize]
    }
}
