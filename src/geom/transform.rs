use crate::geom::{about_equal, Rectangle, Vector};
use std::{
    cmp::{Eq, PartialEq},
    default::Default,
    f32::consts::PI,
    fmt,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

/// A 2D transformation represented by a matrix
///
/// Transforms can be composed together through matrix multiplication, and are applied to Vectors
/// through multiplication, meaning the notation used is the '*' operator. A property of matrix
/// multiplication is that for some matrices A, B, C and vector V is
/// ```text
/// Transform = A * B * C
/// Transform * V = A * (B * (C * V))
/// ```
///
/// This property allows encoding multiple transformations in a single matrix. A transformation
/// that involves rotating a shape 30 degrees and then moving it six units up could be written as
/// ```no_run
/// use quicksilver::geom::{Transform, Vector};
/// let transform = Transform::rotate(30.0) * Transform::translate(Vector::new(0.0, -6.0));
/// ```
/// and then applied to a Vector
/// ```no_run
/// # use quicksilver::geom::{Transform, Vector};
/// # let transform  = Transform::rotate(30.0) * Transform::translate(Vector::new(0.0, -6.0));
/// transform * Vector::new(5.0, 5.0)
/// # ;
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Transform([[f32; 3]; 3]);

impl Transform {
    /// The identity transformation
    pub const IDENTITY: Transform =
        Transform([[1f32, 0f32, 0f32], [0f32, 1f32, 0f32], [0f32, 0f32, 1f32]]);

    /// Create a rotation transformation
    pub fn rotate(angle: f32) -> Transform {
        let c = (angle * PI / 180f32).cos();
        let s = (angle * PI / 180f32).sin();
        Transform([[c, -s, 0f32], [s, c, 0f32], [0f32, 0f32, 1f32]])
    }

    /// Create a translation transformation
    pub fn translate(vec: Vector) -> Transform {
        Transform([[1f32, 0f32, vec.x], [0f32, 1f32, vec.y], [0f32, 0f32, 1f32]])
    }

    /// Create a scale transformation
    pub fn scale(vec: Vector) -> Transform {
        Transform([[vec.x, 0f32, 0f32], [0f32, vec.y, 0f32], [0f32, 0f32, 1f32]])
    }

    /// Create an orthographic projection
    pub fn orthographic(rect: Rectangle) -> Transform {
        Transform::translate(-rect.pos)
            .then(Transform::scale(rect.size.recip()))
            .then(Transform::translate(-Vector::ONE / 2.0))
            .then(Transform::scale(Vector::new(2.0, -2.0)))
    }

    pub fn then(self, next: Transform) -> Transform {
        next * self
    }

    /// Find the inverse of a Transform
    ///
    /// A transform's inverse will cancel it out when multplied with it, as seen below:
    /// ```
    /// # use quicksilver::geom::{Transform, Vector};
    /// let transform = Transform::translate(Vector::new(4.0, 5.0));
    /// let inverse = transform.inverse();
    /// let vector = Vector::new(10.0, 10.0);
    /// assert_eq!(vector, transform * inverse * vector);
    /// assert_eq!(vector, inverse * transform * vector);
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Transform {
        let det = self.0[0][0] * (self.0[1][1] * self.0[2][2] - self.0[2][1] * self.0[1][2])
            - self.0[0][1] * (self.0[1][0] * self.0[2][2] - self.0[1][2] * self.0[2][0])
            + self.0[0][2] * (self.0[1][0] * self.0[2][1] - self.0[1][1] * self.0[2][0]);

        let inv_det = det.recip();

        let mut inverse = Transform::IDENTITY;
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

/// Concat two transforms A and B such that A * B * v = A * (B * v)
impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        let mut returnval = Transform::IDENTITY;
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

/// Uses the `impl Mul<Transform> for Transform` internally.
impl MulAssign<Transform> for Transform {
    #[inline]
    fn mul_assign(&mut self, other: Transform) {
        *self = *self * other;
    }
}

/// Transform a vector
impl Mul<Vector> for Transform {
    type Output = Vector;

    #[inline]
    fn mul(self, other: Vector) -> Vector {
        Vector::new(
            other.x * self.0[0][0] + other.y * self.0[0][1] + self.0[0][2],
            other.x * self.0[1][0] + other.y * self.0[1][1] + self.0[1][2],
        )
    }
}

/// Scale all of the internal values of the Transform matrix
///
/// Note this will NOT scale vectors multiplied by this transform, and generally you shouldn't need
/// to use this.
impl Mul<f32> for Transform {
    type Output = Transform;

    fn mul(self, other: f32) -> Transform {
        let mut ret = Transform::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                ret.0[i][j] = self.0[i][j] * other;
            }
        }
        ret
    }
}

/// Uses the `impl Mul<f32> for Transform` internally.
impl MulAssign<f32> for Transform {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

/// Add the values of two transforms together
///
/// Note you probably want Mul to combine Transforms. Addition is only useful in less common use cases like interpolation
impl Add<Transform> for Transform {
    type Output = Transform;

    fn add(self, other: Transform) -> Transform {
        let mut returnval = Transform::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                returnval.0[i][j] = other.0[i][j] + self.0[i][j];
            }
        }
        returnval
    }
}

