#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use geom::Vector;
use input::ButtonState;

#[derive(Clone, Debug, Eq, PartialEq)]
///A simple mosue cursor abstraction
pub struct Mouse {
    ///The location of the cursor in the viewport space
    pub pos: Vector,
    ///The state of the left mouse button
    pub left: ButtonState,
    ///The state of the right mouse button
    pub right: ButtonState,
    ///The state of the middle mouse button
    pub middle: ButtonState,
}

impl Mouse {
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn process_button(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        let value = match state {
            glutin::ElementState::Pressed => ButtonState::Pressed,
            glutin::ElementState::Released => ButtonState::Released,
        };
        match button {
            glutin::MouseButton::Left => self.left = value,
            glutin::MouseButton::Right => self.right = value,
            glutin::MouseButton::Middle => self.middle = value,
            _ => (),
        }
    }

    pub(crate) fn clear_temporary_states(&mut self) {
        self.left = self.left.clear_temporary();
        self.right = self.right.clear_temporary();
        self.middle = self.middle.clear_temporary();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_presses() {
        let mut mouse = Mouse {
            pos: Vector::zero(),
            left: ButtonState::NotPressed,
            right: ButtonState::NotPressed,
            middle: ButtonState::NotPressed
        };
        for button in [glutin::MouseButton::Left, glutin::MouseButton::Right, glutin::MouseButton::Middle].iter() {
            for state in [glutin::ElementState::Pressed, glutin::ElementState::Released].iter() {
                mouse.process_button(state.clone(), button.clone());
            }
        }
        mouse.clear_temporary_states();
        assert_eq!(mouse.left, ButtonState::NotPressed);
        assert_eq!(mouse.right, ButtonState::NotPressed);
        assert_eq!(mouse.middle, ButtonState::NotPressed);
    }
}

