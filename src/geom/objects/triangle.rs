use crate::geom::Vector;
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A triangle with three points
pub struct Triangle {
    ///The first point
    pub a: Vector,
    ///The second point
    pub b: Vector,
    ///The third point
    pub c: Vector,
}

impl Triangle {
    ///Create a triangle from `Vector`s of all three points
    pub fn new(a: impl Into<Vector>, b: impl Into<Vector>, c: impl Into<Vector>) -> Triangle {
        Triangle { 
            a: a.into(),
            b: b.into(),
            c: c.into()
        }
    }
    ///Calculate the area of the triangle
    pub fn area(self) -> f32 {
        // Heron's Formula
        ((self.b.x - self.a.x) * (self.c.y - self.a.y) - (self.c.x - self.a.x) * (self.b.y - self.a.y)).abs() / 2.0
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Triangle) -> bool {
        (self.a == other.a || self.a == other.b || self.a == other.c)
            && (self.b == other.a || self.b == other.b || self.b == other.c)
            && (self.c == other.a || self.c == other.b || self.c == other.c)
    }
}

impl Eq for Triangle {}

#[cfg(test)]
mod tests {
    use geom::*;

    #[test]
    fn overlap_rectangle() {
        let rect = &Rectangle::new_sized((1, 1));
        let t_inside = Triangle::new((0.25, 0.25), (0.75, 0.25), (0.25, 0.75));
        let t_over = Triangle::new((0.5, -0.5), (0.5, 1.5), (1.5, 0.5));
        let t_outside = Triangle::new((2, 3), (4, 5), (10, 12));
        assert!(t_inside.overlaps(rect));
        assert!(t_over.overlaps(rect));
        assert!(!t_outside.overlaps(rect));
    }

    #[test]
    fn overlap_circle() {
        let circle = &Circle::new((0, 0), 1);
        let t_inside = Triangle::new((-0.5, -0.5), (0.5, -0.5), (0.0, 0.5));
        let t_over = Triangle::new((0, -2), (0, 2), (2, 0));
        let t_outside = Triangle::new((2, 3), (4, 5), (10, 12));
        assert!(t_inside.overlaps(circle));
        assert!(t_over.overlaps(circle));
        assert!(!t_outside.overlaps(circle));
    }

    #[test]
    fn overlap_line() {
        let line = &Line::new(Vector::ZERO, Vector::X);
        let t_on = Triangle::new((0, 0), (1, 1), (0, 1));
        let t_over = Triangle::new((0.25, -0.5), (0.75, -0.5), (0.5, 0.5));
        let t_outside = Triangle::new((2, 3), (4, 5), (10, 12));
        assert!(t_on.overlaps(line));
        assert!(t_over.overlaps(line));
        assert!(!t_outside.overlaps(line));
    }

    #[test]
    fn contains() {
        let triangle = Triangle::new((0, 0), (1, 0), (0, 1));
        let p_in = Vector::new(0.25, 0.25);
        let p_on = Vector::new(0.5, 0.5);
        let p_off = Vector::new(1, 1);
        assert!(triangle.contains(p_in));
        assert!(triangle.contains(p_on));
        assert!(!triangle.contains(p_off));
    }

    #[test]
    fn constraint() {
        let tri = Triangle::new((5, 5), (10, 7), (8, 8));
        let fits = Rectangle::new((0, 0), (15, 15));
        let not_fit = Rectangle::new((0, 0), (9, 6));
        let fits_tri = tri.constrain(&fits);
        let not_fits_tri = tri.constrain(&not_fit);
        assert_eq!(tri.a, fits_tri.a);
        assert_eq!(tri.b, fits_tri.b);
        assert_eq!(tri.c, fits_tri.c);
        assert_eq!(Vector::new(4, 3), not_fits_tri.a);
        assert_eq!(Vector::new(9, 5), not_fits_tri.b);
        assert_eq!(Vector::new(7, 6), not_fits_tri.c);
    }

    #[test]
    fn translate() {
        let triangle = Triangle::new((0, 0), (1, 0), (0, 1)).translate(Vector::ONE);
        assert_eq!(triangle, Triangle::new((1, 1), (2, 1), (1, 2)));
    }

    #[test]
    fn area() {
        let triangle = Triangle::new((0, 0), (1, 0), (0, 1));
        let triangle2 = Triangle::new((-4, -1), (-1, -1), (1, 4));
        assert_eq!(triangle.area(), 0.5);
        assert_eq!(triangle2.area(), 7.5);
    }
}
