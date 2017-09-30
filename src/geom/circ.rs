use super::{Line, Rectangle, Vector};

#[derive(Debug, Clone, Copy)]
///A circle with a center and a radius
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32
}

impl Circle {
    pub fn new(x: f32, y: f32, radius: f32) -> Circle {
        Circle { x: x, y: y, radius: radius }
    }

    ///Get the center of a circle as a vector
    pub fn center(self) -> Vector {
        Vector { x: self.x, y: self.y }
    }

    ///Check to see if a circle contains a point
    pub fn contains(self, v: Vector) -> bool {
        (v - self.center()).len2() < self.radius * self.radius
    }

    ///Check to see if a circle intersects a line
    pub fn intersects(self, l: Line) -> bool {
        let center = self.center();
        let line = l.end - l.start;
        let dist = center - l.start;
        let nor_line = line.normalize();
        let product = dist.dot(nor_line);
        let check_point = if product <= 0f32 { l.start }
            else if product >= 1f32 { l.end }
            else { l.start + nor_line * product };
        (center - check_point).len2() < self.radius.powi(2)
    }

    ///Check if a circle overlaps a rectangle
    pub fn overlaps_rect(self, r: Rectangle) -> bool {
        r.overlaps_circ(self)
    }

    ///Check if two circles overlap
    pub fn overlaps_circ(self, c: Circle) -> bool {
        let x_diff = self.x - c.x;
        let y_diff = self.y - c.y;
        let radius = self.radius + c.radius;
        x_diff.powi(2) + y_diff.powi(2) < radius.powi(2)
    }

    ///Translate a circle by a given vector
    pub fn translate(self, v: Vector) -> Circle {
        Circle::new(self.x + v.x, self.y + v.y, self.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let circ = Circle::new(0f32, 0f32, 10f32);
        let vec1 = Vector::new(0f32, 0f32);
        let vec2 = Vector::new(11f32, 11f32);
        assert!(circ.contains(vec1));
        assert!(!circ.contains(vec2));
    }

    #[test]
    fn overlap() {
        let a = Circle::new(0f32, 0f32, 16f32);
        let b = Circle::new(5f32, 5f32, 4f32);
        let c = Circle::new(50f32, 50f32, 5f32);
        let d = Rectangle::new(10f32, 10f32, 10f32, 10f32);
        assert!(a.overlaps_circ(b));
        assert!(!a.overlaps_circ(c));
        assert!(a.overlaps_rect(d));
        assert!(!c.overlaps_rect(d));
    }

    #[test]
    fn rect_overlap() {
        let circ = Circle::new(0f32, 0f32, 5f32);
        let rec1 = Rectangle::new_sized(2f32, 2f32);
        let rec2 = Rectangle::new(5f32, 5f32, 4f32, 4f32);
        assert!(circ.overlaps_rect(rec1));
        assert!(rec1.overlaps_circ(circ));
        assert!(!circ.overlaps_rect(rec2));
        assert!(!rec2.overlaps_circ(circ));
    }

    #[test]
    fn intersects() {
        let line1 = Line::new(Vector::new(0f32, 0f32), Vector::new(32f32, 32f32));
        let line2 = Line::new(Vector::new(0f32, 32f32), Vector::new(32f32, 0f32));
        let line3 = Line::new(Vector::new(32f32, 32f32), Vector::new(64f32, 64f32));
        let line4 = Line::new(Vector::new(100f32, 100f32), Vector::new(1000f32, 1000f32));
        let circ = Circle::new(0f32, 0f32, 33f32);
        assert!(circ.intersects(line1));
        assert!(circ.intersects(line2));
        assert!(!circ.intersects(line3));
        assert!(!circ.intersects(line4));
    }

}
