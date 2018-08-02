// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Rectangle, Transform, Line, Triangle},
    graphics::{Background::Col, Color, Window, WindowBuilder},
    lifecycle::{State, run},
};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
        window.draw(&Circle::new((400, 300), 100), Col(Color::GREEN));
        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        window.present()
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", (800, 600)));
}
