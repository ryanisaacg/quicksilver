use super::{Circle, Line, Vector};

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

impl Rectangle {
    ///Create a positioned rectangle with dimensions
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
        Rectangle { x: x, y: y, width: width, height: height }
    }

    ///Create a rectangle at the origin with the given size
    pub fn new_sized(width: f32, height: f32) -> Rectangle {
        Rectangle::new(0f32, 0f32, width, height)
    }

    ///Get the top left coordinate of the Rectangle
    pub fn top_left(self) -> Vector {
        Vector::new(self.x, self.y)
    }

    ///Get the size of the Rectangle
    pub fn size(self) -> Vector {
        Vector::new(self.width, self.height)
    }

    ///Checks if a point falls within the rectangle
    pub fn contains(self, v: Vector) -> bool {
        v.x >= self.x && v.y >= self.y && v.x < self.x + self.width && v.y < self.y + self.height
    }

    ///Check if any of the area bounded by this rectangle is bounded by another
    pub fn overlaps_rect(self, b: Rectangle) -> bool {
        self.x < b.x + b.width && self.x + self.width > b.x 
            && self.y < b.y + b.height && self.y + self.height > b.y
    }

    ///Check if any of the area bounded by this rectangle is bounded by a circle
    pub fn overlaps_circ(self, c: Circle) -> bool {
        let closest = Vector::new(
            if c.x < self.x { self.x }
            else if c.x > self.x + self.width { self.x + self.width }
            else { c.x },
            if c.y < self.y { self.y }
            else if c.y > self.y + self.height { self.y + self.height }
            else { c.y });
        let closest = closest - c.center();
        closest.x.powi(2) + closest.y.powi(2) < c.radius.powi(2)
    }

    ///Move the rectangle so it is entirely contained with another
    pub fn constrain(self, outer: Rectangle) -> Rectangle {
        let top_left = self.top_left().clamp(outer.top_left(), outer.top_left() + outer.size() - self.size());
        Rectangle::new(top_left.x, top_left.y, self.width, self.height)
    }

    ///Translate the rectangle by a given vector
    pub fn translate(self, v: Vector) -> Rectangle {
        Rectangle::new(self.x + v.x, self.y + v.y, self.width, self.height)
    }

    ///Check if a line segment intersects a rectangle
    pub fn intersects(self, l: Line) -> bool {
        self.contains(l.start) || self.contains(l.end)
            || Line::new(self.top_left(), self.top_left() + self.size().x_comp()).intersects(l)
            || Line::new(self.top_left(), self.top_left() + self.size().y_comp()).intersects(l)
            || Line::new(self.top_left() + self.size().x_comp(), self.top_left() + self.size()).intersects(l)
            || Line::new(self.top_left() + self.size().y_comp(), self.top_left() + self.size()).intersects(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap() {
        let a = Rectangle::new_sized(32f32, 32f32);
        let b = Rectangle::new(16f32, 16f32, 32f32, 32f32);
        let c = Rectangle::new(50f32, 50f32, 5f32, 5f32);
        assert!(a.overlaps_rect(b));
        assert!(!a.overlaps_rect(c));
    }

    #[test]
    fn contains() {
        let rect = Rectangle::new_sized(32f32, 32f32);
        let vec1 = Vector::new(5f32, 5f32);
        let vec2 = Vector::new(33f32, 1f32);
        assert!(rect.contains(vec1));
        assert!(!rect.contains(vec2));
    }

    #[test]
    fn constraint() {
        let constraint = Rectangle::new_sized(10f32, 10f32);
        let a = Rectangle::new(-1f32, 3f32, 5f32, 5f32);
        let b = Rectangle::new(4f32, 4f32, 8f32, 3f32);
        let a = a.constrain(constraint);
        assert_eq!(a.top_left(), Vector::new(0f32, 3f32));
        let b = b.constrain(constraint);
        assert_eq!(b.top_left(), Vector::new(2f32, 4f32));
    }

    #[test]
    fn translate() {
        let a = Rectangle::new(10f32, 10f32, 5f32, 5f32);
        let v = Vector::new(1f32, -1f32);
        let translated = a.translate(v);
        assert_eq!(a.top_left() + v, translated.top_left());
    }

    #[test]
    fn intersect() {
        let line1 = Line::new(Vector::new(0f32, 0f32), Vector::new(32f32, 32f32));
        let line2 = Line::new(Vector::new(0f32, 32f32), Vector::new(32f32, 0f32));
        let line3 = Line::new(Vector::new(32f32, 32f32), Vector::new(64f32, 64f32));
        let line4 = Line::new(Vector::new(100f32, 100f32), Vector::new(1000f32, 1000f32));
        let rect = Rectangle::new_sized(32f32, 32f32);
        assert!(rect.intersects(line1));
        assert!(rect.intersects(line2));
        assert!(rect.intersects(line3));
        assert!(!rect.intersects(line4));
    }
}


