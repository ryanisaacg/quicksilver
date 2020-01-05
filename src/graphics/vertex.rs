use crate::geom::Vector;
use crate::graphics::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
/// A vertex for drawing items to the GPU
pub struct Vertex {
    /// The position of the vertex in 2D space
    pub pos: Vector,
    /// If there is a texture attached to this vertex, where to get the texture data from
    ///
    /// It is normalized from 0 to 1
    pub uv: Option<Vector>,
    /// The color to blend this vertex with
    pub color: Color,
}

/// A shape to draw, using uploaded [`Vertex`] values
#[derive(Clone)]
pub enum Element {
    /// A single point, with the given vertex index
    Point(u32),
    /// A line 1 pixel thick, with the two given vertices
    Line([u32; 2]),
    /// A filled triangle, with the 3 given indices
    Triangle([u32; 3]),
}
