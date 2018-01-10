use super::{Circle, Line, Rectangle, Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
///A universal shape union
#[allow(missing_docs)]
pub enum Shape {
    Circ(Circle), Line(Line), Rect(Rectangle), Vect(Vector)
}

impl Shape {
    ///Check if the shape overlaps with a circle
    pub fn overlaps_circ(&self, circ: Circle) -> bool {
        match *self {
            Shape::Circ(this) => this.overlaps_circ(circ),
            Shape::Line(this) => circ.intersects(this),
            Shape::Rect(this) => this.overlaps_circ(circ),
            Shape::Vect(this) => circ.contains(this)
        }
    }

    ///Check if the shape overlaps with a rectangle
    pub fn overlaps_rect(&self, rect: Rectangle) -> bool {
        match *self {
            Shape::Circ(this) => this.overlaps_rect(rect),
            Shape::Line(this) => rect.intersects(this),
            Shape::Rect(this) => this.overlaps_rect(rect),
            Shape::Vect(this) => rect.contains(this)
        }
    }

    ///Check if the shape intersects with a line
    pub fn intersects(&self, line: Line) -> bool {
        match *self {
            Shape::Circ(this) => this.intersects(line),
            Shape::Line(this) => line.intersects(this),
            Shape::Rect(this) => this.intersects(line),
            Shape::Vect(this) => line.contains(this)
        }
    }

    ///Check if the shape contains a vector
    pub fn contains(&self, vec: Vector) -> bool {
        match *self {
            Shape::Circ(this) => this.contains(vec),
            Shape::Line(this) => this.contains(vec),
            Shape::Rect(this) => this.contains(vec),
            Shape::Vect(this) => this == vec
        }
    }

    ///Check if the shape overlaps with another shape
    pub fn overlaps(&self, shape: Shape) -> bool {
        match *self {
            Shape::Circ(this) => shape.overlaps_circ(this),
            Shape::Line(this) => shape.intersects(this),
            Shape::Rect(this) => shape.overlaps_rect(this),
            Shape::Vect(this) => shape.contains(this)
        }
    }

    ///Create a shape moved by a given amount
    pub fn translate(&self, vec: Vector) -> Shape {
        match *self {
            Shape::Circ(this) => Shape::Circ(this.translate(vec)),
            Shape::Line(this) => Shape::Line(this.translate(vec)),
            Shape::Rect(this) => Shape::Rect(this.translate(vec)),
            Shape::Vect(this) => Shape::Vect(this + vec)
        }
    }

    ///Create a copy of the shape with a given center
    pub fn with_center(&self, vec: Vector) -> Shape {
        match *self {
            Shape::Circ(this) => Shape::Circ(Circle::new(vec.x, vec.y, this.radius)),
            Shape::Line(this) => { let midlength = (this.end - this.start) / 2; Shape::Line(Line::new(vec - midlength, vec + midlength)) },
            Shape::Rect(this) => Shape::Rect(this.with_center(vec)),
            Shape::Vect(_) => Shape::Vect(vec)
        }
    }

    ///Find the smallest bounding box that contains the shape
    pub fn bounding_box(&self) -> Rectangle {
        match *self {
            Shape::Circ(this) => Rectangle::new(this.x - this.radius, this.y - this.radius, this.radius * 2.0, this.radius * 2.0),
            Shape::Line(this) => {
                let x = this.start.x.min(this.end.x);
                let y = this.start.y.min(this.end.y);
                Rectangle::new(x, y, this.start.x.max(this.end.x) - x, this.start.y.max(this.end.y) - y)
            },
            Shape::Rect(this) => this,
            Shape::Vect(this) => Rectangle::newv(this, Vector::zero())
        }
    }

    ///Find the center of the shape
    pub fn center(&self) -> Vector {
        match *self {
            Shape::Circ(this) => this.center(),
            Shape::Line(this) => (this.start + this.end) / 2,
            Shape::Rect(this) => this.center(),
            Shape::Vect(this) => this
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_shapes() -> [Shape; 4] {
        [
            Shape::Circ(Circle::newi(0, 0, 32)),
            Shape::Line(Line::new(Vector::newi(0, 0), Vector::newi(32, 32))),
            Shape::Rect(Rectangle::newi(0, 0, 32, 32)),
            Shape::Vect(Vector::newi(0, 0))
        ]
    }

    #[test]
    fn overlaps() {
        for a in get_shapes().iter() {
            for b in get_shapes().iter() {
                println!("{:?}, {:?}", a, b);
                assert!(a.overlaps(*b));
            }
        }
    }

    #[test]
    fn with_center() {
        for a in get_shapes().iter() {
            assert_eq!(a.with_center(Vector::newi(50, 40)).center(), Vector::newi(50, 40));
        }
    }

    #[test]
    fn translate() {
        for a in get_shapes().iter() {
            assert_eq!(a.translate(Vector::newi(10, 5)).center(), a.center() + Vector::newi(10, 5));
        }
    }
}
