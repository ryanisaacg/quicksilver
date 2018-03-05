use geom::{Circle, Line, Rectangle, Vector};

/// A trait that indicates something exists in space
pub trait Bounded {
    /// Its center as a vector
    fn center(&self) -> Vector;
    /// Create a new copy of this shape with the given center
    fn with_center(&self, center: Vector) -> Self where Self: Sized;
    /// Check if a point is contained by this shape
    fn contains(&self, vec: Vector) -> bool;
    /// Check if a line is intersected by this shape
    fn intersects(&self, line: Line) -> bool;
    /// Check if a circle is overlapped by this shape
    fn overlaps_circ(&self, circ: Circle) -> bool;
    /// Check if a rect is overlapped by this shape
    fn overlaps_rect(&self, rect: Rectangle) -> bool;
    /// Check if this shapes overlaps with another
    fn overlaps(&self, other: &Bounded) -> bool;
    /// Move the shape to be within the region, if possible
    fn constrain(&self, bounds: Rectangle) -> Self where Self: Sized;
    /// The smallest possible rectangle that fully contains the shape
    fn bounding_box(&self) -> Rectangle;

    /// Translate a shape by a given vector
    fn translate(&self, amount: Vector) -> Self where Self: Sized {
        self.with_center(self.center() + amount)
    }
}