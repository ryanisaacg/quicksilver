#[cfg(feature="ncollide2d")] use ncollide2d::{
    bounding_volume::AABB,
    shape::Cuboid
};
use geom::{about_equal, Circle, Positioned, Scalar, Vector};
use std::cmp::{Eq, PartialEq};
use graphics::Drawable;
use graphics::Vertex;
use geom::Transform;
use graphics::Color;
use graphics::GpuTriangle;
use graphics::Sprite;

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
///A rectangle with a top-left position and a size
pub struct Rectangle {
    ///The top-left x coordinate of the rectangle
    pub x: f32,
    ///The top-left y coordinate of the rectangle
    pub y: f32,
    ///The width of the rectangle
    pub width: f32,
    ///The height of the rectangle
    pub height: f32,
}

impl Rectangle {
    ///Create a positioned rectangle with dimensions
    pub fn new<T: Scalar>(x: T, y: T, width: T, height: T) -> Rectangle {
        Rectangle {
            x: x.float(),
            y: y.float(),
            width: width.float(),
            height: height.float(),
        }
    }

    ///Create a rectangle from a top-left vector and a size vector
    pub fn newv(pos: Vector, size: Vector) -> Rectangle {
        Rectangle::new(pos.x, pos.y, size.x, size.y)
    }

    ///Create a rectangle at the origin with the given size
    pub fn new_sized<T: Scalar>(width: T, height: T) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: width.float(),
            height: height.float()
        }
    }

    ///Create a rectangle at the origin with a size given by a Vector
    pub fn newv_sized(size: Vector) -> Rectangle {
        Rectangle::newv(Vector::zero(), size)
    }

    #[cfg(feature="ncollide2d")]
    ///Create a rectangle with a given center and Cuboid from ncollide
    pub fn from_cuboid(center: Vector, cuboid: &Cuboid<f32>) -> Rectangle {
        let half_size = cuboid.half_extents().clone().into();
        Rectangle::newv(center - half_size, half_size * 2)
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
        self.x < b.x + b.width && self.x + self.width > b.x && self.y < b.y + b.height &&
            self.y + self.height > b.y
    }

    ///Check if any of the area bounded by this rectangle is bounded by a circle
    pub fn overlaps_circ(self, c: Circle) -> bool {
        (c.center().clamp(self.top_left(), self.top_left() + self.size()) - c.center()).len2() < c.radius.powi(2)
    }

    ///Move the rectangle so it is entirely contained with another
    pub fn constrain(self, outer: Rectangle) -> Rectangle {
        Rectangle::newv(self.top_left().clamp(
            outer.top_left(), outer.top_left() + outer.size() - self.size()
        ), self.size())
    }

    ///Translate the rectangle by a given vector
    pub fn translate(self, v: Vector) -> Rectangle {
        Rectangle::new(self.x + v.x, self.y + v.y, self.width, self.height)
    }

    ///Create a rectangle with the same size at a given center
    pub fn with_center(self, v: Vector) -> Rectangle {
        self.translate(v - self.center())
    }
}

impl PartialEq for Rectangle {
    fn eq(&self, other: &Rectangle) -> bool {
        about_equal(self.x, other.x) && about_equal(self.y, other.y) && about_equal(self.width, other.width)
            && about_equal(self.height, other.height)
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

impl Drawable for Rectangle {
    fn get_vertices(&self) -> Vec<Vertex> {
        let trans = Transform::translate(self.top_left() + self.size() / 2)
            * Transform::translate(-self.size() / 2)
            * Transform::scale(self.size());
        let get_vertex = |v: Vector| {
            Vertex {
                pos: trans * v,
                tex_pos: None,
                col: Color::white(),
            }
        };
        vec![
            get_vertex(Vector::zero()),
            get_vertex(Vector::zero() + Vector::x()),
            get_vertex(Vector::zero() + Vector::one()),
            get_vertex(Vector::zero() + Vector::y()),
        ]
    }

    fn get_triangles(&self) -> Vec<GpuTriangle> {
        vec![
            GpuTriangle {
                z: 0.0,
                indices: [0, 1, 2],
                image: None,
            },
            GpuTriangle {
                z: 0.0,
                indices: [2, 3, 0],
                image: None,
            }
        ]
    }

    fn to_sprite(&self) -> Sprite {
        Sprite::new(*self)
    }
}

#[cfg(feature="ncollide2d")]
impl From<AABB<f32>> for Rectangle {
    fn from(other: AABB<f32>) -> Rectangle {
        Rectangle::newv(other.mins().clone().into(), other.maxs().clone().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap() {
        let a = Rectangle::new_sized(32, 32);
        let b = Rectangle::new(16, 16, 32, 32);
        let c = Rectangle::new(50, 50, 5, 5);
        assert!(a.overlaps_rect(b));
        assert!(!a.overlaps_rect(c));
    }

    #[test]
    fn contains() {
        let rect = Rectangle::new_sized(32, 32);
        let vec1 = Vector::new(5, 5);
        let vec2 = Vector::new(33, 1);
        assert!(rect.contains(vec1));
        assert!(!rect.contains(vec2));
    }

    #[test]
    fn constraint() {
        let constraint = Rectangle::new_sized(10, 10);
        let a = Rectangle::new(-1, 3, 5, 5);
        let b = Rectangle::new(4, 4, 8, 3);
        let a = a.constrain(constraint);
        assert_eq!(a.top_left(), Vector::new(0, 3));
        let b = b.constrain(constraint);
        assert_eq!(b.top_left(), Vector::new(2, 4));
    }

    #[test]
    fn translate() {
        let a = Rectangle::new(10, 10, 5, 5);
        let v = Vector::new(1, -1);
        let translated = a.translate(v);
        assert_eq!(a.top_left() + v, translated.top_left());
    }
}
