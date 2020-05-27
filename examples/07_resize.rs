// Example 7: Resize Handling
// Show the different ways of resizing the window
use quicksilver::{
    geom::{Rectangle, Transform, Vector},
    graphics::{Color, Element, Graphics, Mesh, ResizeHandler, Vertex},
    input::Event,
    Input, Window, Result, Settings, run,
};

const SIZE: Vector = Vector {
    x: 800.0,
    y: 600.0,
};

fn main() {
    run(
        Settings {
            size: SIZE,
            title: "RGB Triangle Example",
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Paint a triangle with red, green, and blue vertices, blending the colors for the pixels in-between
    // Define the 3 vertices and move them inside a Vec
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
    let projection = Transform::orthographic(Rectangle::new_sized(SIZE));
    loop {
        while let Some(ev) = input.next_event().await {
            if let Event::Resized(ev) = ev {
                let letterbox = resize_handler.projection(ev.size());
                gfx.set_projection(letterbox * projection);
            }
        }
        // Clear the screen to a blank, black color
        gfx.clear(Color::BLACK);
        // Fill the relevant part of the screen with white
        let rect = Rectangle::new_sized(SIZE);
        gfx.fill_rect(&rect, Color::WHITE);
        // Pass a reference to the mesh to the graphics object to draw
        gfx.draw_mesh(&mesh);
        // Send the data to be drawn
        gfx.present(&window)?;
    }
}
