extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Shape, Rectangle, Vector},
    graphics::{Background::Col, Color},
    input::MouseCursor,
    lifecycle::{Event, Settings, State, Window, run}
};

struct RectangleState {
    grab_rect: Rectangle,
    crosshair_rect: Rectangle,
}

impl State for RectangleState {
    fn new() -> Result<Self> {
        Ok(RectangleState {
            grab_rect: Rectangle::new((0, 0), (200, 100)).with_center((400, 100)),
            crosshair_rect: Rectangle::new((0, 0), (200, 100)).with_center((400, 400)),
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::MouseMoved(vector) => {
                if vector.overlaps_rectangle(&self.crosshair_rect) {
                    window.set_cursor(MouseCursor::Crosshair);
                } else if vector.overlaps_rectangle(&self.grab_rect) {
                    window.set_cursor(MouseCursor::Grab);
                } else {
                    window.set_cursor(MouseCursor::Default);
                }
            }
            _ => {}
        };

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        window.draw(&self.grab_rect, Col(Color::RED));
        window.draw(&self.crosshair_rect, Col(Color::GREEN));

        Ok(())
    }
}

fn main() {
    run::<RectangleState>("set-cursor", Vector::new(800, 600), Settings::default());
}
