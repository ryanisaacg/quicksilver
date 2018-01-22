use input::{ButtonState, Key};
use std::ops::Index;

#[derive(Copy)]
/// A structure that stores each key's state
///
/// Keyboards are maintained and owned by a `Window`, and can be accessed via the `keyboard`
/// function. They are indexed by the Key enum.
pub struct Keyboard {
    pub(crate) keys: [ButtonState; 256],
}

impl Keyboard {
    pub(crate) fn process_event(&mut self, keycode: usize, pressed: bool) {
        self.keys[keycode] = self.keys[keycode].update(if pressed { ButtonState::Pressed } 
                                                       else { ButtonState::Released });
    }

    pub(crate) fn clear_temporary_states(&mut self) {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypress() {
        let mut keyboard = Keyboard {
            keys: [ButtonState::NotPressed; 256]
        };
        keyboard.process_event(Key::A as usize, true);
        assert_eq!(keyboard[Key::A], ButtonState::Pressed);
        keyboard.process_event(Key::A as usize, true);
        assert_eq!(keyboard[Key::A], ButtonState::Pressed);
        keyboard.process_event(Key::A as usize, false);
        assert_eq!(keyboard[Key::A], ButtonState::Released);
        keyboard.process_event(Key::A as usize, false);
        assert_eq!(keyboard[Key::A], ButtonState::Released);
    }

    #[test]
    fn clear_states() {
        let mut keyboard = Keyboard {
            keys: [ButtonState::NotPressed; 256].clone()
        };
        keyboard.process_event(Key::A as usize, true);
        keyboard.clear_temporary_states();
        assert_eq!(keyboard[Key::A], ButtonState::Held);
        keyboard.process_event(Key::A as usize, false);
        keyboard.clear_temporary_states();
        assert_eq!(keyboard[Key::A], ButtonState::NotPressed);
    }
}
