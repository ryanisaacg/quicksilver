use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::cmp::{Eq, PartialEq};

pub const FLOAT_LIMIT: f32 = 0.000001f32;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32
}

impl Vector {
   pub fn new(x: f32, y: f32) -> Vector {
       Vector { x: x, y: y }
   }
    
   pub fn len2(self) -> f32 {
       self.x * self.x + self.y * self.y
   }

   pub fn len(self) -> f32 {
       self.len2().sqrt()
   }

   pub fn clamp(self, min_bound: Vector, max_bound: Vector) -> Vector {
       Vector::new(max_bound.x.min(min_bound.x.max(self.x)),
           max_bound.y.min(min_bound.y.max(self.y)))
   }

   pub fn cross(self, other: Vector) -> f32 {
       self.x * other.y - self.y * other.x
   }

   pub fn dot(self, other: Vector) -> f32 {
       self.x * other.x + self.y * other.y
   }

   pub fn normalize(self) -> Vector {
       self / self.len()
   }

   pub fn x_comp(self) -> Vector {
       Vector::new(self.x, 0f32)
   }

   pub fn y_comp(self) -> Vector {
       Vector::new(0f32, self.y)
   }

   pub fn recip(self) -> Vector {
       Vector::new(self.x.recip(), self.y.recip())
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

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        Vector::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) -> () {
        *self = *self / rhs;
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector::new(self.x * rhs, self.y * rhs)
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) -> () {
        *self = *self * rhs;
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() < FLOAT_LIMIT && (self.y - other.y).abs() < FLOAT_LIMIT
    }
}

impl Eq for Vector {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic() {
        let a = Vector::new(5f32, 10f32);
        let b = Vector::new(1f32, -2f32);
        assert!((a + b).x == 6f32);
        assert!((a - b).y == 12f32);
    }

    #[test]
    fn equality() {
        assert_eq!(Vector::new(5f32, 5f32), Vector::new(5f32, 5f32));
        assert_ne!(Vector::new(0f32, 5f32), Vector::new(5f32, 5f32));
    }

    #[test]
    fn inverse() {
        let vec = Vector::new(3f32, 5f32);
        let inverse = vec.recip();
        assert!((inverse.x - vec.x.recip()).abs() < FLOAT_LIMIT &&
                (inverse.y - vec.y.recip()).abs() < FLOAT_LIMIT);
    }

    #[test]
    fn length() {
        let vec = Vector::new(5f32, 0f32);
        assert!((vec.len2() - 25f32).abs() < FLOAT_LIMIT);
        assert!((vec.len() - 5f32).abs() < FLOAT_LIMIT);
    }

    #[test]
    fn scale() {
        let vec = Vector::new(1f32, 1f32);
        let doubled = Vector::new(2f32, 2f32);
        assert_eq!(vec * 2f32, doubled);
        let halved = Vector::new(0.5f32, 0.5f32);
        assert_eq!(vec / 2f32, halved);
    }

    #[test]
    fn clamp() {
        let min = Vector::new(-10f32, -2f32);
        let max = Vector::new(5f32, 6f32);
        let vec = Vector::new(-11f32, 3f32);
        let clamped = vec.clamp(min, max);
        let expected = Vector::new(-10f32, 3f32);
        assert_eq!(clamped, expected);
    }

    #[test]
    fn dot() {
        assert!((Vector::new(6f32, 5f32).dot(Vector::new(2f32, -8f32)) - 28f32) <= FLOAT_LIMIT);
    }
}
