use crate::geom::{about_equal, Vector};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug)]
///A rectangle with a top-left position and a size
pub struct Rectangle {
    ///The top-left coordinate of the rectangle
    pub pos: Vector,
    ///The width and height of the rectangle
    pub size: Vector,
}

impl Rectangle {
    ///Create a rectangle from a top-left vector and a size vector
    pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>) -> Rectangle {
        Rectangle {
            pos: pos.into(),
            size: size.into(),
        }
    }

    ///Create a rectangle at the origin with the given size
    pub fn new_sized(size: impl Into<Vector>) -> Rectangle {
        Rectangle {
            pos: Vector::ZERO,
            size: size.into(),
        }
    }

    ///Get the top left coordinate of the Rectangle
    pub fn top_left(&self) -> Vector {
        self.pos
    }

    ///Get the x-coordinate of the Rectangle
    ///(The origin of a Rectangle is at the top left)
    pub fn x(&self) -> f32 {
        self.pos.x
    }

    ///Get the y-coordinate of the Rectangle
    ///(The origin of a Rectangle is at the top left)
    pub fn y(&self) -> f32 {
        self.pos.y
    }

    ///Get the size of the Rectangle
    pub fn size(&self) -> Vector {
        self.size
    }

    ///Get the height of the Rectangle
    pub fn height(&self) -> f32 {
        self.size.y
    }

    ///Get the width of the Rectangle
    pub fn width(&self) -> f32 {
        self.size.x
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        about_equal(self.x(), other.pos.x)
            && about_equal(self.y(), other.pos.y)
            && about_equal(self.width(), other.size.x)
            && about_equal(self.height(), other.size.y)
    }
}

impl Eq for Rectangle {}

#[cfg(test)]
mod tests {
    use crate::geom::*;

    #[test]
    fn overlap() {
        let a = &Rectangle::new_sized((32, 32));
        let b = &Rectangle::new((16, 16), (32, 32));
        let c = &Rectangle::new((50, 50), (5, 5));
        assert!(a.overlaps(b));
        assert!(!a.overlaps(c));
    }

    #[test]
    fn contains() {
        let rect = Rectangle::new_sized((32, 32));
        let vec1 = Vector::new(5, 5);
        let vec2 = Vector::new(33, 1);
        assert!(rect.contains(vec1));
        assert!(!rect.contains(vec2));
    }

    #[test]
    fn constraint() {
        let constraint = &Rectangle::new_sized((10, 10));
        let a = Rectangle::new((-1, 3), (5, 5));
        let b = Rectangle::new((4, 4), (8, 3));
        let a = a.constrain(constraint);
        assert_eq!(a.top_left(), Vector::new(0, 3));
        let b = b.constrain(constraint);
        assert_eq!(b.top_left(), Vector::new(2, 4));
    }

    #[test]
    fn translate() {
        let a = Rectangle::new((10, 10), (5, 5));
        let v = Vector::new(1, -1);
        let translated = a.translate(v);
        assert_eq!(a.top_left() + v, translated.top_left());
    }
}
