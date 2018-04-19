#[cfg(feature="ncollide")] use ncollide::shape::Ball;
use geom::{about_equal, Line, Positioned, Rectangle, Scalar, Vector};
use rand::{Rand, Rng};
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
    pub fn new<T: Scalar>(x: T, y: T, radius: T) -> Circle {
        Circle {
            x: x.float(),
            y: y.float(),
            radius: radius.float(),
        }
    }

    ///Create a circle with the center as a vector
    pub fn newv<T: Scalar>(position: Vector, radius: T) -> Circle {
        Circle {
            x: position.x,
            y: position.y,
            radius: radius.float()
        }
    }

    ///Construct a circle from a center and a Ball
    #[cfg(feature="ncollide")]
    pub fn from_ball(position: Vector, ball: Ball<f32>) -> Circle {
        Circle::newv(position, ball.radius())
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

    ///Move a circle so it is entirely contained within a Rectangle
    pub fn constrain(self, outer: Rectangle) -> Circle {
        Circle::newv(Rectangle::new(self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0).constrain(outer).center(), self.radius)
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        about_equal(self.x, other.x) && about_equal(self.y, other.y) && about_equal(self.radius, other.radius)
    }
}

impl Eq for Circle {}

impl Positioned for Circle {
    fn center(&self) -> Vector {
        Vector::new(self.x, self.y)
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.x - self.radius, self.y - self.radius, self.radius * 2.0, self.radius * 2.0)
    }
}

#[cfg(feature="ncollide")]
impl Into<Ball<f32>> for Circle {
    fn into(self) -> Ball<f32> {
        Ball::new(self.radius)
    }
}

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
        let circ = Circle::new(0, 0, 10);
        let vec1 = Vector::new(0, 0);
        let vec2 = Vector::new(11, 11);
        assert!(circ.contains(vec1));
        assert!(!circ.contains(vec2));
    }

    #[test]
    fn overlap() {
        let a = Circle::new(0, 0, 16);
        let b = Circle::new(5, 5, 4);
        let c = Circle::new(50, 50, 5);
        let d = Rectangle::new(10, 10, 10, 10);
        assert!(a.overlaps_circ(b));
        assert!(!a.overlaps_circ(c));
        assert!(a.overlaps_rect(d));
        assert!(!c.overlaps_rect(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = Circle::new(0, 0, 5);
        let rec1 = Rectangle::new_sized(2, 2);
        let rec2 = Rectangle::new(5, 5, 4, 4);
        assert!(circ.overlaps_rect(rec1));
        assert!(rec1.overlaps_circ(circ));
        assert!(!circ.overlaps_rect(rec2));
        assert!(!rec2.overlaps_circ(circ));
    }

    #[test]
    fn intersects() {
        let line1 = Line::new(Vector::new(0, 0), Vector::new(32, 32));
        let line2 = Line::new(Vector::new(0, 32), Vector::new(32, 0));
        let line3 = Line::new(Vector::new(32, 32), Vector::new(64, 64));
        let line4 = Line::new(Vector::new(100, 100), Vector::new(1000, 1000));
        //TODO: fix this test
//        let line5 = Line::new(Vector::new(-100, 32), Vector::new(100, 32));
        let circ = Circle::new(0, 0, 33);
        assert!(circ.intersects(line1));
        assert!(circ.intersects(line2));
        assert!(!circ.intersects(line3));
        assert!(!circ.intersects(line4));
//        assert!(circ.intersects(line5));
    }

    #[test]
    fn translate() {
        let circ = Circle::new(0, 0, 16);
        let translate = Vector::new(4, 4);
        assert_eq!(circ.center() + translate, circ.translate(translate).center());
    }

}

impl Rand for Circle {
    fn rand<R: Rng>(rand: &mut R) -> Self {
        Circle::newv(rand.gen(), rand.gen::<f32>())
    }
}
