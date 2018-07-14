use geom::Vector;
use input::ButtonState;
use std::ops::Index;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// The different buttons a user can press on a mouse
pub enum MouseButton {
    /// The left mouse button
    Left = 0, 
    /// The right mouse button
    Right = 1, 
    /// The middle mouse button
    Middle = 2
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// A simple mouse cursor abstraction
///
/// Mice are owned and maintained a `Window` and can be accessed via the `mouse` function.
pub struct Mouse {
    pub(crate) pos: Vector,
    pub(crate) buttons: [ButtonState; 3],
    pub(crate) wheel: Vector
}

impl Mouse {
    pub(crate) fn process_button(&mut self, button: MouseButton, state: ButtonState) {
        self.buttons[button as usize] = self.buttons[button as usize].update(state);
    }

    pub(crate) fn clear_temporary_states(&mut self) {
        self.wheel = Vector::zero();
        for button in self.buttons.iter_mut() {
            *button = button.clear_temporary();
        }
    }

    ///The location of the cursor in the viewport space
    pub fn pos(&self) -> Vector {
        self.pos
    }

    ///The amount the wheel moved this frame
    pub fn wheel(&self) -> Vector {
        self.wheel
    }
}

impl Index<MouseButton> for Mouse {
    type Output = ButtonState;

    fn index(&self, index: MouseButton) -> &ButtonState {
        &self.buttons[index as usize]
    }
}