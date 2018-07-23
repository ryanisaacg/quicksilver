// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    run, Result, State,
    geom::{Circle, Rectangle, Transform, Line, Triangle},
    graphics::{Color, RenderTarget, Window, WindowBuilder}
};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.draw_ex(&Rectangle::new((100, 100), (32, 32)), Transform::IDENTITY, Color::BLUE, 0);
        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Transform::rotate(45), Color::BLUE, 10);
        window.draw_ex(&Circle::new((400, 300), 100), Transform::IDENTITY, Color::GREEN, 0);
        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Transform::IDENTITY,
            Color::RED,
            5
        );
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            Color::RED,
            0
        );
        window.present()
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", (800, 600)));
}
