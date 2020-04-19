// Example 7: Resize Handling
// Show the different ways of resizing the window
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Element, Graphics, Mesh, ResizeHandler, Vertex},
    lifecycle::{run, Event, EventStream, Settings, Window},
    Result,
};

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "RGB Triangle Example",
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Paint a triangle with red, green, and blue vertices, blending the colors for the pixels in-between
    // Define the 3 vertices and move them inside a Vec
    let vertices = {
        let top = Vertex {
            pos: Vector::new(400, 200),
            uv: None,
            color: Color::RED,
        };
        let left = Vertex {
            pos: Vector::new(200, 400),
            uv: None,
            color: Color::GREEN,
        };
        let right = Vertex {
            pos: Vector::new(600, 400),
            uv: None,
            color: Color::BLUE,
        };
        vec![top, left, right]
    };
    // A triangle is simply a pointer to indices of the vertices
    let elements = vec![Element::Triangle([0, 1, 2])];
    // Bring the vertices and the triangle elements together to define a mesh
    let mesh = Mesh {
        vertices,
        elements,
        image: None,
    };
    let resize_handler = ResizeHandler::Fit {
        aspect_width: 4.0,
        aspect_height: 3.0,
    };
    loop {
        while let Some(ev) = events.next_event().await {
            if let Event::Resized(ev) = ev {
                gfx.set_projection(resize_handler.projection(ev.logical_size()));
            }
        }
        // Clear the screen to a blank, black color
        gfx.clear(Color::BLACK);
        // Fill the relevant part of the screen with white
        let rect = Rectangle::new_sized(Vector::new(800.0, 600.0));
        gfx.fill_rect(&rect, Color::WHITE);
        // Pass a reference to the mesh to the graphics object to draw
        gfx.draw_mesh(&mesh);
        // Send the data to be drawn
        gfx.present(&window)?;
    }
}
