#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use geom::Vector;
use input::ButtonState;

const LINES_TO_PIXELS: f32 = 15.0;

#[derive(Clone, Debug, Eq, PartialEq)]
///A simple mosue cursor abstraction
pub struct Mouse {
    pub(crate) pos: Vector,
    pub(crate) left: ButtonState,
    pub(crate) right: ButtonState,
    pub(crate) middle: ButtonState,
    pub(crate) wheel: Vector
}

impl Mouse {
    #[cfg(target_arch="wasm32")]
    pub(crate) fn process_button(&mut self, button: u32, state: bool) {
        let value = if state { ButtonState::Pressed } else { ButtonState::Released };
        match button {
            0 => self.left = value,
            1 => self.right = value,
            2 => self.middle = value,
            _ => (),
        }
    }

    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn process_button(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        let value = match state {
            glutin::ElementState::Pressed => ButtonState::Pressed,
            glutin::ElementState::Released => ButtonState::Released,
        };
        match button {
            glutin::MouseButton::Left => self.left = self.left.update(value),
            glutin::MouseButton::Right => self.right = self.right.update(value),
            glutin::MouseButton::Middle => self.middle = self.middle.update(value),
            _ => (),
        }
    }

    pub(crate) fn process_wheel_lines(&mut self, x: f32, y: f32) {
        let x = if x == 0.0 { x } else { x.signum() };
        let y = if y == 0.0 { y } else { y.signum() };
        self.process_wheel_pixels(x * LINES_TO_PIXELS, y * LINES_TO_PIXELS);
    }
    
    pub(crate) fn process_wheel_pixels(&mut self, x: f32, y: f32) {
        self.wheel = Vector::new(x, y);
    }

    pub(crate) fn clear_temporary_states(&mut self) {
        self.wheel = Vector::zero();
        self.left = self.left.clear_temporary();
        self.right = self.right.clear_temporary();
        self.middle = self.middle.clear_temporary();
    }

    ///The location of the cursor in the viewport space
    pub fn pos(&self) -> Vector {
        self.pos
    }

    ///The state of the left mouse button
    pub fn left(&self) -> ButtonState {
        self.left
    }

    ///The state of the right mouse button
    pub fn middle(&self) -> ButtonState {
        self.middle
    }

    ///The state of the middle mouse button
    pub fn right(&self) -> ButtonState {
        self.right
    }

    ///The amount the wheel moved this frame
    pub fn wheel(&self) -> Vector {
        self.wheel
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
            middle: ButtonState::NotPressed,
            wheel: Vector::zero()
        };
        mouse.process_wheel_lines(1.0, 1.0);
        for button in [glutin::MouseButton::Left, glutin::MouseButton::Right, glutin::MouseButton::Middle].iter() {
            for state in [glutin::ElementState::Pressed, glutin::ElementState::Released].iter() {
                mouse.process_button(state.clone(), button.clone());
            }
        }
        mouse.clear_temporary_states();
        assert_eq!(mouse.left, ButtonState::NotPressed);
        assert_eq!(mouse.right, ButtonState::NotPressed);
        assert_eq!(mouse.middle, ButtonState::NotPressed);
        assert_eq!(mouse.wheel, Vector::zero());
    }
}

