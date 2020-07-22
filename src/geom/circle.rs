use crate::geom::{about_equal, Vector};
use std::cmp::{Eq, PartialEq};
#[cfg(feature = "saving")]
use serde::{Serialize,Deserialize};

#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "saving", derive(Serialize, Deserialize))]
///A circle with a center and a radius
pub struct Circle {
    /// The position of the center of the circle
    pub pos: Vector,
    /// The radius of the circle
    pub radius: f32,
}

impl Circle {
    /// Create a circle with the center as a vector
    pub fn new(center: Vector, radius: f32) -> Circle {
        Circle {
            pos: center,
            radius,
        }
    }
}

impl PartialEq for Circle {
    fn eq(&self, other: &Circle) -> bool {
        about_equal(self.pos.x, other.pos.x)
            && about_equal(self.pos.y, other.pos.y)
            && about_equal(self.radius, other.radius)
    }
}

impl Eq for Circle {}

#[cfg(test)]
mod tests {
    use crate::geom::*;

    #[test]
    fn construction() {
        let circ = Circle::new(Vector::new(0.0, 1.0), 2f32);
        assert_eq!(circ.pos.x, 0f32);
        assert_eq!(circ.pos.y, 1f32);
        assert_eq!(circ.radius, 2f32);
    }

    #[test]
    fn contains() {
        let circ = Circle::new(Vector::ZERO, 10.0);
        let vec1 = Vector::ZERO;
        let vec2 = Vector::new(11.0, 11.0);
        assert!(circ.contains(vec1));
        assert!(!circ.contains(vec2));
    }

    #[test]
    fn overlap() {
        let a = &Circle::new(Vector::ZERO, 16.0);
        let b = &Circle::new(Vector::new(5.0, 5.0), 4.0);
        let c = &Circle::new(Vector::new(50.0, 50.0), 5.0);
        let d = &Rectangle::new(Vector::new(10.0, 10.0), Vector::new(10.0, 10.0));
        assert!(a.overlaps_circle(b));
        assert!(!a.overlaps_circle(c));
        assert!(a.overlaps_rectangle(d));
        assert!(!c.overlaps_rectangle(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = &Circle::new(Vector::new(0.0, 0.0), 5.0);
        let rec1 = &Rectangle::new_sized(Vector::new(2.0, 2.0));
        let rec2 = &Rectangle::new(Vector::new(5.0, 5.0), Vector::new(4.0, 4.0));
        assert!(circ.overlaps_rectangle(rec1));
        assert!(rec1.overlaps_circle(circ));
        assert!(!circ.overlaps_rectangle(rec2));
        assert!(!rec2.overlaps_circle(circ));
    }

    #[test]
    fn translate() {
        let circ = Circle::new(Vector::new(0.0, 0.0), 16.0);
        let translate = Vector::new(4.0, 4.0);
        assert_eq!(
            circ.center() + translate,
            circ.translate(translate).center()
        );
    }
}
