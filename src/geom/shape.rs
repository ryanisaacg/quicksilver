use crate::geom::{Circle, Line, Rectangle, Triangle, Vector, Transform, about_equal};

/// The collision and positional attributes of shapes
pub trait Shape {
    /// If the point lies on the shape's boundary or within it
    #[must_use]
    fn contains(&self, point: impl Into<Vector>) -> bool;
    /// If any area bounded by the shape falls on the line
    #[must_use]
    fn intersects(&self, line: &Line) -> bool { self.overlaps(line) }
    /// If any area is bounded by both the shape and the circle
    #[must_use]
    fn overlaps_circle(&self, circle: &Circle) -> bool { self.overlaps(circle) }
    /// If any area is bounded by both the shape and the rectangle
    #[must_use]
    fn overlaps_rectangle(&self, rectangle: &Rectangle) -> bool { self.overlaps(rectangle) }
    /// If any area is bounded by both either shape
    #[must_use]
    fn overlaps(&self, other: &impl Shape) -> bool;

    /// The point all other points are equidistant to in the shape
    #[must_use]
    fn center(&self) -> Vector;
    /// A Rectangle that contains the entire shape
    #[must_use]
    fn bounding_box(&self) -> Rectangle;
    /// Apply a transform to a shape then get the bounding box for the transformed shape
    #[must_use]
    fn transform_bounding_box(&self, transform: Transform) -> Rectangle {
        let bb = self.bounding_box();
        // Build the transform to rotate the shape
        let transform = Transform::translate(bb.center() + bb.pos)
            * transform
            * Transform::translate(-bb.pos - bb.center());
        // Get new corner position
        let tl = transform * bb.pos;
        let tr = transform * Vector::new(bb.pos.x + bb.size.x, bb.pos.y);
        let bl = transform * Vector::new(bb.pos.x, bb.pos.y + bb.size.y);
        let br = transform * Vector::new(bb.pos.x + bb.size.x, bb.pos.y + bb.size.y);
        // Get min and max points
        let min = tr.min(tl).min(bl).min(br);
        let max = tr.max(tl).max(bl).max(br);
        // Make new bounding box
        Rectangle::new(min, max - min)
    }
    /// Create a copy of the shape with an offset center
    #[must_use]
    fn translate(&self, amount: impl Into<Vector>) -> Self where Self: Sized;
    /// Create a copy of the shape that is contained within the bound
    #[must_use]
    fn constrain(&self, outer: &Rectangle) -> Self where Self: Sized {
        let area = self.bounding_box();
        let clamped = area.top_left().clamp(outer.top_left(), outer.top_left() + outer.size() - area.size());
        self.translate(clamped - area.top_left())
    }
    /// Create a copy of the shape with an offset center
    #[must_use]
    fn with_center(&self, center: impl Into<Vector>) -> Self where Self: Sized {
        self.translate(center.into() - self.center())
    }
}

impl Shape for Circle {
    fn contains(&self, v: impl Into<Vector>) -> bool {
        (v.into() - self.center()).len2() < self.radius.powi(2)
    }
    fn overlaps_circle(&self, c: &Circle) -> bool { 
        (self.center() - c.center()).len2() < (self.radius + c.radius).powi(2)
    }
    fn overlaps(&self, shape: &impl Shape) -> bool {
        shape.overlaps_circle(self)
    }

    fn center(&self) -> Vector { self.pos }
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.pos - Vector::ONE * self.radius, Vector::ONE * 2 * self.radius)
    }
    fn translate(&self, v: impl Into<Vector>) -> Self {
        Circle {
            pos: self.pos + v.into(),
            radius: self.radius
        }
    }
}

impl Shape for Rectangle {
    fn contains(&self, point: impl Into<Vector>) -> bool {
        let p = point.into();

        return p.x >= self.x()
            && p.y >= self.y()
            && p.x < self.x() + self.width()
            && p.y < self.y() + self.height()
    }
    fn overlaps_circle(&self, c: &Circle) -> bool {
        (c.center().clamp(self.top_left(), self.top_left() + self.size()) - c.center()).len2() < c.radius.powi(2)
    }
    fn overlaps_rectangle(&self, b: &Rectangle) -> bool {
        self.x() < b.pos.x + b.size.x && self.x() + self.width() > b.pos.x && self.y() < b.pos.y + b.size.y &&
            self.y() + self.height() > b.pos.y
    }

    fn intersects(&self, l: &Line) -> bool { l.overlaps_rectangle(self) }
    fn overlaps(&self, shape: &impl Shape) -> bool { shape.overlaps_rectangle(self) }


