#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use geom::Vector;
use input::ButtonState;
use std::ops::Index;

const LINES_TO_PIXELS: f32 = 15.0;

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
    #[cfg(target_arch="wasm32")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_presses() {
        let mut mouse = Mouse {
            pos: Vector::zero(),
            buttons: [ButtonState::NotPressed; 3],
            wheel: Vector::zero()
        };
        mouse.process_wheel_lines(1.0, 1.0);
        for button in [glutin::MouseButton::Left, glutin::MouseButton::Right, glutin::MouseButton::Middle].iter() {
            for state in [glutin::ElementState::Pressed, glutin::ElementState::Released].iter() {
                mouse.process_button(state.clone(), button.clone());
            }
        }
        mouse.clear_temporary_states();
        assert_eq!(mouse[MouseButton::Left], ButtonState::NotPressed);
        assert_eq!(mouse[MouseButton::Right], ButtonState::NotPressed);
        assert_eq!(mouse[MouseButton::Middle], ButtonState::NotPressed);
        assert_eq!(mouse.wheel, Vector::zero());
    }
}

