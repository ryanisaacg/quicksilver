// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Rectangle, Transform},
    graphics::{Color, Draw, Window, WindowBuilder}
};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> DrawGeometry { DrawGeometry }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        window.draw(&Draw::rectangle(Rectangle::new(100, 100, 32, 32)).with_color(Color::red()));
        window.draw(&Draw::rectangle(Rectangle::new(400, 300, 32, 32)).with_color(Color::blue()).with_transform(Transform::rotate(45)).with_z(10));
        window.draw(&Draw::circle(Circle::new(400, 300, 100)).with_color(Color::green()));
        window.present();
   }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", 800, 600));
}
