use crate::geom::{about_equal, Vector};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///A rectangle with a top-left position and a size
pub struct Rectangle {
    ///The top-left coordinate of the rectangle
    pub pos: Vector,
    ///The width and height of the rectangle
    pub size: Vector,
}

impl Rectangle {
    ///Create a rectangle from a top-left vector and a size vector
    #[inline]
    pub fn new(pos: Vector, size: Vector) -> Rectangle {
        Rectangle { pos, size }
    }

    ///Create a rectangle at the origin with the given size
    #[inline]
    pub fn new_sized(size: Vector) -> Rectangle {
        Rectangle {
            pos: Vector::ZERO,
            size,
        }
    }

    ///Get the top left coordinate of the Rectangle
    #[inline]
    pub fn top_left(&self) -> Vector {
        self.pos
    }

    ///Get the x-coordinate of the Rectangle
    ///(The origin of a Rectangle is at the top left)
    #[inline]
    pub fn x(&self) -> f32 {
        self.pos.x
    }

    ///Get the y-coordinate of the Rectangle
    ///(The origin of a Rectangle is at the top left)
    #[inline]
    pub fn y(&self) -> f32 {
        self.pos.y
    }

    ///Get the size of the Rectangle
    #[inline]
    pub fn size(&self) -> Vector {
        self.size
    }

    ///Get the height of the Rectangle
    #[inline]
    pub fn height(&self) -> f32 {
        self.size.y
    }

    ///Get the width of the Rectangle
    #[inline]
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
        let a = &Rectangle::new_sized(Vector::new(32.0, 32.0));
        let b = &Rectangle::new(Vector::new(16.0, 16.0), Vector::new(32.0, 32.0));
        let c = &Rectangle::new(Vector::new(50.0, 50.0), Vector::new(5.0, 5.0));
        assert!(a.overlaps_rectangle(b));
        assert!(!a.overlaps_rectangle(c));
    }

    #[test]
    fn contains() {
        let rect = Rectangle::new_sized(Vector::new(32.0, 32.0));
        let vec1 = Vector::new(5.0, 5.0);
        let vec2 = Vector::new(33.0, 1.0);
        assert!(rect.contains(vec1));
        assert!(!rect.contains(vec2));
    }

    #[test]
    fn constraint() {
        let constraint = &Rectangle::new_sized(Vector::new(10.0, 10.0));
        let a = Rectangle::new(Vector::new(-1.0, 3.0), Vector::new(5.0, 5.0));
        let b = Rectangle::new(Vector::new(4.0, 4.0), Vector::new(8.0, 3.0));
        let a = a.constrain(constraint);
        assert_eq!(a.top_left(), Vector::new(0.0, 3.0));
        let b = b.constrain(constraint);
        assert_eq!(b.top_left(), Vector::new(2.0, 4.0));
    }

    #[test]
    fn translate() {
        let a = Rectangle::new(Vector::new(10.0, 10.0), Vector::new(5.0, 5.0));
        let v = Vector::new(1.0, -1.0);
        let translated = a.translate(v);
        assert_eq!(a.top_left() + v, translated.top_left());
    }
}
