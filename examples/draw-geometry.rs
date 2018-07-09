// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    run, Result, State,
    geom::{Circle, Rectangle, Vector, Transform},
    graphics::{Color, Window, WindowBuilder}
};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.draw_color(&Rectangle::new(100, 100, 32, 32), Transform::IDENTITY, Color::BLUE);
        window.draw_ex(&Rectangle::new(400, 300, 32, 32), Transform::rotate(45), Color::BLUE, 10);
        window.draw_color(&Circle::new(400, 300, 100), Transform::IDENTITY, Color::GREEN);
        // TODO: restore line rendering functionality
        window.present()
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", 800, 600)).unwrap();
}
