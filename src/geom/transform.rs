use super::Vector;
use std::ops::Mul;
use std::f32::consts::PI;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    data: [f32; 9],
}

impl Transform {
    pub fn identity() -> Transform {
        Transform { data: [1f32, 0f32, 0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 1f32] }
    }

    pub fn rotate(angle: f32) -> Transform {
        let c = (angle * PI / 180f32).cos();
        let s = (angle * PI / 180f32).sin();
        Transform { data: [c, -s, 0f32, s, c, 0f32, 0f32, 0f32, 1f32] }
    }

    pub fn translate(vec: Vector) -> Transform {
        Transform { data: [1f32, 0f32, vec.x, 0f32, 1f32, vec.y, 0f32, 0f32, 1f32] }
    }

    pub fn scale(vec: Vector) -> Transform {
        Transform { data: [vec.x, 0f32, 0f32, 0f32, vec.y, 0f32, 0f32, 0f32, 1f32] }
    }

    fn index(&self, x: usize, y: usize) -> f32 {
        self.data[x * 3 + y]
    }

    fn index_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        &mut self.data[x * 3 + y]
    }

    pub fn transpose(&self) -> Transform {
        Transform {
            data: [
                self.index(0, 0),
                self.index(0, 1),
                self.index(0, 2),
                self.index(1, 0),
                self.index(1, 1),
                self.index(1, 2),
                self.index(2, 0),
                self.index(2, 1),
                self.index(2, 2),
            ],
        }
    }
    
    pub fn inverse(&self) -> Transform {
        let det = 
            self.index(0, 0) * (self.index(1, 1) * self.index(2, 2) - self.index(2, 1) * self.index(1, 2))
            - self.index(0, 1) * (self.index(1, 0) * self.index(2, 2) - self.index(1, 2) * self.index(2, 0))
            + self.index(0, 2) * (self.index(1, 0) * self.index(2, 1) - self.index(1, 1) * self.index(2, 0));

        let inv_det = det.recip();

        let mut inverse = Transform::identity();
        *inverse.index_mut(0, 0) = self.index(1, 1) * self.index(2, 2) - self.index(2, 1) * self.index(1, 2);
        *inverse.index_mut(0, 1) = self.index(0, 2) * self.index(2, 1) - self.index(0, 1) * self.index(2, 2);
        *inverse.index_mut(0, 2) = self.index(0, 1) * self.index(1, 2) - self.index(0, 2) * self.index(1, 1);
        *inverse.index_mut(1, 0) = self.index(1, 2) * self.index(2, 0) - self.index(1, 0) * self.index(2, 2);
        *inverse.index_mut(1, 1) = self.index(0, 0) * self.index(2, 2) - self.index(0, 2) * self.index(2, 0);
        *inverse.index_mut(1, 2) = self.index(1, 0) * self.index(0, 2) - self.index(0, 0) * self.index(1, 2);
        *inverse.index_mut(2, 0) = self.index(1, 0) * self.index(2, 1) - self.index(2, 0) * self.index(1, 1);
        *inverse.index_mut(2, 1) = self.index(2, 0) * self.index(0, 1) - self.index(0, 0) * self.index(2, 1);
        *inverse.index_mut(2, 2) = self.index(0, 0) * self.index(1, 1) - self.index(1, 0) * self.index(0, 1);
        inverse * inv_det
    }

    pub fn get_array(&self) -> &[f32] {
        &self.data
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        let mut returnval = Transform::identity();
        for i in 0..3 {
            for j in 0..3 {
                *returnval.index_mut(i, j) = 0f32;
                for k in 0..3 {
                    *returnval.index_mut(i, j) += other.index(k, j) * self.index(i, k);
                }
            }
        }
        returnval
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector::new(
            other.x * self.index(0, 0) + other.y * self.index(0, 1) + self.index(0, 2),
            other.x * self.index(1, 0) + other.y * self.index(1, 1) + self.index(1, 2),
        )
    }
}

impl Mul<f32> for Transform {
    type Output = Transform;

    fn mul(self, other: f32) -> Transform {
        let mut ret = Transform::identity();
        for i in 0..3 {
            for j in 0..3 {
                *ret.index_mut(i, j) = self.index(i, j) * other;
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
                write!(f, "{},", self.index(i, j))?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse() {
        let vec = Vector::newi(2, 4);
        let translate = Transform::scale(Vector::one() * 0.5);
        let inverse = translate.inverse();
        let transformed = inverse * vec;
        let expected = vec * 2;
        for i in 0..3 {
            for j in 0..3 {
                print!("{},", inverse.index(i, j));
            }
            print!("\n");
        }
        assert_eq!(transformed, expected);
    }

    #[test]
    fn scale() {
        let trans = Transform::scale(Vector::one() * 2);
        let vec = Vector::newi(2, 5);
        let scaled = trans * vec;
        let expected = vec * 2;
        assert_eq!(scaled, expected);
    }

    #[test]
    fn translate() {
        let translate = Vector::newi(3, 4);
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
        let vec = Vector::newi(15, 12);
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
