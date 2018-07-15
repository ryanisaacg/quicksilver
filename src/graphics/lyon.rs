use geom::Vector;
use graphics::{Color, DrawAttributes, Drawable, GpuTriangle, Vertex, Window};
use lyon::tessellation::{
    geometry_builder::{Count, GeometryBuilder, VertexId},
    FillVertex, VertexConstructor
};

/// A way to render complex shapes using the lyon API
pub struct ShapeRenderer {
    vertices: Vec<Vertex>,
    indices: Vec<GpuTriangle>,
    color: Color,
    z: f32
}

impl ShapeRenderer {
    /// Create a shape renderer with an initial color
    pub fn new(color: Color) -> ShapeRenderer {
        ShapeRenderer {
            vertices: Vec::new(),
            indices: Vec::new(),
            color,
            z: 0.0
        }
    }

    /// Get the current color of the renderer
    pub fn color(&self) -> Color {
        self.color
    }

    /// Set the color of the renderer
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the Z position of the shapes
    pub fn z(&self) -> f32 {
        self.z
    }

    /// Set the Z position of the shapes
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }
}

impl<Input> GeometryBuilder<Input> for ShapeRenderer 
        where Color: VertexConstructor<Input, Vertex> {
    fn begin_geometry(&mut self) {
        println!("pls");
        //TODO: don't require a drawn between multiple begin
        self.vertices.clear();
        self.indices.clear();
    }

    fn end_geometry(&mut self) -> Count {
        println!("{:?}", self.vertices);
        Count {
            vertices: self.vertices.len() as u32,
            indices: self.indices.len() as u32 * 3
        }
    }

    fn add_vertex(&mut self, vertex: Input) -> VertexId {
        self.vertices.push(self.color.new_vertex(vertex));
        VertexId(self.vertices.len() as u32)
    }

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        self.indices.push(GpuTriangle::new_untextured([a.0, b.0, c.0], self.z));
    }

    fn abort_geometry(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

impl VertexConstructor<FillVertex, Vertex> for Color {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let position = Vector::new(vertex.position.x, vertex.position.y);
        Vertex::new_untextured(position, *self)
    }
}

// TODO: document the peculiarites of the draw attributes

impl Drawable for ShapeRenderer {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        let vertices = self.vertices
            .iter()
            .map(|vertex| Vertex::new_untextured(
                params.transform * vertex.pos,
                vertex.col.blend(params.color)
            ));
        let indices = self.indices
            .iter()
            .map(|triangle| GpuTriangle::new_untextured(
                triangle.indices,
                triangle.z + params.z
            ));
        window.add_vertices(vertices, indices);
    }
}