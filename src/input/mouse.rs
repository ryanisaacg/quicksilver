extern crate glutin;

use geom::{Rectangle, Transform, Vector};
use input::{ButtonState, Viewport, ViewportBuilder};

#[derive(Clone)]
pub struct MouseBuilder {
    pub(crate) mouse: Mouse,
    pub(crate) viewport: ViewportBuilder
}

impl MouseBuilder {
    pub fn transform(&self, trans: Transform) -> MouseBuilder {
        MouseBuilder {
            viewport: self.viewport.transform(trans),
            ..self.clone()
        }
    }

    pub fn build(&self, area: Rectangle) -> (Mouse, Viewport) {
        let viewport = self.viewport.build(area);
        let mouse = Mouse {
            pos: viewport.project() * self.mouse.pos,
            ..self.mouse
        };
        (mouse, viewport)
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mouse {
    pub pos: Vector,
    pub left: ButtonState,
    pub right: ButtonState,
    pub middle: ButtonState,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos: Vector::newi(0, 0),
            left: ButtonState::NotPressed,
            right: ButtonState::NotPressed,
            middle: ButtonState::NotPressed,
        }
    }

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
        let mut mouse = Mouse::new();
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

