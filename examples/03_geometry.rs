// Example 3: Geometry
// Draw a fun variety of shapes
use mint::Vector2;
use quicksilver::{
    Result,
    graphics::{Color, Graphics, Image},
    lifecycle::{EventStream, Settings, Window, run},
    traits::*,
};

fn main() {
    run(Settings {
        size: Vector2 { x: 800.0, y: 600.0 },
        title: "Image Example",
        ..Settings::default()
    }, app);
}

// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let image = Image::load(&gfx, "static/image.png").await?;

    loop {
        while let Some(_) = events.next().await {}
        // Remove any lingering artifacts from the previous frame
        gfx.clear(Color::WHITE)?;
        //Draw a rectangle with a top-left corner at (100, 100) and a width and height of 32 with a red background, at a 45 degree angle
        gfx.fill_shape_transform(
            Shape::Rect {
                min: Vector2 { x: 10.0, y: 100.0 },
                max: Vector2 { x: 32.0, y: 32.0 },
                color: Color::BLUE,
            },
            Transform::rotate(45.0),
            ShapeDraw { angle: 45.0, ..ShapeDraw::default() }
        );
        // Draw a circle with its center at (400, 300) and a radius of 100, with a background of
        // green
        gfx.fill_shape(Shape::Circ {
            center: Vector2 { x: 400.0, y: 300.0 },
            radius: 100.0,
            color: Color::GREEN,
        });
        // Draw a line with a thickness of 2 pixels and a red background
        gfx.fill_shape(Shape::Line {
            start: Vector2 { x: 50.0, y: 80.0 },
            end: Vector2 { x: 600.0, y: 450.0 },
            thickness: 2.0,
            color: Color::RED,
        });
        // Draw a triangle with a gradient background, rotated by 45 degrees, and scaled down to half
        // its size
        gfx.fill_shape_transform(Shape::Polygon(&[
               (Vector2 { x: 500.0, y: 50.0 }, Color::RED),
               (Vector2 { x: 450.0, y: 100.0 }, Color::BLUE),
               (Vector2 { x: 650.0, y: 150.0 }, Color::GREEN),
            ]),
            transform: Transform::rotate(45) * Transform::scale((0.5, 0.5)),
        });
        gfx.present(&window);
    }
}

