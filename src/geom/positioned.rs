use geom::{Rectangle, Vector};

pub trait Positioned {
    fn center(&self) -> Vector;
    fn bounding_box(&self) -> Rectangle;
}
