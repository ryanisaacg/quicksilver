use crate::geom::{Scalar, Vector};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug)]
///A line with a starting and end point
pub struct Line {
    /// The starting point
    pub a: Vector,
    /// The end point
    pub b: Vector,
    /// The thickness, used only for rendering and not collision
    pub t: f32,
}

impl Line {
    ///Create a new line with a start- and an endpoint
    pub fn new(start: impl Into<Vector>, end: impl Into<Vector>) -> Line {
        Line {
            a: start.into(),
            b: end.into(),
            t: 1.0,
        }
    }

    ///Create a line with a changed thickness
    #[must_use]
    pub fn with_thickness(self, thickness: impl Scalar) -> Line {
        Line {
            t: thickness.float(),
            ..self
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Eq for Line {}

#[cfg(test)]
mod tests {
    use crate::geom::*;

    #[test]
    fn overlap_rectangle() {
        let rect = &Rectangle::new_sized((1, 1));
        let line_in = Line::new((0.5, 0.5), (3.0, 3.0));
        let line_on = Line::new(Vector::ZERO, Vector::X * 2);
        let line_not = Line::new((10, 10), (12, 12));
        assert!(line_in.overlaps(rect));
        assert!(line_on.overlaps(rect));
        assert!(!line_not.overlaps(rect));
    }

    #[test]
    fn overlap_circle() {
        let circle = &Circle::new((0, 0), 1);
        let line_on = Line::new((-1, 1), (1, 1));
        let line_in = Line::new(Vector::ZERO, Vector::X * 2);
        let line_not = Line::new((10, 10), (12, 12));
        assert!(line_in.overlaps(circle));
        assert!(line_on.overlaps(circle));
        assert!(!line_not.overlaps(circle));
    }

    #[test]
    fn overlap_line() {
        let line = Line::new(Vector::ZERO, Vector::X);
        let line_parallel = Line::new((0.0, 0.5), (1.0, 0.5));
        let line_cross = Line::new((0.5, -1.0), (0.5, 1.0));
        let line_touch = Line::new((0.0, -1.0), (0.0, 1.0));
        let line_away = Line::new((4, 2), (6, 9));
        assert!(!line.overlaps(&line_parallel));
        assert!(line.overlaps(&line_cross));
        assert!(line.overlaps(&line_touch));
        assert!(!line.overlaps(&line_away));
    }

    #[test]
    fn contains() {
        let line = Line::new(Vector::ZERO, Vector::X);
        let v_on = Vector::new(0.3, 0.0);
        let v_close = Vector::new(0.999, 0.1);
        let v_off = Vector::new(3, 5);
        assert!(line.contains(v_on));
        assert!(!line.contains(v_close));
        assert!(!line.contains(v_off));
    }

    #[test]
    fn constraint() {
        let line = Line::new((5, 5), (10, 7));
        let fits = Rectangle::new((0, 0), (15, 15));
        let not_fit = Rectangle::new((0, 0), (9, 6));
        let fits_line = line.constrain(&fits);
        let not_fits_line = line.constrain(&not_fit);
        assert_eq!(line.a, fits_line.a);
        assert_eq!(line.b, fits_line.b);
        assert_eq!(Vector::new(4, 4), not_fits_line.a);
        assert_eq!(Vector::new(9, 6), not_fits_line.b);
    }

    #[test]
    fn translate() {
        let line = Line::new(Vector::ZERO, Vector::ONE).translate((3, 5));
        assert_eq!(line.a.x, 3.0);
        assert_eq!(line.a.y, 5.0);
        assert_eq!(line.b.x, 4.0);
        assert_eq!(line.b.y, 6.0);
    }
}
