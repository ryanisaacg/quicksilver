use geom::{Bounded, Circle, Line, Rectangle, Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
///A universal shape union
#[allow(missing_docs)]
pub enum Shape {
    Circ(Circle), Line(Line), Rect(Rectangle), Vect(Vector)
}

impl Shape {
    fn as_bounds(&self) -> &Bounded {
        match self {
            &Shape::Circ(ref this) => this as &Bounded,
            &Shape::Line(ref this) => this as &Bounded,
            &Shape::Rect(ref this) => this as &Bounded,
            &Shape::Vect(ref this) => this as &Bounded,
        }

    }
}

impl Bounded for Shape {
    /// Its center as a vector
    fn center(&self) -> Vector { self.as_bounds().center() }
    /// Create a new copy of this shape with the given center
    fn with_center(&self, center: Vector) -> Self {
        match self {
            &Shape::Circ(ref this) => Shape::Circ(this.with_center(center)),
            &Shape::Line(ref this) => Shape::Line(this.with_center(center)),
            &Shape::Rect(ref this) => Shape::Rect(this.with_center(center)),
            &Shape::Vect(ref this) => Shape::Vect(this.with_center(center)),
        }
    }
    /// Check if a point is contained by this shape
    fn contains(&self, vec: Vector) -> bool {
        self.as_bounds().contains(vec)
    }
    /// Check if a line is intersected by this shape
    fn intersects(&self, line: Line) -> bool {
        self.as_bounds().intersects(line)
    }
    /// Check if a circle is overlapped by this shape
    fn overlaps_circ(&self, circ: Circle) -> bool {
        self.as_bounds().overlaps_circ(circ)
    }
    /// Check if a rect is overlapped by this shape
    fn overlaps_rect(&self, rect: Rectangle) -> bool {
        self.as_bounds().overlaps_rect(rect)
    }
    /// Check if this shapes overlaps with another
    fn overlaps(&self, other: &Bounded) -> bool {
        self.as_bounds().overlaps(other)
    }
    /// Move the shape to be within the region, if possible
    fn constrain(&self, bounds: Rectangle) -> Self {
        match self {
            &Shape::Circ(ref this) => Shape::Circ(this.constrain(bounds)),
            &Shape::Line(ref this) => Shape::Line(this.constrain(bounds)),
            &Shape::Rect(ref this) => Shape::Rect(this.constrain(bounds)),
            &Shape::Vect(ref this) => Shape::Vect(this.constrain(bounds)),
        }
    }
    /// The smallest possible rectangle that fully contains the shape
    fn bounding_box(&self) -> Rectangle {
        self.as_bounds().bounding_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_shapes() -> [Shape; 4] {
        [
            Shape::Circ(Circle::new(0, 0, 32)),
            Shape::Line(Line::new(Vector::new(0, 0), Vector::new(32, 32))),
            Shape::Rect(Rectangle::new(0, 0, 32, 32)),
            Shape::Vect(Vector::new(0, 0))
        ]
    }

    #[test]
    fn overlaps() {
        for a in get_shapes().iter() {
            for b in get_shapes().iter() {
                println!("{:?}, {:?}", a, b);
                assert!(a.overlaps(b));
            }
        }
    }

    #[test]
    fn with_center() {
        for a in get_shapes().iter() {
            assert_eq!(a.with_center(Vector::new(50, 40)).center(), Vector::new(50, 40));
        }
    }

    #[test]
    fn translate() {
        for a in get_shapes().iter() {
            assert_eq!(a.translate(Vector::new(10, 5)).center(), a.center() + Vector::new(10, 5));
        }
    }
}
