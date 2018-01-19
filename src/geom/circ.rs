use super::{about_equal, Line, Rectangle, Vector};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A circle with a center and a radius
pub struct Circle {
    /// The x coordinate of the center
    pub x: f32,
    /// The y coordinate of the center
    pub y: f32,
    /// The radius of the circle
    pub radius: f32,
}

impl Circle {
    ///Create a new circle with the given dimensions
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }

    ///Create a new circle with integer dimensions
    pub fn newi(x: i32, y: i32, radius: i32) -> Circle {
        Circle::new(x as f32, y as f32, radius as f32)
    }

    ///Create a circle with the center as a vector
    pub fn newv(position: Vector, radius: f32) -> Circle {
        Circle::new(position.x, position.y, radius)
    }

    ///Get the center of a circle as a vector
    pub fn center(self) -> Vector {
        Vector {
            x: self.x,
            y: self.y,
        }
    }

    ///Check to see if a circle contains a point
    pub fn contains(self, v: Vector) -> bool {
        (v - self.center()).len2() < self.radius.powi(2)
    }

    ///Check to see if a circle intersects a line
    pub fn intersects(self, l: Line) -> bool {
        let line_direction = (l.end - l.start).normalize();
        //Check if the circle contains the closest point
        //The dot product of the distance to the start and the direction yields
        //the normalized distance along the line
        self.contains(match (self.center() - l.start).dot(line_direction) {
            x if x <= 0f32 => l.start,
            x if x >= 1f32 => l.end,
            x => l.start + line_direction * x
        })
    }

    ///Check if a circle overlaps a rectangle
    pub fn overlaps_rect(self, r: Rectangle) -> bool {
        r.overlaps_circ(self)
    }

    ///Check if two circles overlap
    pub fn overlaps_circ(self, c: Circle) -> bool {
        (self.center() - c.center()).len2() < (self.radius + c.radius).powi(2)
    }

    ///Translate a circle by a given vector
    pub fn translate(self, v: Vector) -> Circle {
        Circle::new(self.x + v.x, self.y + v.y, self.radius)
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        about_equal(self.x, other.x) && about_equal(self.y, other.y) && about_equal(self.radius, other.radius)
    }
}

impl Eq for Circle {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let circ = Circle::new(0f32, 1f32, 2f32);
        assert_eq!(circ.x, 0f32);
        assert_eq!(circ.y, 1f32);
        assert_eq!(circ.radius, 2f32);
    }

    #[test]
    fn contains() {
        let circ = Circle::newi(0, 0, 10);
        let vec1 = Vector::newi(0, 0);
        let vec2 = Vector::newi(11, 11);
        assert!(circ.contains(vec1));
        assert!(!circ.contains(vec2));
    }

    #[test]
    fn overlap() {
        let a = Circle::newi(0, 0, 16);
        let b = Circle::newi(5, 5, 4);
        let c = Circle::newi(50, 50, 5);
        let d = Rectangle::newi(10, 10, 10, 10);
        assert!(a.overlaps_circ(b));
        assert!(!a.overlaps_circ(c));
        assert!(a.overlaps_rect(d));
        assert!(!c.overlaps_rect(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = Circle::newi(0, 0, 5);
        let rec1 = Rectangle::newi_sized(2, 2);
        let rec2 = Rectangle::newi(5, 5, 4, 4);
        assert!(circ.overlaps_rect(rec1));
        assert!(rec1.overlaps_circ(circ));
        assert!(!circ.overlaps_rect(rec2));
        assert!(!rec2.overlaps_circ(circ));
    }

    #[test]
    fn intersects() {
        let line1 = Line::new(Vector::newi(0, 0), Vector::newi(32, 32));
        let line2 = Line::new(Vector::newi(0, 32), Vector::newi(32, 0));
        let line3 = Line::new(Vector::newi(32, 32), Vector::newi(64, 64));
        let line4 = Line::new(Vector::newi(100, 100), Vector::newi(1000, 1000));
        //TODO: fix this test
//        let line5 = Line::new(Vector::newi(-100, 32), Vector::newi(100, 32));
        let circ = Circle::newi(0, 0, 33);
        assert!(circ.intersects(line1));
        assert!(circ.intersects(line2));
        assert!(!circ.intersects(line3));
        assert!(!circ.intersects(line4));
//        assert!(circ.intersects(line5));
    }

    #[test]
    fn translate() {
        let circ = Circle::newi(0, 0, 16);
        let translate = Vector::newi(4, 4);
        assert_eq!(circ.center() + translate, circ.translate(translate).center());
    }

}
