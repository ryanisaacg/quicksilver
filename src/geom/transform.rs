#[cfg(feature="nalgebra")] use nalgebra::core::Matrix3;

use geom::{about_equal, Scalar, Vector};
use std::{
    ops::Mul,
    f32::consts::PI,
    fmt,
    default::Default,
    cmp::{Eq, PartialEq}
};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
///A 2D transformation represented by a matrix
pub struct Transform([[f32; 3]; 3]);

impl Transform {
    ///Create an identity transformation
    pub fn identity() -> Transform {
        Transform([[1f32, 0f32, 0f32],
                  [0f32, 1f32, 0f32],
                  [0f32, 0f32, 1f32]])
    }

    ///Create a rotation transformation
    pub fn rotate<T: Scalar>(angle: T) -> Transform {
        let angle = angle.float();
        let c = (angle * PI / 180f32).cos();
        let s = (angle * PI / 180f32).sin();
        Transform([[c, -s, 0f32],
                  [s, c, 0f32],
                  [0f32, 0f32, 1f32]])
    }

    ///Create a translation transformation
    pub fn translate(vec: Vector) -> Transform {
        Transform([[1f32, 0f32, vec.x],
                  [0f32, 1f32, vec.y],
                  [0f32, 0f32, 1f32]])
    }

    ///Create a scale transformation
    pub fn scale(vec: Vector) -> Transform {
        Transform([[vec.x, 0f32, 0f32],
                  [0f32, vec.y, 0f32],
                  [0f32, 0f32, 1f32]])
    }
 
    ///Find the inverse of a Transform
    pub fn inverse(&self) -> Transform {
        let det = 
            self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
            - self.0[0][1] * (self.0[1][0] * self.0[2][2] - self.0[1][2] * self.0[2][0])
            + self.0[0][2] * (self.0[1][0] * self.0[2][1] - self.0[1][1] * self.0[2][0]);

        let inv_det = det.recip();

        let mut inverse = Transform::identity();
        inverse.0[0][0] = self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2];
        inverse.0[0][1] = self.0[0][2] * self.0[2][1] - self.0[0][1] * self.0[2][2];
        inverse.0[0][2] = self.0[0][1] * self.0[1][2] - self.0[0][2] * self.0[1][1];
        inverse.0[1][0] = self.0[1][2] * self.0[2][0] - self.0[1][0] * self.0[2][2];
        inverse.0[1][1] = self.0[0][0] * self.0[2][2] - self.0[0][2] * self.0[2][0];
        inverse.0[1][2] = self.0[1][0] * self.0[0][2] - self.0[0][0] * self.0[1][2];
        inverse.0[2][0] = self.0[1][0] * self.0[2][1] - self.0[2][0] * self.0[1][1];
        inverse.0[2][1] = self.0[2][0] * self.0[0][1] - self.0[0][0] * self.0[2][1];
        inverse.0[2][2] = self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1];
        inverse * inv_det
    }
}

///Concat two transforms A and B such that A * B * v = A * (B * v)
impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        let mut returnval = Transform::identity();
        for i in 0..3 {
            for j in 0..3 {
                returnval.0[i][j] = 0f32;
                for k in 0..3 {
                    returnval.0[i][j] += other.0[k][j] * self.0[i][k];
                }
            }
        }
        returnval
    }
}

///Transform a vector
impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector::new(
            other.x * self.0[0][0] + other.y * self.0[0][1] + self.0[0][2],
            other.x * self.0[1][0] + other.y * self.0[1][1] + self.0[1][2],
        )
    }
}

impl Mul<f32> for Transform {
    type Output = Transform;

    fn mul(self, other: f32) -> Transform {
        let mut ret = Transform::identity();
        for i in 0..3 {
            for j in 0..3 {
                ret.0[i][j] = self.0[i][j] * other;
            }
        }
        ret
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..3 {
            for j in 0..3 {
                write!(f, "{},", self.0[i][j])?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::identity()
    }
}


impl PartialEq for Transform {
    fn eq(&self, other: &Transform) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if about_equal(self.0[i][j], other.0[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Transform {}

impl Into<Matrix3<f32>> for Transform {
    fn into(self) -> Matrix3<f32> {
        Matrix3::new(
            self.0[0][0], self.0[0][1], self.0[0][2],
            self.0[1][0], self.0[1][1], self.0[1][2],
            self.0[2][0], self.0[2][1], self.0[2][2],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse() {
        let vec = Vector::new(2, 4);
        let translate = Transform::scale(Vector::one() * 0.5);
        let inverse = translate.inverse();
        let transformed = inverse * vec;
        let expected = vec * 2;
        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale() {
        let trans = Transform::scale(Vector::one() * 2);
        let vec = Vector::new(2, 5);
        let scaled = trans * vec;
        let expected = vec * 2;
        assert_eq!(scaled, expected);
    }

    #[test]
    fn translate() {
        let translate = Vector::new(3, 4);
        let trans = Transform::translate(translate);
        let vec = Vector::one();
        let translated = trans * vec;
        let expected = vec + translate;
        assert_eq!(translated, expected);
    }

    #[test]
    fn identity() {
        let trans = Transform::identity() * Transform::translate(Vector::zero()) *
            Transform::rotate(0f32) * Transform::scale(Vector::one());
        let vec = Vector::new(15, 12);
        assert_eq!(vec, trans * vec);
    }

    #[test]
    fn complex_inverse() {
        let a = Transform::rotate(5f32) * Transform::scale(Vector::new(0.2, 1.23)) *
            Transform::translate(Vector::one() * 100f32);
        let a_inv = a.inverse();
        let vec = Vector::new(120f32, 151f32);
        assert_eq!(vec, a * a_inv * vec);
        assert_eq!(vec, a_inv * a * vec);
    }

}
