// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    run, Result, State,
    geom::{Circle, Rectangle, Vector, Transform, Line, Triangle},
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
        window.draw_ex(
            &Line::newv(Vector::new(50, 80),Vector::new(600, 450)).with_thickness(2.0),
            Transform::IDENTITY,
            Color::RED,
            5
        );
        window.draw_color(
            &Triangle::new(500, 50, 450, 100, 650, 150),
            Transform::rotate(45) * Transform::scale(Vector::new(0.5, 0.5)),
            Color::RED
        );
        window.present()
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", 800, 600));
}
