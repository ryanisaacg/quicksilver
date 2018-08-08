// Draw some multi-colored geometry to the screen
// This is a good place to get a feel for the basic structure of a Quicksilver app
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Rectangle, Transform, Line, Triangle},
    graphics::{Background::Col, Color, Window, WindowBuilder},
    lifecycle::{State, run},
};

// A unit struct that we're going to use to run the Quicksilver functions
// If we wanted to store persistent state, we would put it in here.
struct DrawGeometry;

impl State for DrawGeometry {
    // Initialize the struct
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // Remove any lingering artifacts from the previous frame
        window.clear(Color::WHITE)?;
        // Draw a rectangle with a top-left corner at (100, 100) and a width and height of 32 with
        // a blue background
        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        // Draw another rectangle, rotated by 45 degrees, with a z-height of 10
        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
        // Draw a circle with its center at (400, 300) and a radius of 100, with a background of
        // green
        window.draw(&Circle::new((400, 300), 100), Col(Color::GREEN));
        // Draw a line with a thickness of 2 pixels, a red background,
        // and a z-height of 5
        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        // Draw a triangle with a red background, rotated by 45 degrees, and scaled down to half
        // its size
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        // We completed with no errors
        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    // Run with DrawGeometry as the event handler, with a window title of 'Draw Geometry' and a
    // size of (800, 600)
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", (800, 600)));
}
