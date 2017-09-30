mod vector;
use vector::{Vector, FLOAT_LIMIT};

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub start: Vector,
    pub end: Vector
}

impl Line {
    pub fn new(start: Vector, end: Vector) -> Line {
        Line { start: start, end: end }
    }

    pub fn intersects(self, other: Line) -> bool {
        let p = self.start;
        let q = other.start;
        let r = self.end - self.start;
        let s = other.end - other.start;
        //t = (q - p) x s / (r x s)
        //u = (q - p) x r / (r x s)
        let u_numerator = (q - p).cross(r);
        let t_numerator = (q - p).cross(s);
        let denominator = r.cross(s);
        if denominator == 0 {
            if u_numerator == 0 {
                false
            } else {
                let t0 = (q - p).dot(r) / r.dot(r);
                let t1 = t0 + s.dot(r) / r.dot(r);
                (t0 >= 0 && t0 <= 1) || (t1 >= 0 && t1 <= 1) 
                    || (t0.signum() == t1.signum()) || t0 == 0 && t1 == 0
            }
        } else {
            let u = u_numerator / denominator;
            let t = t_numerator / denominator;
            t >= 0 && t <= 1 && u >= 0 && u <= 1
        }
    }
}

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let line1 = Line::new(Vector::new(0f32, 0f32), Vector::new(32f32, 32f32));
        let line2 = Line::new(Vector::new(0f32, 32f32), Vector::new(32f32, 0f32));
        let line3 = Line::new(Vector::new(32f32, 32f32), Vector::new(64f32, 64f32));
        let line4 = Line::new(Vector::new(100f32, 100f32), Vector::new(1000f32, 1000f32));
        assert!(line1.intersects(line2));
        assert!(line1.intersects(line3));
        assert!(!line2.intersects(line3));
        assert!(!line1.intersects(line4));
        assert!(!line2.intersects(line4));
        assert!(!line3.intersects(line4));
    }
}
