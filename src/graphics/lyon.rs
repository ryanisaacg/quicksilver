use geom::Vector;
use graphics::{Color, GpuTriangle, Vertex, Window};
use lyon::tessellation::{
    geometry_builder::{Count, GeometryBuilder, VertexId},
    FillVertex, VertexConstructor
};

pub struct ShapeRenderer {
    vertices: Vec<Vertex>,
    indices: Vec<GpuTriangle>,
    color: Color,
    z: f32
}

impl ShapeRenderer {
    pub fn new() -> ShapeRenderer {
        ShapeRenderer {
            vertices: Vec::new(),
            indices: Vec::new(),
            color: Color::WHITE,
            z: 0.0
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    pub fn draw(&self, window: &mut Window) {
        window.add_vertices(self.vertices.iter().cloned(), self.indices.iter().cloned());
    }
}

impl<Input> GeometryBuilder<Input> for ShapeRenderer 
        where Color: VertexConstructor<Input, Vertex> {
    fn begin_geometry(&mut self) {
        self.vertices.clear();
        self.indices.clear();
        self.color = Color::WHITE;
        self.z = 0.0;
    }

    fn end_geometry(&mut self) -> Count {
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
