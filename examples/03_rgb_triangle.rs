// Example 3: The RGB Triangle
// Open a window, and draw the standard GPU triangle
use quicksilver::{
    geom::Vector,
    graphics::{Color, Element, Mesh, Vertex},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "RGB Triangle Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, black color
    gfx.clear(Color::BLACK);
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
    // Pass a reference to the mesh to the graphics object to draw
    gfx.draw_mesh(&mesh);
    // Send the data to be drawn
    gfx.present(&window)?;
    loop {
        while let Some(_) = input.next_event().await {}
    }
}
