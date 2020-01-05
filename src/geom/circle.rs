use crate::geom::{about_equal, Scalar, Vector};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug)]
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
            pos: center.into(),
            radius: radius.float(),
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
        let a = &Circle::new((0, 0), 16);
        let b = &Circle::new((5, 5), 4);
        let c = &Circle::new((50, 50), 5);
        let d = &Rectangle::new((10, 10), (10, 10));
        assert!(a.overlaps(b));
        assert!(!a.overlaps(c));
        assert!(a.overlaps(d));
        assert!(!c.overlaps(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = &Circle::new((0, 0), 5);
        let rec1 = &Rectangle::new_sized((2, 2));
        let rec2 = &Rectangle::new((5, 5), (4, 4));
        assert!(circ.overlaps(rec1));
        assert!(rec1.overlaps(circ));
        assert!(!circ.overlaps(rec2));
        assert!(!rec2.overlaps(circ));
    }

    #[test]
    fn translate() {
        let circ = Circle::new((0, 0), 16);
        let translate = Vector::new(4, 4);
        assert_eq!(
            circ.center() + translate,
            circ.translate(translate).center()
        );
    }
}
