extern crate glutin;

use input::ButtonState;
use std::ops::Index;

#[derive(Copy)]
pub struct Keyboard {
    keys: [ButtonState; 256]
}

#[derive(Clone, Copy)]
#[repr(usize)]
pub enum Key {
    Escape = 1, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Zero, Minus, Plus, Backspace, Tab,
    Q, W, E, R, T, Y, U, I, O, P, LeftBracket, RightBracket, Enter, LeftControl,
    A, S, D, F, G, H, J, K, L, Colon, Quote, Tilde, LeftShift, Backslash,
    Z, X, C, V, B, N, M, Comma, Period, Forwardslash, RightShift, LeftAlt, Spacebar, CapsLock,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, NumLock, ScrollLock
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [ButtonState::NotPressed; 256]
        }
    }

    pub fn process_event(&mut self, event: &glutin::KeyboardInput) {
        let index = event.scancode as usize;
        let previous_state = self.keys[index];
        self.keys[index] = match event.state {
            glutin::ElementState::Pressed => 
                if previous_state.is_down() { ButtonState::Held } else { ButtonState::Pressed },
            glutin::ElementState::Released => 
                if previous_state.is_down() { ButtonState::Released } else { ButtonState::NotPressed }
        };
    }

    pub fn clear_temporary_states(&mut self) {
        for index in 0..self.keys.len() {
            self.keys[index] = match self.keys[index] {
                ButtonState::Pressed => ButtonState::Held,
                ButtonState::Released => ButtonState::NotPressed,
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

impl Index<Key> for Keyboard {
    type Output = ButtonState;
    
    fn index(&self, index: Key) -> &ButtonState {
        &self.keys[index as usize]
    }
}
