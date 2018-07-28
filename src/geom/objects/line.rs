use geom::{about_equal, Circle, Positioned, Vector, Rectangle};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
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
            t: 1.0
        }
    }

    ///Check if a point falls on the line
    pub fn contains(self, v: impl Into<Vector>) -> bool {
        let v = v.into();
        about_equal(v.distance(self.a) + v.distance(self.b), self.a.distance(self.b))
    }

    ///Check if a line overlaps another line
    pub fn overlaps_line(self, l: Line) -> bool {
        // calculate the distance to intersection point
        let d1 = ((l.b.x - l.a.x) * (self.a.y - l.a.y) - (l.b.y - l.a.y) * (self.a.x - l.a.x))
            / ((l.b.y - l.a.y) * (self.b.x - self.a.x) - (l.b.x - l.a.x) * (self.b.y - self.a.y));
        let d2 = ((self.b.x - self.a.x) * (self.a.y - l.a.y) - (self.b.y - self.a.y) * (self.a.x - l.a.x))
            / ((l.b.y - l.a.y) * (self.b.x - self.a.x) - (l.b.x - l.a.x) * (self.b.y - self.a.y));

        // if d1 and d2 are between 0-1, lines are colliding
        if d1 >= 0.0 && d1 <= 1.0 && d2 >= 0.0 && d2 <= 1.0 {
            return true;

            // for anyone interested, here is the intersection point:
            // let intersect_x = self.a.x + (d1 * (self.b.x-self.a.x));
            // let intersect_y = self.a.y + (d1 * (self.b.y-self.a.y));
        }
        false
    }

    ///Check if a line overlaps a rectangle
    pub fn overlaps_rect(self, b: Rectangle) -> bool {
        // check each edge (top, bottom, left, right) if it overlaps our line
        let top_left = b.top_left();
        let top_right = top_left + Vector::new(b.width(), 0.0);
        let bottom_left = top_left + Vector::new(0.0, b.height());
        let bottom_right = top_left + Vector::new(b.width(), b.height());

        b.contains(self.a) || b.contains(self.b)
            || self.overlaps_line(Line::new(top_left, top_right))
            || self.overlaps_line(Line::new(top_left, bottom_left))
            || self.overlaps_line(Line::new(top_right, bottom_right))
            || self.overlaps_line(Line::new(bottom_left, bottom_right))
    }

    ///Check if a line overlaps a circle
    pub fn overlaps_circ(self, c: Circle) -> bool {
        // check if start or end point is in the circle
        if c.contains(self.a) || c.contains(self.b) {
            true
        } else {
            let length = self.b - self.a;
            // get dot product of the line and circle
            let dot = length.dot(c.center() - self.a) / length.len2();
            // find the closest point on the line
            let closest = self.a + length * dot;
            // make sure the point is on the line, and in the circle
            self.contains(closest)
                && (closest - c.center()).len2() <= c.radius.powi(2)
        }
    }

    ///Move the line so it is entirely contained within a rectangle
    #[must_use]
    pub fn constrain(self, outer: Rectangle) -> Line {
        let mut line = self;
        line = line.translate(line.a.constrain(outer) - line.a);
        line.translate(line.b.constrain(outer) - line.b)
    }

    ///Translate the line by a given vector
    #[must_use]
    pub fn translate(self, vec: impl Into<Vector>) -> Line {
        let vec = vec.into();
        Line::new(self.a + vec, self.b + vec)
    }

    ///Create a line with the same size at a given center
    #[must_use]
    pub fn with_center(self, v: impl Into<Vector>) -> Line {
        self.translate(v.into() - self.center())
    }

    ///Create a line with a changed thickness
    #[must_use]
    pub fn with_thickness(self, thickness: f32) -> Line {
        Line {
            t: thickness,
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

impl Positioned for Line {
    fn center(&self) -> Vector {
        (self.a + self.b) / 2
    }

    fn bounding_box(&self) -> Rectangle {
        let top_left = Vector::new(self.a.x.min(self.b.x), self.a.y.min(self.b.y));
        Rectangle::new(
            (
                top_left.x, 
                top_left.y
            ),
            (
                self.a.x.max(self.b.x) - top_left.x,
                self.a.y.max(self.b.y) - top_left.y
            )
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_rectangle() {
        let rect = Rectangle::new_sized((1, 1));
        let line_in = Line::new((0.5, 0.5), (3.0, 3.0));
        let line_on = Line::new(Vector::ZERO, Vector::X * 2);
        let line_not = Line::new((10, 10), (12, 12));
        assert!(line_in.overlaps_rect(rect));
        assert!(line_on.overlaps_rect(rect));
        assert!(!line_not.overlaps_rect(rect));
    }

    #[test]
    fn overlap_circle() {
        let circle = Circle::new((0, 0), 1);
        let line_on = Line::new((-1, 1), (1, 1));
        let line_in = Line::new(Vector::ZERO, Vector::X * 2);
        let line_not = Line::new((10, 10), (12, 12));
        assert!(line_in.overlaps_circ(circle));
        assert!(line_on.overlaps_circ(circle));
        assert!(!line_not.overlaps_circ(circle));
    }

    #[test]
    fn overlap_line() {
        let line = Line::new(Vector::ZERO, Vector::X);
        let line_parallel = Line::new((0.0, 0.5), (1.0, 0.5));
        let line_cross = Line::new((0.5, -1.0), (0.5, 1.0));
        let line_touch = Line::new((0.0, -1.0), (0.0, 1.0));
        let line_away = Line::new((4, 2), (6, 9));
        assert!(!line.overlaps_line(line_parallel));
        assert!(line.overlaps_line(line_cross));
        assert!(line.overlaps_line(line_touch));
        assert!(!line.overlaps_line(line_away));
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
        let fits_line = line.constrain(fits);
        let not_fits_line = line.constrain(not_fit);
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
