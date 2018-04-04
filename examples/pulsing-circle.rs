// Draw a pulsing circle in the middle of the window
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Vector, Transform},
    graphics::{Color, Sprite, Window, WindowBuilder}
};

struct PulsingCircle {
    step: f32
}

impl State for PulsingCircle {
    fn configure() -> Window {
        WindowBuilder::new().build("Pulsing Circle", 800, 600)
    }

   fn new() -> PulsingCircle { 
       PulsingCircle { step: 0.0 }
   }

   fn update(&mut self, _window: &mut Window) {
       self.step = (self.step + 1.0) % 360.0;
   }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        let scale = Transform::scale(Vector::one() * (1.0 + (self.step.to_radians().sin() / 2.0)));
        window.draw(&Sprite::circle(Circle::new(400, 300, 50)).with_color(Color::green()).with_transform(scale));
        window.present();
   }
}

fn main() {
    run::<PulsingCircle>();
}
