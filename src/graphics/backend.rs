use graphics::Color;
use geom::Vector;

pub(crate) trait Backend {
    fn clear(&mut self, col: Color);
    fn display(&mut self);
    fn add(&mut self, texture: u32, vertices: &[Vertex], indices: &[u32]);
    fn add_vertex(&mut self, vertex: &Vertex);
    fn add_index(&mut self, index: u32);
    fn num_vertices(&self) -> usize;
    fn vertices(&self) -> &Vec<f32>;
    fn indices(&self) -> &Vec<u32>;
}

pub(crate) const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

#[derive(Clone, Copy)]
pub(crate) struct Vertex {
    pub pos: Vector,
    pub tex_pos: Vector,
    pub col: Color,
    pub use_texture: bool,
}