    fn center(&self) -> Vector { self.pos + self.size / 2 }
    fn bounding_box(&self) -> Rectangle { *self }
    fn translate(&self, v: impl Into<Vector>) -> Self {
        Rectangle {
            pos: self.pos + v.into(),
            size: self.size
        }
    }
}

impl Shape for Triangle {
    fn contains(&self, v: impl Into<Vector>) -> bool {
        let v = v.into();
        // form three triangles with this new vector
        let t_1 = Triangle::new(v, self.a, self.b);
        let t_2 = Triangle::new(v, self.b, self.c);
        let t_3 = Triangle::new(v, self.c, self.a);

        // calculate the area these smaller triangles make
        // if they add up to be the area of this triangle, the point is inside it
        about_equal(t_1.area() + t_2.area() + t_3.area(), self.area())
    }
    fn intersects(&self, line: &Line) -> bool {
        // check each the vertices and each edge if it overlaps the line
        self.contains(line.a)
            || self.contains(line.b)
            || Line::new(self.a, self.b).intersects(line)
            || Line::new(self.b, self.c).intersects(line)
            || Line::new(self.c, self.a).intersects(line)
    }
    fn overlaps_circle(&self, circ: &Circle) -> bool{
        Line::new(self.a, self.b).overlaps_circle(circ)
            || Line::new(self.b, self.c).overlaps_circle(circ)
            || Line::new(self.c, self.a).overlaps_circle(circ)
    }
    fn overlaps_rectangle(&self, rect: &Rectangle) ->bool {
        Line::new(self.a, self.b).overlaps_rectangle(rect)
            || Line::new(self.b, self.c).overlaps_rectangle(rect)
            || Line::new(self.c, self.a).overlaps_rectangle(rect)
    }
    fn overlaps(&self, other: &impl Shape) -> bool {
        self.contains(other.center())
            || other.intersects(&Line::new(self.a, self.b))
            || other.intersects(&Line::new(self.b, self.c))
            || other.intersects(&Line::new(self.a, self.c))
    }
    
    fn center(&self) -> Vector {
        (self.a + self.b + self.c) / 3
    }
    fn bounding_box(&self) -> Rectangle {
        let min = self.a.min(self.b.min(self.c));
        let max = self.a.max(self.b.max(self.c));
        Rectangle::new(min, max - min)
    }
    fn translate(&self, v: impl Into<Vector>) -> Self {
        let v = v.into();
        Triangle {
            a: self.a + v,
            b: self.b + v,
            c: self.c + v
        }
    }
}

impl Shape for Line {
    fn contains(&self, v: impl Into<Vector>) -> bool {
        let v = v.into();
        about_equal(v.distance(self.a) + v.distance(self.b), self.a.distance(self.b))
    }
    fn intersects(&self, l: &Line) -> bool {
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
    fn overlaps_circle(&self, c: &Circle) -> bool {
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
    fn overlaps_rectangle(&self, b: &Rectangle) -> bool {
        // check each edge (top, bottom, left, right) if it overlaps our line
        let top_left = b.top_left();
        let top_right = top_left + Vector::new(b.width(), 0.0);
        let bottom_left = top_left + Vector::new(0.0, b.height());
        let bottom_right = top_left + Vector::new(b.width(), b.height());

        b.contains(self.a) || b.contains(self.b)
            || self.intersects(&Line::new(top_left, top_right))
            || self.intersects(&Line::new(top_left, bottom_left))
            || self.intersects(&Line::new(top_right, bottom_right))
            || self.intersects(&Line::new(bottom_left, bottom_right))
    }
    fn overlaps(&self, shape: &impl Shape) -> bool {
        shape.intersects(self)
    }
    
    fn center(&self) -> Vector { (self.a + self.b) / 2 }
    fn bounding_box(&self) -> Rectangle {
        let min = self.a.min(self.b);
        let max = self.a.max(self.b);
        Rectangle::new(min, max - min)
    }
    fn translate(&self, v: impl Into<Vector>) -> Self {
        let v = v.into();
        Line {
            a: self.a + v,
            b: self.b + v,
            t: self.t
        }
    }
}

impl Shape for Vector {
    fn contains(&self, v: impl Into<Vector>) -> bool {
        *self == v.into()
    }
    fn overlaps(&self, shape: &impl Shape) -> bool {
        shape.contains(*self)
    }

    fn center(&self) -> Vector { *self }
    fn bounding_box(&self) -> Rectangle { Rectangle::new(*self, Vector::ONE) }
    fn translate(&self, v: impl Into<Vector>) -> Vector { *self + v.into() }
}
