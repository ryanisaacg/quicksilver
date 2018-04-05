#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use geom::Vector;
use input::ButtonState;
use std::ops::Index;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

    /*#[cfg(target_arch="wasm32")]
    pub(crate) fn process_button(&mut self, button: u32, state: bool) -> Option<(MouseButton, ButtonState)> {
        if button < 3 {
            let value = if state { ButtonState::Pressed } else { ButtonState::Released };
            if value != self.buttons[button as usize] {
                self.buttons[button as usize] = value;
                Some((match button {
                    0 => MouseButton::Left,
                    1 => MouseButton::Right,
                    2 => MouseButton::Middle,
                    _ => unreachable!()
                }, value))
            } else {
                None
            }
        } else {
            None
        }
    }

    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn process_button(&mut self, state: glutin::ElementState, button: glutin::MouseButton) -> Option<(MouseButton, ButtonState)> {
        let value = match state {
            glutin::ElementState::Pressed => ButtonState::Pressed,
            glutin::ElementState::Released => ButtonState::Released,
        };
        let index = match button {
            glutin::MouseButton::Left => MouseButton::Left,
            glutin::MouseButton::Right => MouseButton::Right,
            glutin::MouseButton::Middle => MouseButton::Middle,
            _ => { return None; },
        }; 
        let updated = self.buttons[index as usize].update(value);
        if updated != self.buttons[index as usize] {
            self.buttons[index as usize] = updated;
            Some((index, updated))
        } else {
            None
        }
    }*/

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
