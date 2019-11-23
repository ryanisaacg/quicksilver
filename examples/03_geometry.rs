// Draw an image to the screen
use mint::Vector2;
use quicksilver::{
    QuicksilverError,
    graphics::{Color, Context, Shape, ShapeDraw, Transform},
    lifecycle::{Event, EventStream, Settings, Window, run},
};

async fn app(window: Window, mut events: EventStream) -> Result<(), QuicksilverError> {
    while let Some(_) = events.next().await {
        let paint = Paint::new(&mut window);
        // Remove any lingering artifacts from the previous frame
        paint.clear(Color::WHITE)?;
        //Draw a rectangle with a top-left corner at (100, 100) and a width and height of 32 with a red background, at a 45 degree angle
        paint.fill_shape_transform(
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
        paint.fill_shape(Shape::Circ {
            center: Vector2 { x: 400.0, y: 300.0 },
            radius: 100.0,
            color: Color::GREEN,
        });
        // Draw a line with a thickness of 2 pixels and a red background
        paint.fill_shape(Shape::Line {
            start: Vector2 { x: 50.0, y: 80.0 },
            end: Vector2 { x: 600.0, y: 450.0 },
            thickness: 2.0,
            color: Color::RED,
        });
        // Draw a triangle with a gradient background, rotated by 45 degrees, and scaled down to half
        // its size
        paint.fill_shape_transform(Shape::Polygon(&[
               (Vector2 { x: 500.0, y: 50.0 }, Color::RED),
               (Vector2 { x: 450.0, y: 100.0 }, Color::BLUE),
               (Vector2 { x: 650.0, y: 150.0 }, Color::GREEN),
            ]),
            transform: Transform::rotate(45) * Transform::scale((0.5, 0.5)),
        });
    }
}

fn main() {
    run(app, Settings {
        size: Vector2 { x: 800, 600 },
        title: "Image Example",
        icon_path: Some("image.png"),
        ..Settings::default()
    });
}
