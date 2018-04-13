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
    /// Return if there was a state change, and if so what was changed
    pub(crate) fn process_event(&mut self, keycode: usize, pressed: ButtonState) {
        self.keys[keycode] = self.keys[keycode].update(pressed);
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
