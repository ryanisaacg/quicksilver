// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{run, Result, State, geom::{Circle, Rectangle, Transform, Vector},
                  graphics::{Color, Sprite, Window, WindowBuilder}};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::black())?;
        window.draw(&Sprite::rectangle(Rectangle::new(100, 100, 32, 32)).with_color(Color::red()));
        window.draw(&Sprite::rectangle(Rectangle::new(400, 300, 32, 32))
            .with_color(Color::blue())
            .with_transform(Transform::rotate(45))
            .with_z(10));
        window.draw(&Sprite::circle(Circle::new(400, 300, 100)).with_color(Color::green()));
        window.draw(&Sprite::line(
            Vector::new(100, 150),
            Vector::new(450, 350),
            2.0,
        ));
        window.present()
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", 800, 600)).unwrap();
}
