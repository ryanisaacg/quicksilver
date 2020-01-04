use crate::geom::{Transform, Vector};
use crate::graphics::{Color, Image};

#[derive(Clone, Copy, Debug, PartialEq)]
/// A vertex for drawing items to the GPU
pub struct Vertex {
    /// The position of the vertex in space
    pub pos: Vector,
    /// If there is a texture attached to this vertex, where to get the texture data from
    ///
    /// It is normalized from 0 to 1
    pub uv: Option<Vector>,
    /// The color to blend this vertex with
    pub color: Color,
}

pub struct DrawGroup {
    pub elements: Vec<Element>,
    pub image: Option<Image>,
    pub transform: Transform,
}

#[derive(Clone)]
pub enum Element {
    Point(u32),
    Line([u32; 2]),
    Triangle([u32; 3]),
}
