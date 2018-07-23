// Draw the classic triangle to the screen
extern crate quicksilver;

use quicksilver::{
    run, Result, State,
    geom::Vector,
    graphics::{Color, GpuTriangle, RenderTarget, Vertex, Window, WindowBuilder}
};

struct RgbTriangle;

impl State for RgbTriangle {
    fn new() -> Result<RgbTriangle> {
        Ok(RgbTriangle)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let vertices = [
            Vertex::new_untextured((400, 200), Color::RED),
            Vertex::new_untextured((200, 400), Color::BLUE),
            Vertex::new_untextured((600, 400), Color::GREEN),
        ];
        let indices = [ GpuTriangle::new_untextured([0, 1, 2], 0.0) ];
        window.add_vertices(vertices.iter().cloned(), indices.iter().cloned());
        window.present()
    }
}

fn main() {
    run::<RgbTriangle>(WindowBuilder::new("RGB Triangle", (800, 600)));
}
