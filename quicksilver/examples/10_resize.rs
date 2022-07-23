// Example 10: Resize Handling
// Show the different ways of resizing the window
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Element, Graphics, Mesh, ResizeHandler, Vertex},
    run, Input, Result, Settings, Window,
};

const SIZE: Vector = Vector { x: 800.0, y: 600.0 };

fn main() {
    run(
        Settings {
            size: SIZE,
            title: "Resizing example",
            // By default, resizing is disabled: Here we need to enable it!
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // We'll use the triangle from the rgb_triangle example
    let vertices = {
        let top = Vertex {
            pos: Vector::new(400.0, 200.0),
            uv: None,
            color: Color::RED,
        };
        let left = Vertex {
            pos: Vector::new(200.0, 400.0),
            uv: None,
            color: Color::GREEN,
        };
        let right = Vertex {
            pos: Vector::new(600.0, 400.0),
            uv: None,
            color: Color::BLUE,
        };
        vec![top, left, right]
    };
    let elements = vec![Element::Triangle([0, 1, 2])];
    let mesh = Mesh {
        vertices,
        elements,
        image: None,
    };
    // Create a ResizeHandler that will Fit the content to the screen, leaving off area if we need
    // to. Here, we provide an aspect ratio of 4:3.
    // By default, a ResizeHandler::Fit is applied with the same aspect ratio as the initial size
    let resize_handler = ResizeHandler::Fit {
        aspect_width: 4.0,
        aspect_height: 3.0,
    };
    gfx.set_resize_handler(resize_handler);
    loop {
        while let Some(_) = input.next_event().await {}
        gfx.clear(Color::BLACK);
        // Fill the relevant part of the screen with white
        // This helps us determine what part of the screen is the black bars, and what is the
        // background. If we wanted white bars and a black background, we could simply clear to
        // Color::WHITE and fill a rectangle of Color::BLACK
        gfx.fill_rect(&Rectangle::new_sized(SIZE), Color::WHITE);
        // Draw the RGB triangle, which lets us see the squash and stress caused by resizing
        gfx.draw_mesh(&mesh);
        gfx.present(&window)?;
    }
}
