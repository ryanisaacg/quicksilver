extern crate gl;

use gl::types::*;
use graphics::Color;
use geom::Vector;

pub trait Backend: Send {
    fn clear(&mut self, col: Color);
    fn display(&mut self);
    fn add(&mut self, texture: GLuint, vertices: &[Vertex], indices: &[GLuint]);
    fn add_vertex(&mut self, vertex: &Vertex);
    fn add_index(&mut self, index: GLuint);
    fn num_vertices(&self) -> usize;
}

pub const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vector,
    pub tex_pos: Vector,
    pub col: Color,
    pub use_texture: bool,
}
