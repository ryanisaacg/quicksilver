// Draw the classic triangle to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex, Window, WindowBuilder},
    lifecycle::{State, run},
};

struct RgbTriangle {
    mesh: Mesh
}

impl State for RgbTriangle {
    fn new() -> Result<RgbTriangle> {
        let vertices = vec![
            Vertex::new((400, 200), None, Col(Color::RED)),
            Vertex::new((200, 400), None, Col(Color::BLUE)),
            Vertex::new((600, 400), None, Col(Color::GREEN))
        ];
        let triangles = vec![ GpuTriangle::new(0, [0, 1, 2], 0.0, Col(Color::WHITE)) ];
        let mesh = Mesh { vertices, triangles };
        Ok(RgbTriangle { mesh })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        window.mesh().apply(&self.mesh);
        window.present()
    }
}

fn main() {
    run::<RgbTriangle>(WindowBuilder::new("RGB Triangle", (800, 600)));
}
