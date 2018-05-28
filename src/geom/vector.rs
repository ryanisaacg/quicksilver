#[cfg(feature="nalgebra")] use nalgebra::{
    core::Vector2,
    geometry::Point2
};

use geom::{about_equal, Positioned, Rectangle, Scalar};
use rand::{
    Rng,
    distributions::{Distribution, Standard}
};
use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    cmp::{Eq, PartialEq},
    fmt
};

#[derive(Copy, Clone, Default, Debug, Deserialize, Serialize)]
///A 2D vector with an arbitrary numeric type
pub struct Vector {
    ///The x coordinate of the vector
    pub x: f32,
    ///The y coordinate of the vector
    pub y: f32,
}

impl Vector {
    ///The zero vector
    pub fn zero() -> Vector {
        Vector { x: 0f32, y: 0f32 }
    }

    ///A vector with x = 1f32, y = 0f32
    pub fn x() -> Vector {
        Vector { x: 1f32, y: 0f32 }
    }

    ///A vector with x = 0f32, y = 1f32
    pub fn y() -> Vector {
        Vector { x: 0f32, y: 1f32 }
    }

    ///A vector with x = 1f32, y = 1f32
    pub fn one() -> Vector {
        Vector { x: 1f32, y: 1f32 }
    }

    ///Create a new vector
    pub fn new<T: Scalar>(x: T, y: T) -> Vector {
        Vector { x: x.float(), y: y.float() }
    }

    ///Convert this vector into an nalgebra Vector2
    #[cfg(feature="nalgebra")]
    pub fn into_vector(self) -> Vector2<f32> {
        Vector2::new(self.x, self.y)
    }
   
    ///Convert this vector into an nalgebra Point2
    #[cfg(feature="nalgebra")]
    pub fn into_point(self) -> Point2<f32> {
        Point2::new(self.x, self.y)
    }

    ///Create a unit vector at a given angle
    pub fn from_angle<T: Scalar>(angle: T) -> Vector {
        Vector::new(angle.float().to_radians().cos(), angle.float().to_radians().sin())
    }

    ///Get the squared length of the vector (faster than getting the length)
    pub fn len2(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    ///Get the length of the vector
    pub fn len(self) -> f32 {
        self.len2().sqrt()
    }

    ///Clamp a vector somewhere between a minimum and a maximum
    pub fn clamp(self, min_bound: Vector, max_bound: Vector) -> Vector {
        Vector::new(
            max_bound.x.min(min_bound.x.max(self.x)),
            max_bound.y.min(min_bound.y.max(self.y)),
        )
    }

    ///Get the cross product of a vector
    pub fn cross(self, other: Vector) -> f32 {
        self.x * other.y - self.y * other.x
    }

    ///Get the dot product of a vector
    pub fn dot(self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y
    }

    ///Normalize the vector's length from [0, 1]
    pub fn normalize(self) -> Vector {
        self / self.len()
    }

    ///Get only the X component of the Vector, represented as a vector
    pub fn x_comp(self) -> Vector {
        Vector::new(self.x, 0f32)
    }

    ///Get only the Y component of the Vector, represented as a vector
    pub fn y_comp(self) -> Vector {
        Vector::new(0f32, self.y)
    }

    ///Get the vector equal to Vector(1 / x, 1 / y)
    pub fn recip(self) -> Vector {
        Vector::new(self.x.recip(), self.y.recip())
    }

    ///Multiply the components in the matching places
    pub fn times(self, other: Vector) -> Vector {
        Vector::new(self.x * other.x, self.y * other.y)
    }

    ///Get the angle a vector forms with the positive x-axis, counter clockwise
    pub fn angle(self) -> f32 {
        self.y.atan2(self.x).to_degrees()
    }

    ///Create a vector with the same angle and the given length
    pub fn with_len(self, length: f32) -> Vector {
        self.normalize() * length
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) -> () {
        *self = *self + rhs;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        self + (-rhs)
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Vector) -> () {
        *self = *self - rhs;
    }
}

impl<T: Scalar> Div<T> for Vector {
    type Output = Vector;

    fn div(self, rhs: T) -> Vector {
        let rhs = rhs.float();
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Scalar> DivAssign<T> for Vector {
    fn div_assign(&mut self, rhs: T) -> () {
        let rhs = rhs.float();
        *self = *self / rhs;
    }
}

impl<T: Scalar> Mul<T> for Vector {
    type Output = Vector;

    fn mul(self, rhs: T) -> Vector {
        let rhs = rhs.float();
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Scalar> MulAssign<T> for Vector {
    fn mul_assign(&mut self, rhs: T) -> () {
        let rhs = rhs.float();
        *self = *self * rhs;
    }
}


impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        about_equal(self.x, other.x) && about_equal(self.y, other.y)
    }
}

impl Eq for Vector {}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}>", self.x, self.y)
    }
}

impl Distribution<Vector> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rand: &mut R) -> Vector {
        Vector {
            x: self.sample(rand),
            y: self.sample(rand)
        }
    }
}

impl Positioned for Vector {
    fn center(&self) -> Vector {
        *self
    }
    
    fn bounding_box(&self) -> Rectangle {
        Rectangle::newv(*self, Vector::zero())
    }
}

#[cfg(feature="nalgebra")]
impl From<Vector2<f32>> for Vector {
    fn from(other: Vector2<f32>) -> Vector {
        Vector::new(other.x, other.y)
    }
}

#[cfg(feature="nalgebra")]
impl From<Point2<f32>> for Vector {
    fn from(other: Point2<f32>) -> Vector {
        Vector::new(other.x, other.y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        let a = Vector::new(5, 10);
        let b = Vector::new(1, -2);
        assert!((a + b).x == 6f32);
        assert!((a - b).y == 12f32);
    }

    #[test]
    fn equality() {
        assert_eq!(Vector::new(5, 5), Vector::new(5, 5));
        assert_ne!(Vector::new(0, 5), Vector::new(5, 5));
    }

    #[test]
    fn inverse() {
        let vec = Vector::new(3, 5);
        let inverse = vec.recip();
        assert_eq!(Vector::new(1.0 / 3.0, 1.0 / 5.0), inverse);
    }

    #[test]
    fn length() {
        let vec = Vector::x() * 5;
        assert!(about_equal(vec.len2(), 25f32));
        assert!(about_equal(vec.len(), 5f32));
    }

    #[test]
    fn scale() {
        let vec = Vector::new(1, 1);
        let doubled = Vector::new(2, 2);
        assert_eq!(vec * 2, doubled);
        let halved = Vector::new(0.5, 0.5);
        assert_eq!(vec / 2, halved);
    }

    #[test]
    fn clamp() {
        let min = Vector::new(-10, -2);
        let max = Vector::new(5, 6);
        let vec = Vector::new(-11, 3);
        let clamped = vec.clamp(min, max);
        let expected = Vector::new(-10, 3);
        assert_eq!(clamped, expected);
    }

    #[test]
    fn dot() {
        assert!(about_equal(Vector::new(6, 5).dot(Vector::new(2, -8)), -28f32));
    }

    #[test]
    fn times() {
        let vec = Vector::new(3, -2);
        let two = Vector::one() * 2;
        assert_eq!(vec.times(two), vec * 2);
    }

    #[test]
    fn angle() {
        let a = Vector::x();
        let b = Vector::y();
        let c = a + b;
        assert_eq!(a.angle(), 0.0);
        assert_eq!(b.angle(), 90.0);
        assert_eq!(c.angle(), 45.0);
    }
}
