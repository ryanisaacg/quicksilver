use crate::geom::Vector;
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug)]
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
    #[deprecated(
        since = "0.4.0-alpha0.5",
        note = "Use another collision library like `vek` instead; please comment on issue #552 for use-cases other libraries don't solve"
    )]
    ///Create a triangle from `Vector`s of all three points
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle { a, b, c }
    }
    ///Calculate the area of the triangle
    pub fn area(self) -> f32 {
        // Heron's Formula
        ((self.b.x - self.a.x) * (self.c.y - self.a.y)
            - (self.c.x - self.a.x) * (self.b.y - self.a.y))
            .abs()
            / 2.0
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
