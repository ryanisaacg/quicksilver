use geom::{Circle, Line, Positioned, Rectangle, Vector};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
///A universal shape union
#[allow(missing_docs)]
pub enum Shape {
    Circle(Circle), Line(Line), Rectangle(Rectangle), Vector(Vector)
}

impl Shape {
    ///Check if the shape overlaps with a circle
    pub fn overlaps_circ(&self, circ: Circle) -> bool {
        match *self {
            Shape::Circle(this) => this.overlaps_circ(circ),
            Shape::Line(this) => circ.intersects(this),
            Shape::Rectangle(this) => this.overlaps_circ(circ),
            Shape::Vector(this) => circ.contains(this)
        }
    }

    ///Check if the shape overlaps with a rectangle
    pub fn overlaps_rect(&self, rect: Rectangle) -> bool {
        match *self {
            Shape::Circle(this) => this.overlaps_rect(rect),
            Shape::Line(this) => rect.intersects(this),
            Shape::Rectangle(this) => this.overlaps_rect(rect),
            Shape::Vector(this) => rect.contains(this)
        }
    }

    ///Check if the shape intersects with a line
    pub fn intersects(&self, line: Line) -> bool {
        match *self {
            Shape::Circle(this) => this.intersects(line),
            Shape::Line(this) => line.intersects(this),
            Shape::Rectangle(this) => this.intersects(line),
            Shape::Vector(this) => line.contains(this)
        }
    }

    ///Check if the shape contains a vector
    pub fn contains(&self, vec: Vector) -> bool {
        match *self {
            Shape::Circle(this) => this.contains(vec),
            Shape::Line(this) => this.contains(vec),
            Shape::Rectangle(this) => this.contains(vec),
            Shape::Vector(this) => this == vec
        }
    }

    ///Check if the shape overlaps with another shape
    pub fn overlaps(&self, shape: Shape) -> bool {
        match *self {
            Shape::Circle(this) => shape.overlaps_circ(this),
            Shape::Line(this) => shape.intersects(this),
            Shape::Rectangle(this) => shape.overlaps_rect(this),
            Shape::Vector(this) => shape.contains(this)
        }
    }

    ///Create a shape moved by a given amount
    pub fn translate(&self, vec: Vector) -> Shape {
        match *self {
            Shape::Circle(this) => Shape::Circle(this.translate(vec)),
            Shape::Line(this) => Shape::Line(this.translate(vec)),
            Shape::Rectangle(this) => Shape::Rectangle(this.translate(vec)),
            Shape::Vector(this) => Shape::Vector(this + vec)
        }
    }

    ///Create a copy of the shape with a given center
    pub fn with_center(&self, vec: Vector) -> Shape {
        match *self {
            Shape::Circle(this) => Shape::Circle(Circle::new(vec.x, vec.y, this.radius)),
            Shape::Line(this) => { let midlength = (this.end - this.start) / 2; Shape::Line(Line::new(vec - midlength, vec + midlength)) },
            Shape::Rectangle(this) => Shape::Rectangle(this.with_center(vec)),
            Shape::Vector(_) => Shape::Vector(vec)
        }
    }

    fn as_positioned(&self) -> &Positioned {
        match self {
            &Shape::Circle(ref this) => this as &Positioned,
            &Shape::Line(ref this) => this as &Positioned,
            &Shape::Rectangle(ref this) => this as &Positioned,
            &Shape::Vector(ref this) => this as &Positioned,
        }

    }
}

impl Positioned for Shape {
    fn center(&self) -> Vector {
        self.as_positioned().center()
    }

    fn bounding_box(&self) -> Rectangle {
        self.as_positioned().bounding_box()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_shapes() -> [Shape; 4] {
        [
            Shape::Circle(Circle::new(0, 0, 32)),
            Shape::Line(Line::new(Vector::new(0, 0), Vector::new(32, 32))),
            Shape::Rectangle(Rectangle::new(0, 0, 32, 32)),
            Shape::Vector(Vector::new(0, 0))
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
