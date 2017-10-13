extern crate glutin;

use input::State;

use geom::Vector;

pub struct Mouse {
    pub pos: Vector,
    pub left: State,
    pub right: State,
    pub middle: State
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            pos: Vector::newi(0, 0),
            left: State::NotPressed,
            right: State::NotPressed,
            middle: State::NotPressed
        }
    }

    pub fn set_position(&mut self, pos: (f64, f64)) {
        let (x, y) = pos;
        self.pos = Vector::new(x as f32, y as f32);
    }

    pub fn process_button(&mut self, state: glutin::ElementState, button: glutin::MouseButton) {
        let value = match state {
            glutin::ElementState::Pressed => State::Pressed,
            glutin::ElementState::Released => State::Released
        };
        match button {
            glutin::MouseButton::Left => self.left = value,
            glutin::MouseButton::Right => self.right = value,
            glutin::MouseButton::Middle => self.middle = value,
            _ => ()
        }
    }
}
