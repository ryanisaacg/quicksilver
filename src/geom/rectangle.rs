#[cfg(feature="ncollide2d")] use ncollide2d::{
    bounding_volume::AABB,
    shape::Cuboid
};
use geom::{about_equal, Circle, Positioned, Transform, Vector};
use graphics::{DrawAttributes, Drawable, GpuTriangle, Vertex, Window};
use std::cmp::{Eq, PartialEq};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A rectangle with a top-left position and a size
pub struct Rectangle {
    ///The top-left coordinate of the rectangle
    pub pos: Vector,
    ///The width and height of the rectangle
    pub size: Vector
}

impl Rectangle {
    ///Create a rectangle from a top-left vector and a size vector
    pub fn new(pos: impl Into<Vector>, size: impl Into<Vector>) -> Rectangle {
        Rectangle {
            pos:  pos.into(),
            size: size.into()
        }
    }

    ///Create a rectangle at the origin with the given size
    pub fn new_sized(size: impl Into<Vector>) -> Rectangle {
        Rectangle {
            pos:  Vector::ZERO,
            size: size.into()
        }
    }

    #[cfg(feature="ncollide2d")]
    ///Create a rectangle with a given center and Cuboid from ncollide
    pub fn from_cuboid(center: impl Into<Vector>, cuboid: &Cuboid<f32>) -> Rectangle {
        let half_size = cuboid.half_extents().clone().into();
        Rectangle::new(center.into() - half_size, half_size * 2)
    }
   
    ///Convert this rect into an ncollide Cuboid2
    #[cfg(feature="ncollide2d")]
    pub fn into_cuboid(self) -> Cuboid<f32> {
        Cuboid::new((self.size() / 2).into_vector())
    }
    
    ///Convert this rect into an ncollide AABB2
    #[cfg(feature="ncollide2d")]
    pub fn into_aabb(self) -> AABB<f32> { 
        let min = self.top_left().into_point(); 
        let max = (self.top_left() + self.size()).into_point();
        AABB::new(min, max)
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

    ///Checks if a point falls within the rectangle
    pub fn contains(self, point: impl Into<Vector>) -> bool {
        let p = point.into();

        return p.x >= self.x()
            && p.y >= self.y()
            && p.x < self.x() + self.width()
            && p.y < self.y() + self.width()
    }

    ///Check if any of the area bounded by this rectangle is bounded by another
    pub fn overlaps_rect(self, b: Rectangle) -> bool {
        self.x() < b.pos.x + b.size.x && self.x() + self.width() > b.pos.x && self.y() < b.pos.y + b.size.y &&
            self.y() + self.height() > b.pos.y
    }

    ///Check if any of the area bounded by this rectangle is bounded by a circle
    pub fn overlaps_circ(self, c: Circle) -> bool {
        (c.center().clamp(self.top_left(), self.top_left() + self.size()) - c.center()).len2() < c.radius.powi(2)
    }

    ///Move the rectangle so it is entirely contained with another
    #[must_use]
    pub fn constrain(self, outer: Rectangle) -> Rectangle {
        Rectangle::new(self.top_left().clamp(
            outer.top_left(), outer.top_left() + outer.size() - self.size()
        ), self.size())
    }

    ///Translate the rectangle by a given vector
    #[must_use]
    pub fn translate(self, v: impl Into<Vector>) -> Rectangle {
        Rectangle::new(self.pos + v.into(), self.size)
    }

    ///Create a rectangle with the same size at a given center
    #[must_use]
    pub fn with_center(self, v: impl Into<Vector>) -> Rectangle {
        self.translate(v.into() - self.center())
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        about_equal(self.x(), other.pos.x) && about_equal(self.y(), other.pos.y) && about_equal(self.width(), other.size.x)
            && about_equal(self.height(), other.size.y)
    }
}

impl Eq for Rectangle {}

impl Positioned for Rectangle {
    fn center(&self) -> Vector {
        self.top_left() + self.size() / 2
    }

    fn bounding_box(&self) -> Rectangle {
        *self
    }
}

#[cfg(feature="ncollide2d")]
impl From<AABB<f32>> for Rectangle {
    fn from(other: AABB<f32>) -> Rectangle {
        Rectangle::new(other.mins().clone(), other.maxs().clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, window: &mut Window, params: DrawAttributes) {
        let trans = Transform::translate(self.top_left() + self.size() / 2)
            * params.transform
            * Transform::translate(-self.size() / 2)
            * Transform::scale(self.size());
        let vertices = &[
            Vertex::new_untextured(trans * Vector::ZERO, params.color),
            Vertex::new_untextured(trans * Vector::X, params.color),
            Vertex::new_untextured(trans * Vector::ONE, params.color),
            Vertex::new_untextured(trans * Vector::Y, params.color),
        ];
        let triangles = &[
            GpuTriangle::new_untextured([0, 1, 2], params.z),
            GpuTriangle::new_untextured([2, 3, 0], params.z)
        ];
        window.add_vertices(vertices.iter().cloned(), triangles.iter().cloned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap() {
        let a = Rectangle::new_sized((32, 32));
        let b = Rectangle::new((16, 16), (32, 32));
        let c = Rectangle::new((50, 50), (5, 5));
        assert!(a.overlaps_rect(b));
        assert!(!a.overlaps_rect(c));
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
        let constraint = Rectangle::new_sized((10, 10));
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
