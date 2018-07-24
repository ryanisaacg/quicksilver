use graphics::{DrawAttributes, Drawable, GpuTriangle, RenderTarget, Vertex};

/// A way to store rendered objects without having to re-process them
pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) triangles: Vec<GpuTriangle>
}

impl Mesh {
    /// Create a new, empty mesh
    ///
    /// This allocates, so hold on to meshes rather than creating and destroying them
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            triangles: Vec::new()
        }
    }

    /// Clear the mesh, removing anything that has been drawn to it
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangles.clear();
    }
}

impl RenderTarget for Mesh {
    fn add_vertices(&mut self, vertices: impl IntoIterator<Item = Vertex>, triangles: impl IntoIterator<Item = GpuTriangle>) {
        let offset = self.vertices.len() as u32;
        self.triangles.extend(triangles.into_iter().map(|t| GpuTriangle {
            indices: [
                t.indices[0] + offset,
                t.indices[1] + offset,
                t.indices[2] + offset,
            ],
            ..t
        }));
        self.vertices.extend(vertices.into_iter().map(|v| Vertex {
            pos: v.pos,
            ..v
        }));
    }
}

impl Drawable for Mesh {
    fn draw(&self, target: &mut impl RenderTarget, attr: DrawAttributes) {
        let vertices = self.vertices.iter().map(|vertex| Vertex {
            pos: attr.transform * vertex.pos,
            tex_pos: vertex.tex_pos,
            col: vertex.col.multiply(attr.color)
        });
        let triangles = self.triangles.iter().map(|triangle| GpuTriangle {
            z: triangle.z + attr.z,
            ..triangle.clone()
        });
        target.add_vertices(vertices, triangles);
    }
}