/// Uses the `impl Add<Transform> for Transform` internally
impl AddAssign<Transform> for Transform {
    #[inline]
    fn add_assign(&mut self, other: Transform) {
        *self = *self + other;
    }
}

/// Subtract the values of one transform from another
///
/// Note you probably want Mul to combine Transforms. Subtraction is only useful in less common use cases like interpolation
impl Sub<Transform> for Transform {
    type Output = Transform;

    fn sub(self, other: Transform) -> Transform {
        let mut returnval = Transform::IDENTITY;
        for i in 0..3 {
            for j in 0..3 {
                returnval.0[i][j] = self.0[i][j] - other.0[i][j];
            }
        }
        returnval
    }
}

/// Uses the `impl Sub<Transform> for Transform` internally
impl SubAssign<Transform> for Transform {
    #[inline]
    fn sub_assign(&mut self, other: Transform) {
        *self = *self - other;
    }
}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..3 {
            for j in 0..3 {
                write!(f, "{},", self.0[i][j])?;
            }
            writeln!(f)?;
        }
        write!(f, "]")
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Transform {
        Transform::IDENTITY
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Transform) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !about_equal(self.0[i][j], other.0[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Transform {}

impl From<[[f32; 3]; 3]> for Transform {
    #[inline]
    fn from(array: [[f32; 3]; 3]) -> Transform {
        Transform(array)
    }
}

impl From<Transform> for [[f32; 3]; 3] {
    #[inline]
    fn from(trans: Transform) -> [[f32; 3]; 3] {
        trans.0
    }
}

impl From<mint::RowMatrix3<f32>> for Transform {
    #[inline]
    fn from(mat: mint::RowMatrix3<f32>) -> Transform {
        let data: [f32; 9] = mat.into();
        Transform(bytemuck::cast(data))
    }
}

impl From<Transform> for mint::RowMatrix3<f32> {
    #[inline]
    fn from(trans: Transform) -> mint::RowMatrix3<f32> {
        let data: [f32; 9] = bytemuck::cast(trans.0);
        data.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(Transform::IDENTITY, Transform::IDENTITY);
        assert_eq!(Transform::rotate(5.0), Transform::rotate(5.0));
    }

    #[test]
    fn inverse() {
        let vec = Vector::new(2.0, 4.0);
        let translate = Transform::scale(Vector::ONE * 0.5);
        let inverse = translate.inverse();
        let transformed = inverse * vec;
        let expected = vec * 2.0;
        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale() {
        let trans = Transform::scale(Vector::ONE * 2.0);
        let vec = Vector::new(2.0, 5.0);
        let scaled = trans * vec;
        let expected = vec * 2.0;
        assert_eq!(scaled, expected);
    }

    #[test]
    fn translate() {
        let translate = Vector::new(3.0, 4.0);
        let trans = Transform::translate(translate);
        let vec = Vector::ONE;
        let translated = trans * vec;
        let expected = vec + translate;
        assert_eq!(translated, expected);
    }

    #[test]
    fn identity() {
        let trans = Transform::IDENTITY
            * Transform::translate(Vector::ZERO)
            * Transform::rotate(0.0)
            * Transform::scale(Vector::ONE);
        let vec = Vector::new(15.0, 12.0);
        assert_eq!(vec, trans * vec);
    }

    #[test]
    fn test_add() {
        let identity = Transform::IDENTITY;
        let trans = Transform::scale(Vector::ONE * 2.0);
        let double_trans = identity + identity;
        let vec = Vector::new(2.0, 2.0);
        let scaled = trans * vec;
        let doubled = double_trans * vec;
        assert_eq!(scaled, doubled);
    }

    #[test]
    fn test_sub() {
        let identity = Transform::IDENTITY;
        let double = identity + identity;
        let triple = double + identity;
        let right = triple - identity;
        assert_eq!(double, right);
    }
    #[test]
    fn complex_inverse() {
        let a = Transform::rotate(5.0)
            * Transform::scale(Vector::new(0.2, 1.23))
            * Transform::translate(Vector::ONE * 100.0);
        let a_inv = a.inverse();
        let vec = Vector::new(120.0, 151.0);
        assert_eq!(vec, a * a_inv * vec);
        assert_eq!(vec, a_inv * a * vec);
    }

    #[test]
    fn ortho() {
        let region = Rectangle::new(Vector::new(40.0, 40.0), Vector::new(50.0, 50.0));
        let view = Transform::orthographic(region);
        assert_eq!(view * region.pos, -Vector::X + Vector::Y);
        assert_eq!(
            view * (region.pos + region.size.y_comp()),
            -Vector::X + -Vector::Y
        );
        assert_eq!(view * (region.pos + region.size), Vector::X + -Vector::Y);
        assert_eq!(
            view * (region.pos + region.size.x_comp()),
            Vector::X + Vector::Y
        );
        assert_eq!(view * (region.pos + region.size / 2.0), Vector::ZERO);
    }
}
