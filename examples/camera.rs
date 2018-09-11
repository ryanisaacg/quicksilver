// Demonstrate adding a View to the draw-geometry example
// The camera can be controlled with the arrow keys
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, View},
    input::{Key},
    lifecycle::{Settings, State, Window, run},
};

struct Camera {
    view: Rectangle
}

impl State for Camera {
    // Initialize the struct
    fn new() -> Result<Camera> {
        Ok(Camera {
            view: Rectangle::new_sized((800, 600))
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Left].is_down() {
            self.view = self.view.translate((-4, 0));
        }
        if window.keyboard()[Key::Right].is_down() {
            self.view = self.view.translate((4, 0));
        }
        if window.keyboard()[Key::Down].is_down() {
            self.view = self.view.translate((0, 4));
        }
        if window.keyboard()[Key::Up].is_down() {
            self.view = self.view.translate((0, -4));
        }
        window.set_view(View::new(self.view));
        Ok(())
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
        Ok(())
    }
}

fn main() {
    run::<Camera>("Camera", Vector::new(800, 600), Settings::default());
}

