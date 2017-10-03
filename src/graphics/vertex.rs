use super::super::geom::Vector;
use super::Color;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vector,
    pub tex_pos: Vector,
    pub col: Color
}
