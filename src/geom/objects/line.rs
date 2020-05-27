use crate::geom::Vector;
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug)]
///A line with a starting and end point
pub struct Line {
    /// The starting point
    pub a: Vector,
    /// The end point
    pub b: Vector,
    /// The thickness, used only for rendering and not collision
    pub t: f32,
}

impl Line {
    #[deprecated(
        since = "0.4.0-alpha0.5",
        note = "Use another collision library like `vek` instead; please comment on issue #552 for use-cases other libraries don't solve"
    )]
    ///Create a new line with a start- and an endpoint
    pub fn new(start: Vector, end: Vector) -> Line {
        Line {
            a: start,
            b: end,
            t: 1.0,
        }
    }

    ///Create a line with a changed thickness
    #[must_use]
    pub fn with_thickness(self, thickness: f32) -> Line {
        Line {
            t: thickness,
            ..self
        }
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Line) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Eq for Line {}

