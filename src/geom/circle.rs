#[cfg(feature="ncollide2d")] use ncollide2d::shape::Ball;
use geom::{about_equal, Positioned, Rectangle, Scalar, Vector};
use std::{
    cmp::{Eq, PartialEq},
};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A circle with a center and a radius
pub struct Circle {
    /// The position of the center of the circle
    pub pos: Vector,
    /// The radius of the circle
    pub radius: f32,
}

impl Circle {
    /// Create a circle with the center as a vector
    pub fn new(center: impl Into<Vector>, radius: impl Scalar) -> Circle {
        Circle {
            pos:    center.into(),
            radius: radius.float()
        }
    }

    ///Construct a circle from a center and a Ball
    #[cfg(feature="ncollide2d")]
    pub fn from_ball(center: impl Into<Vector>, ball: Ball<f32>) -> Circle {
        Circle::new(center.into(), ball.radius())
    }

    ///Convert the circle into an ncollide Ball
    #[cfg(feature="ncollide2d")]
    pub fn into_ball(self) -> Ball<f32> {
        Ball::new(self.radius)
    }

    /// Check to see if a circle contains a point
    pub fn contains(self, v: impl Into<Vector>) -> bool {
        (v.into() - self.center()).len2() < self.radius.powi(2)
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
    #[must_use]
    pub fn translate(self, v: impl Into<Vector>) -> Circle {
        Circle::new(self.pos + v.into(), self.radius)
    }

    ///Move a circle so it is entirely contained within a Rectangle
    #[must_use]
    pub fn constrain(self, outer: Rectangle) -> Circle {
        Circle::new(Rectangle::new((self.pos.x - self.radius, self.pos.y - self.radius), (self.radius * 2.0, self.radius * 2.0)).constrain(outer).center(), self.radius)
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        return about_equal(self.pos.x, other.pos.x)
            && about_equal(self.pos.y, other.pos.y)
            && about_equal(self.radius, other.radius)
    }
}

impl Eq for Circle {}

impl Positioned for Circle {
    
    fn center(&self) -> Vector {
        self.pos
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            self.pos - Vector::ONE * self.radius, 
            Vector::ONE * self.radius * 2
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        let circ = Circle::new((0f32, 1f32), 2f32);
        assert_eq!(circ.pos.x, 0f32);
        assert_eq!(circ.pos.y, 1f32);
        assert_eq!(circ.radius, 2f32);
    }

    #[test]
    fn contains() {
        let circ = Circle::new((0, 0), 10);
        let vec1 = Vector::new(0, 0);
        let vec2 = Vector::new(11, 11);
        assert!(circ.contains(vec1));
        assert!(!circ.contains(vec2));
    }

    #[test]
    fn overlap() {
        let a = Circle::new((0, 0), 16);
        let b = Circle::new((5, 5), 4);
        let c = Circle::new((50, 50), 5);
        let d = Rectangle::new((10, 10), (10, 10));
        assert!(a.overlaps_circ(b));
        assert!(!a.overlaps_circ(c));
        assert!(a.overlaps_rect(d));
        assert!(!c.overlaps_rect(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = Circle::new((0, 0), 5);
        let rec1 = Rectangle::new_sized((2, 2));
        let rec2 = Rectangle::new((5, 5), (4, 4));
        assert!(circ.overlaps_rect(rec1));
        assert!(rec1.overlaps_circ(circ));
        assert!(!circ.overlaps_rect(rec2));
        assert!(!rec2.overlaps_circ(circ));
    }

    #[test]
    fn translate() {
        let circ = Circle::new((0, 0), 16);
        let translate = Vector::new(4, 4);
        assert_eq!(circ.center() + translate, circ.translate(translate).center());
    }

}
