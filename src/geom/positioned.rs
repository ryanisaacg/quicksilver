use geom::{Rectangle, Vector};

/// A trait that indicates something exists in space
pub trait Positioned {
    /// Its center as a vector
    fn center(&self) -> Vector;
    /// The smallest possible rectangle that fully contains the shape
    fn bounding_box(&self) -> Rectangle;
}