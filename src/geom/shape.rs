use super::{Circle, Line, Rectangle, Vector};

#[derive(Clone, Copy)]
pub enum Shape {
    Circ(Circle), Line(Line), Rect(Rectangle), Vect(Vector)
}

impl Shape {
    pub fn overlaps_circ(&self, circ: Circle) -> bool {
        match *self {
            Shape::Circ(this) => this.overlaps_circ(circ),
            Shape::Line(this) => circ.intersects(this),
            Shape::Rect(this) => this.overlaps_circ(circ),
            Shape::Vect(this) => circ.contains(this)
        }
    }

    pub fn overlaps_rect(&self, rect: Rectangle) -> bool {
        match *self {
            Shape::Circ(this) => this.overlaps_rect(rect),
            Shape::Line(this) => rect.intersects(this),
            Shape::Rect(this) => this.overlaps_rect(rect),
            Shape::Vect(this) => rect.contains(this)
        }
    }

    pub fn intersects(&self, line: Line) -> bool {
        match *self {
            Shape::Circ(this) => this.intersects(line),
            Shape::Line(this) => line.intersects(this),
            Shape::Rect(this) => this.intersects(line),
            Shape::Vect(this) => line.contains(this)
        }
    }

    pub fn contains(&self, vec: Vector) -> bool {
        match *self {
            Shape::Circ(this) => this.contains(vec),
            Shape::Line(this) => this.contains(vec),
            Shape::Rect(this) => this.contains(vec),
            Shape::Vect(this) => this == vec
        }
    }

    pub fn overlaps(&self, shape: Shape) -> bool {
        match *self {
            Shape::Circ(this) => shape.overlaps_circ(this),
            Shape::Line(this) => shape.intersects(this),
            Shape::Rect(this) => shape.overlaps_rect(this),
            Shape::Vect(this) => shape.contains(this)
        }
    }

    pub fn translate(&self, vec: Vector) -> Shape {
        match *self {
            Shape::Circ(this) => Shape::Circ(this.translate(vec)),
            Shape::Line(this) => Shape::Line(this.translate(vec)),
            Shape::Rect(this) => Shape::Rect(this.translate(vec)),
            Shape::Vect(this) => Shape::Vect(this + vec)
        }
    }

    pub fn with_center(&self, vec: Vector) -> Shape {
        self.translate(vec - self.center())
    }

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

    pub fn center(&self) -> Vector {
        match *self {
            Shape::Circ(this) => this.center(),
            Shape::Line(this) => (this.start + this.end) / 2,
            Shape::Rect(this) => this.center(),
            Shape::Vect(this) => this
        }
    }
}
