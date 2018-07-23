// Draw a pulsing circle in the middle of the window
extern crate quicksilver;

use quicksilver::{
    run, Result, State,
    geom::{Circle, Transform, Vector},
    graphics::{Color, RenderTarget, Window, WindowBuilder}
};

struct PulsingCircle {
    step: f32,
}

impl State for PulsingCircle {
    fn new() -> Result<PulsingCircle> {
        Ok(PulsingCircle { step: 0.0 })
    }

    fn update(&mut self, _window: &mut Window) -> Result<()> {
        self.step = (self.step + 1.0) % 360.0;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let scale = Transform::scale(Vector::ONE * (1.0 + (self.step.to_radians().sin() / 2.0)));
        window.draw_ex(&Circle::new((400, 300), 50), scale, Color::GREEN, 0);
        window.present()
    }
}

fn main() {
    run::<PulsingCircle>(WindowBuilder::new("Pulsing Circle", (800, 600)));
}
