use geom::{Point, Line, Vector, Rectangle};
use graphics::Window;

/// Any polygon
pub trait Polygon {
    ///Check if this polygon overlaps with another
    fn overlaps(&self, polygon: impl Polygon) -> bool;

    ///Check if this polygon contains a point
    fn contains(&self, point: Point) -> bool;

    ///Get all vertices/corners as a growable array
    fn get_vertices(&self) -> Vec<Point>;

    ///Get all edges/sides as a growable array
    fn get_edges(&self) -> Vec<Line>;

    ///Translate this polygon by the given vector
    fn translate(&self, v: Vector) -> Self where Self: Sized;

    ///Center this polygon on the given point
    fn with_center(&self, c: Point) -> Self where Self: Sized {
        self.translate(c - self.center())
    }

    ///Get the center of this polygon
    fn center(&self) -> Point;

    ///Create a box around this polygon
    fn bounding_box(&self) -> Rectangle;
}
