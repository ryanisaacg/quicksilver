extern crate glutin;

use input::ButtonState;

use geom::Vector;

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

    pub fn set_position(&mut self, pos: (f64, f64), scale: f32) {
        let (x, y) = pos;
        self.pos = Vector::new(x as f32 / scale, y as f32 / scale);
    }

    pub fn process_button(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
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

    pub fn clear_temporary_states(&mut self) {
        self.left = self.left.clear_temporary();
        self.right = self.right.clear_temporary();
        self.middle = self.middle.clear_temporary();
    }
}
