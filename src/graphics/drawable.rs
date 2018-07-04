use graphics::{Vertex, GpuTriangle, Sprite};
use geom::Transform;

/// Some object that can be drawn to the screen
pub trait Drawable {
    /// Get all vertices that make up this `Drawable`
    fn get_vertices(&self) -> Vec<Vertex>;
    /// Get all triangles that make up this `Drawable`
    fn get_triangles(&self) -> Vec<GpuTriangle>;
    /// Create a default sprite from this `Drawable`
    fn to_sprite(&self) -> Sprite;
}