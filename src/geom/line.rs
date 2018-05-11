#[cfg(feature="ncollide2d")] use ncollide2d::shape::Segment;
use geom::{Positioned, Rectangle, Vector};
use rand::{Rand, Rng};

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Deserialize, Serialize)]
///Represents a 2D line segment
pub struct Line {
    ///One of the two points of the line segment
    pub start: Vector,
    ///The other of the two points of the line segment
    pub end: Vector,
}
impl Line {
    ///Create a line segment with two endpoints
    pub fn new(start: Vector, end: Vector) -> Line {
        Line {
            start: start,
            end: end,
        }
    }
   
    #[cfg(feature="ncollide2d")]
    ///Convert the line into an equivalent ncollide Segment2
    pub fn into_segment(self) -> Segment<f32> {
        Segment::new(self.start.into_point(), self.end.into_point())
    }

    ///Check if two line segments interact
    pub fn intersects(self, other: Line) -> bool {
        if self.start == other.start || self.end == other.end {
            true
        } else {
            //See https://stackoverflow.com/a/565282 for algorithm
            let p = self.start;
            let q = other.start;
            let r = self.end - self.start;
            let s = other.end - other.start;
            //t = (q - p) x s / (r x s)
            //u = (q - p) x r / (r x s)
            let u_numerator = (q - p).cross(r);
            let t_numerator = (q - p).cross(s);
            let denominator = r.cross(s);
            if denominator == 0f32 {
                if u_numerator == 0f32 {
                    false
                } else {
                    let t0 = (q - p).dot(r) / r.dot(r);
                    let t1 = t0 + s.dot(r) / r.dot(r);
                    (t0 >= 0f32 && t0 <= 1f32) || (t1 >= 0f32 && t1 <= 1f32) ||
                        (t0.signum() == t1.signum()) || t0 == 0f32 && t1 == 0f32
                }
            } else {
                let u = u_numerator / denominator;
                let t = t_numerator / denominator;
                t >= 0f32 && t <= 1f32 && u >= 0f32 && u <= 1f32
            }
        }
    }

    ///Check if a point falls on the line segment
    pub fn contains(self, other: Vector) -> bool {
        self.start == other || self.end == other || self.start + (other - self.start).normalize() * (self.end - self.start).len() == self.end
    }

    ///Create a line segment translated by a given vector
    pub fn translate(self, other: Vector) -> Line {
        Line::new(self.start + other, self.end + other)
    }
}

impl Positioned for Line {
    fn center(&self) -> Vector {
        (self.start + self.end) / 2
    }

    fn bounding_box(&self) -> Rectangle {
        let x = self.start.x.min(self.end.x);
        let y = self.start.y.min(self.end.y);
        Rectangle::new(x, y, self.start.x.max(self.end.x) - x, self.start.y.max(self.end.y) - y)
    }
}

impl Rand for Line {
    fn rand<R: Rng>(rand: &mut R) -> Self {
        Line::new(rand.gen(), rand.gen())
    }
}

#[cfg(feature="ncollide2d")]
impl From<Segment<f32>> for Line {
    fn from(other: Segment<f32>) -> Line {
        Line::new(other.a().clone().into(), other.b().clone().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect() {
        let line1 = Line::new(Vector::new(0, 0), Vector::new(32, 32));
        let line2 = Line::new(Vector::new(0, 32), Vector::new(32, 0));
        let line3 = Line::new(Vector::new(32, 32), Vector::new(64, 64));
        let line4 = Line::new(Vector::new(100, 100), Vector::new(1000, 1000));
        assert!(line1.intersects(line2));
        assert!(!line2.intersects(line3));
        assert!(!line1.intersects(line4));
        assert!(!line2.intersects(line4));
        assert!(!line3.intersects(line4));
    }

    #[test]
    fn contains() {
        let line1 = Line::new(Vector::new(0, 0), Vector::new(32, 32));
        let line2 = Line::new(Vector::new(0, 32), Vector::new(32, 0));
        assert!(line1.contains(Vector::new(5, 5)));
        assert!(!line1.contains(Vector::new(6, 5)));
        assert!(line2.contains(Vector::new(0, 32)));
    }
}
