// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{State, run};
use quicksilver::geom::{Circle, Rectangle, Transform};
use quicksilver::graphics::{Color, DrawCall, Window, WindowBuilder};

struct DrawGeometry;

impl State for DrawGeometry {
    fn configure() -> Window {
        WindowBuilder::new().build("Draw Geometry", 800, 600)
    }

   fn new() -> DrawGeometry { DrawGeometry }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        let items = [
            DrawCall::rectangle(Rectangle::new(100, 100, 32, 32)).with_color(Color::red()),
            DrawCall::rectangle(Rectangle::new(400, 300, 32, 32)).with_color(Color::blue()).with_transform(Transform::rotate(45)).with_z(10),
            DrawCall::circle(Circle::new(400, 300, 100)).with_color(Color::green())
        ];
        window.draw(&items);
        window.present();
   }
}

fn main() {
    run::<DrawGeometry>();
}
