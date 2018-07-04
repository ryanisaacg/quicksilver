use geom::{about_equal, Circle, Positioned, Scalar, Point, Polygon, Rectangle, Vector};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A line with two points
pub struct Line {
    ///The first point of the line
    pub p1: Point,
    ///The second point of the line
    pub p2: Point,
}

impl Line {
    ///Create a new line
    pub fn new<T: Scalar>(x1: T, y1: T, x2: T, y2: T) -> Self {
        Self::newv(Point::new(x1, y1), Point::new(x2, y2))
    }

    ///Create a new line from vectors
    pub fn newv(p1: Point, p2: Point) -> Self {
        Line {
            p1,
            p2
        }
    }

    ///Create a line at the origin with a given length on the x axis
    pub fn new_sized<T: Scalar>(length: T) -> Self {
        Self::newv_sized(Point::new(length.float(), 0.0))
    }

    ///Create a line starting at the origin and ending at the given point
    pub fn newv_sized(end: Point) -> Self {
        Self::newv(Point::zero(), end)
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        about_equal(self.p1.x, other.p1.x) && about_equal(self.p1.y, other.p1.y)
            && about_equal(self.p2.x, other.p2.x) && about_equal(self.p2.y, other.p2.y)
    }
}

impl Eq for Line {}

impl Polygon for Line {
    fn overlaps(&self, polygon: impl Polygon) -> bool {
        let vertices = polygon.get_vertices();
        // check if any point is in the polygon
        let edges = polygon.get_edges();
        // check if the line crosses any edge
        unimplemented!()
    }

    fn contains(&self, point: Point) -> bool {
        unimplemented!()
    }

    fn get_vertices(&self) -> Vec<Vector> {
        unimplemented!()
    }

    fn get_edges(&self) -> Vec<Line> {
        unimplemented!()
    }

    fn translate(&self, v: Vector) -> Self where Self: Sized {
        unimplemented!()
    }

    fn with_center(&self, c: Vector) -> Self where Self: Sized {
        unimplemented!()
    }

    fn center(&self) -> Vector {
        unimplemented!()
    }

    fn bounding_box(&self) -> Rectangle {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

}
