use geom::{about_equal, Circle, Positioned, Scalar, Transform, Vector, Rectangle, Line};
use graphics::{DrawAttributes, Drawable, GpuTriangle, Vertex, Window};
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
    ///Create a triangle from x and y coordinates of the three points
    pub fn new<T: Scalar>(x_1: T, y_1: T, x_2: T, y_2: T, x_3: T, y_3: T) -> Triangle {
        Triangle {
            a: Vector::new(x_1, y_1),
            b: Vector::new(x_2, y_2),
            c: Vector::new(x_3, y_3),
        }
    }

    ///Create a triangle from `Vector`s of all three points
    pub fn newv(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle { a, b, c }
    }

    ///Check if a point is inside the triangle
    pub fn contains(self, v: Vector) -> bool {
        // form three triangles with this new vector
        let t_1 = Triangle::newv(v, self.a, self.b);
        let t_2 = Triangle::newv(v, self.b, self.c);
        let t_3 = Triangle::newv(v, self.c, self.a);

        // calculate the area these smaller triangles make
        // if they add up to be the area of this triangle, the point is inside it
        about_equal(t_1.area() + t_2.area() + t_3.area(), self.area())
    }

    ///Check if this triangle overlaps a line
    pub fn overlaps_line(self, l: Line) -> bool {
        // check if start or end point is in the triangle
        if self.contains(l.a) || self.contains(l.b) {
            return true;
        }

        // check each edge if it overlaps the line
        Line::newv(self.a, self.b).overlaps_line(l)
        || Line::newv(self.b, self.c).overlaps_line(l)
        || Line::newv(self.c, self.a).overlaps_line(l)
    }

    ///Check if this triangle overlaps a rectangle
    pub fn overlaps_rect(self, b: Rectangle) -> bool {
        Line::newv(self.a, self.b).overlaps_rect(b)
        || Line::newv(self.b, self.c).overlaps_rect(b)
        || Line::newv(self.c, self.a).overlaps_rect(b)
    }

    ///Check if this triangle overlaps a circle
    pub fn overlaps_circ(self, c: Circle) -> bool {
        Line::newv(self.a, self.b).overlaps_circ(c)
        || Line::newv(self.b, self.c).overlaps_circ(c)
        || Line::newv(self.c, self.a).overlaps_circ(c)
    }

    ///Move the triangle so it is entirely contained within a rectangle
    #[must_use]
    pub fn constrain(self, outer: Rectangle) -> Triangle {
        let mut line = self;
        line = line.translate(line.a.constrain(outer) - line.a);
        line = line.translate(line.b.constrain(outer) - line.b);
        line.translate(line.c.constrain(outer) - line.c)
    }

    ///Translate the triangle by a given vector
    #[must_use]
    pub fn translate(self, v: Vector) -> Triangle {
        Triangle::newv(self.a + v, self.b + v, self.c + v)
    }

    ///Create a triangle with the same size at a given center
    #[must_use]
    pub fn with_center(self, v: Vector) -> Triangle {
        self.translate(v - self.center())
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

impl Positioned for Triangle {
    fn center(&self) -> Vector {
        (self.a + self.b + self.c) / 3
    }

    fn bounding_box(&self) -> Rectangle {
        let min_x = self.a.x.min(self.b.x.min(self.c.x));
        let min_y = self.a.y.min(self.b.y.min(self.c.y));
        let max_x = self.a.x.max(self.b.x.max(self.c.x));
        let max_y = self.a.y.max(self.b.y.max(self.c.y));
        Rectangle::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }
}

impl Drawable for Triangle {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        let trans = Transform::translate(self.center())
            * params.transform
            * Transform::translate(-self.center());
        let vertices = &[
            Vertex::new_untextured(trans * self.a, params.color),
            Vertex::new_untextured(trans * self.b, params.color),
            Vertex::new_untextured(trans * self.c, params.color)
        ];
        let triangles = &[
            GpuTriangle::new_untextured([0, 1, 2], params.z),
        ];
        window.add_vertices(vertices.iter().cloned(), triangles.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_rectangle() {
        let rect = Rectangle::new_sized(1, 1);
        let t_inside = Triangle::new(0.25, 0.25, 0.75, 0.25, 0.25, 0.75);
        let t_over = Triangle::new(0.5, -0.5, 0.5, 1.5, 1.5, 0.5);
        let t_outside = Triangle::new(2, 3, 4, 5, 10, 12);
        assert!(t_inside.overlaps_rect(rect));
        assert!(t_over.overlaps_rect(rect));
        assert!(!t_outside.overlaps_rect(rect));
    }

    #[test]
    fn overlap_circle() {
        let circle = Circle::new(0, 0, 1);
        let t_inside = Triangle::new(-0.5, -0.5, 0.5, -0.5, 0.0, 0.5);
        let t_over = Triangle::new(0, -2, 0, 2, 2, 0);
        let t_outside = Triangle::new(2, 3, 4, 5, 10, 12);
        assert!(t_inside.overlaps_circ(circle));
        assert!(t_over.overlaps_circ(circle));
        assert!(!t_outside.overlaps_circ(circle));
    }

    #[test]
    fn overlap_line() {
        let line = Line::new_sized(1);
        let t_on = Triangle::new(0, 0, 1, 1, 0, 1);
        let t_over = Triangle::new(0.25, -0.5, 0.75, -0.5, 0.5, 0.5);
        let t_outside = Triangle::new(2, 3, 4, 5, 10, 12);
        assert!(t_on.overlaps_line(line));
        assert!(t_over.overlaps_line(line));
        assert!(!t_outside.overlaps_line(line));
    }

    #[test]
    fn contains() {
        let triangle = Triangle::new(0, 0, 1, 0, 0, 1);
        let p_in = Vector::new(0.25, 0.25);
        let p_on = Vector::new(0.5, 0.5);
        let p_off = Vector::new(1, 1);
        assert!(triangle.contains(p_in));
        assert!(triangle.contains(p_on));
        assert!(!triangle.contains(p_off));
    }

    #[test]
    fn constraint() {
        let tri = Triangle::new(5, 5, 10, 7, 8, 8);
        let fits = Rectangle::new(0, 0, 15, 15);
        let not_fit = Rectangle::new(0, 0, 9, 6);
        let fits_tri = tri.constrain(fits);
        let not_fits_tri = tri.constrain(not_fit);
        assert_eq!(tri.a, fits_tri.a);
        assert_eq!(tri.b, fits_tri.b);
        assert_eq!(tri.c, fits_tri.c);
        assert_eq!(Vector::new(4, 3), not_fits_tri.a);
        assert_eq!(Vector::new(9, 5), not_fits_tri.b);
        assert_eq!(Vector::new(7, 6), not_fits_tri.c);
    }

    #[test]
    fn translate() {
        let triangle = Triangle::new(0, 0, 1, 0, 0, 1).translate(Vector::ONE);
        assert_eq!(triangle, Triangle::new(1, 1, 2, 1, 1, 2));
    }

    #[test]
    fn area() {
        let triangle = Triangle::new(0, 0, 1, 0, 0, 1);
        let triangle2 = Triangle::new(-4, -1, -1, -1, 1, 4);
        assert_eq!(triangle.area(), 0.5);
        assert_eq!(triangle2.area(), 7.5);
    }
}
