use geom::{about_equal, Circle, Positioned, Scalar, Transform, Vector, Rectangle};
use graphics::{DrawAttributes, Drawable, GpuTriangle, Vertex, Window};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A line with a starting and end point
pub struct Line {
    ///The starting point
    pub a: Vector,
    ///The end point
    pub b: Vector,
    ///The thickness
    pub t: f32,
}

impl Line {
    ///Create a line from x and y coordinates of the start and end point
    pub fn new<T: Scalar>(x_1: T, y_1: T, x_2: T, y_2: T) -> Line {
        Line {
            a: Vector::new(x_1, y_1),
            b: Vector::new(x_2, y_2),
            t: 1.0
        }
    }

    ///Create a line from `Vector`s of the start and end point
    pub fn newv(start: Vector, end: Vector) -> Line {
        Line {
            a: start,
            b: end,
            t: 1.0
        }
    }

    ///Create a line starting at the origin with a given length on the x axis
    pub fn new_sized<T: Scalar>(length: T) -> Line {
        Line::newv(Vector::zero(), Vector::new(length.float(), 0.0))
    }

    ///Create a line starting at the origin and ending on the given point
    pub fn newv_sized(end: Vector) -> Line {
        Line::newv(Vector::zero(), end)
    }

    ///Check if a point falls on the line
    pub fn contains(self, v: Vector) -> bool {
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
        // check if start or end point is in the rectangle
        if b.contains(self.a) || b.contains(self.b) {
            return true;
        }

        // check each edge (top, bottom, left, right) if it overlaps our line
        let top_left = b.top_left();
        let top_right = top_left + Vector::new(b.width, 0.0);

        // check top
        if self.overlaps_line(Line::newv(top_left, top_right)) {
            return true; // instantly return because we do not have to check the others
        }

        let bottom_left = top_left + Vector::new(0.0, b.height);

        // check left
        if self.overlaps_line(Line::newv(top_left, bottom_left)) {
            return true;
        }

        let bottom_right = top_left + Vector::new(b.width, b.height);

        // check right
        if self.overlaps_line(Line::newv(top_right, bottom_right)) {
            return true;
        }

        // check bottom
        if self.overlaps_line(Line::newv(bottom_left, bottom_right)) {
            return true;
        }

        false
    }

    ///Check if a line overlaps a circle
    pub fn overlaps_circ(self, c: Circle) -> bool {
        // check if start or end point is in the circle
        if c.contains(self.a) || c.contains(self.b) {
            return true;
        }

        // get dot product of the line and circle
        let dot = ( ((c.x-self.a.x)*(self.b.x-self.a.x)) + ((c.y-self.a.y)*(self.b.y-self.a.y)) )
            / self.a.distance(self.b).powi(2);

        // find the closest point on the line
        let closest_x = self.a.x + (dot * (self.b.x-self.a.x));
        let closest_y = self.a.y + (dot * (self.b.y-self.a.y));

        // is this point actually on the line segment?
        // if so keep going, but if not, return false
        if !self.contains(Vector::new(closest_x, closest_y)) { return false; }

        // get distance to closest point
        let distance = c.center().distance(Vector::new(closest_x, closest_y));

        if distance <= c.radius {
            return true;
        }
        false
    }

    ///Move the line so it is entirely contained within a rectangle
    pub fn constrain(self, outer: Rectangle) -> Line {
        unimplemented!()
    }

    ///Translate the line by a given vector
    pub fn translate(self, v: Vector) -> Line {
        Line::newv(self.a + v, self.b + v)
    }

    ///Create a line with the same size at a given center
    pub fn with_center(self, v: Vector) -> Line {
        self.translate(v - self.center())
    }

    ///Create a line with a changed thickness
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
            top_left.x,
            top_left.y,
            self.a.x.max(self.b.x) - top_left.x,
            self.a.y.max(self.b.y) - top_left.y
        )
    }
}

impl Drawable for Line {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        // create rectangle in right size
        let rect = Rectangle::new(self.a.x, self.a.y + self.t / 2.0, self.a.distance(self.b), self.t);

        // shift position of rectangle
        let trans_x = (self.a.x + self.b.x) / 2.0 - rect.center().x;
        let trans_y = (self.a.y + self.b.y) / 2.0 - rect.center().y;

        let dx = self.b.x - self.a.x;
        let dy = self.b.y - self.a.y;

        let transform = Transform::translate(Vector::new(trans_x, trans_y))
            * Transform::rotate(dy.atan2(dx).to_degrees());

        let new_params = DrawAttributes {
            transform: transform * params.transform,
            ..params
        };
        rect.draw(window, new_params);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_rectangle() {
        let rect = Rectangle::new_sized(1, 1);
        let line_in = Line::new(0.5, 0.5, 3.0, 3.0);
        let line_on = Line::new_sized(2);
        let line_not = Line::new(10, 10, 12, 12);
        assert!(line_in.overlaps_rect(rect));
        assert!(line_on.overlaps_rect(rect));
        assert!(!line_not.overlaps_rect(rect));
    }

    #[test]
    fn overlap_circle() {
        let circle = Circle::new(0, 0, 1);
        let line_on = Line::new(-1, 1, 1, 1);
        let line_in = Line::new_sized(2);
        let line_not = Line::new(10, 10, 12, 12);
        assert!(line_in.overlaps_circ(circle));
        assert!(line_on.overlaps_circ(circle));
        assert!(!line_not.overlaps_circ(circle));
    }

    #[test]
    fn overlap_line() {
        let line = Line::new_sized(1);
        let line_parallel = Line::new(0.0, 0.5, 1.0, 0.5);
        let line_cross = Line::new(0.5, -1.0, 0.5, 1.0);
        let line_touch = Line::new(0.0, -1.0, 0.0, 1.0);
        let line_away = Line::new(4, 2, 6, 9);
        assert!(!line.overlaps_line(line_parallel));
        assert!(line.overlaps_line(line_cross));
        assert!(line.overlaps_line(line_touch));
        assert!(!line.overlaps_line(line_away));
    }

    #[test]
    fn contains() {
        let line = Line::new_sized(1);
        let v_on = Vector::new(0.3, 0.0);
        let v_close = Vector::new(0.999, 0.1);
        let v_off = Vector::new(3, 5);
        assert!(line.contains(v_on));
        assert!(!line.contains(v_close));
        assert!(!line.contains(v_off));
    }

    #[test]
    fn constraint() {

    }

    #[test]
    fn translate() {
        let line = Line::newv_sized(Vector::one()).translate(Vector::new(3, 5));
        assert_eq!(line.a.x, 3.0);
        assert_eq!(line.a.y, 5.0);
        assert_eq!(line.b.x, 4.0);
        assert_eq!(line.b.y, 6.0);
    }
}
