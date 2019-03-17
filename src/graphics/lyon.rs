use crate::{
    geom::{Transform, Vector},
    graphics::{Background::Col, Color, GpuTriangle, Mesh, Vertex}
};
use lyon::tessellation::{
    geometry_builder::{Count, GeometryBuilder, GeometryBuilderError, VertexId},
    FillVertex, VertexConstructor
};

/// A way to render complex shapes using the lyon API
///
/// The ShapeRenderer has a color, transform, and z-ordering it applies to all
/// incoming shapes. It outputs the shapes to a mutable Mesh reference, which
/// can be a standalone mesh object or the one obtained by `window.mesh()`
pub struct ShapeRenderer<'a> {
    mesh: &'a mut Mesh,
    color: Color,
    z: f32,
    trans: Transform,
    dirty: Option<usize>
}

impl<'a> ShapeRenderer<'a> {
    /// Create a shape renderer with a target mesh and an initial color
    pub fn new(mesh: &'a mut Mesh, color: Color) -> ShapeRenderer<'a> {
        ShapeRenderer {
            mesh,
            color,
            z: 0.0,
            trans: Transform::IDENTITY,
            dirty: None
        }
    }

    /// Get the current color of the incoming shapes
    pub fn color(&self) -> Color {
        self.color
    }

    /// Set the color of the incoming shapes
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get the Z position of the incoming shapes
    pub fn z(&self) -> f32 {
        self.z
    }

    /// Set the Z position of the incoming shapes
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }

    /// Get the transformation that will be applied to all incoming shapes
    pub fn transform(&self) -> Transform {
        self.trans
    }

    /// Set the transformation that will be applied to all incoming shapes
    pub fn set_transform(&mut self, trans: Transform) {
        self.trans = trans;
    }
}

impl<'a, Input> GeometryBuilder<Input> for ShapeRenderer<'a>
        where Color: VertexConstructor<Input, Vertex> {
    fn begin_geometry(&mut self) {
        assert!(self.dirty.is_none());
        self.dirty = Some(self.mesh.triangles.len());
    }

    fn end_geometry(&mut self) -> Count {
        let dirty = self.dirty.expect("begin_geometry must be called before end_geometry");
        self.dirty = None;
        Count {
            vertices: self.mesh.vertices[dirty..].len() as u32,
            indices: self.mesh.triangles[dirty..].len() as u32 * 3
        }
    }

    fn add_vertex(&mut self, vertex: Input) -> Result<VertexId, GeometryBuilderError> {
        let mut vertex = self.color.new_vertex(vertex);
        vertex.pos = self.trans * vertex.pos;
        self.mesh.vertices.push(vertex);
        Ok(VertexId(self.mesh.vertices.len() as u32 - 1))
    }

    fn add_triangle(&mut self, a: VertexId, b: VertexId, c: VertexId) {
        let triangle = GpuTriangle::new(0, [a.0, b.0, c.0], self.z, Col(Color::WHITE));
        self.mesh.triangles.push(triangle);
    }

    fn abort_geometry(&mut self) {
        let dirty = self.dirty.expect("begin_geometry must be called before abort_geometry");
        self.dirty = None;
        self.mesh.vertices.truncate(dirty);
        self.mesh.triangles.truncate(dirty);
    }
}

impl VertexConstructor<FillVertex, Vertex> for Color {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let position = Vector::new(vertex.position.x, vertex.position.y);
        Vertex::new(position, None, Col(*self))
    }
}
