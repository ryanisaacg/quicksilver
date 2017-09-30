use super::Vector;
use std::ops::{Mul};
use std::f32::consts::PI;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    data: [f32; 9]
}

impl Transform {
    pub fn identity() -> Transform {
        Transform { data: 
            [1f32, 0f32, 0f32,
            0f32, 1f32, 0f32,
            0f32, 0f32, 1f32]
        }
    }

    pub fn rotate(angle: f32) -> Transform {
        let c = (angle * PI / 180f32).cos();
        let s = (angle * PI / 180f32).sin();
        Transform { data:
            [c, -s, 0f32,
            s, c, 0f32,
            0f32, 0f32, 1f32]
        }
    }

    pub fn translate(vec: Vector) -> Transform {
        Transform { data:
            [1f32, 0f32, vec.x,
            0f32, 1f32, vec.y,
            0f32, 0f32, 1f32]
        }
    }

    pub fn scale(vec: Vector) -> Transform {
        Transform { data:
            [vec.x, 0f32, 0f32,
            0f32, vec.y, 0f32,
            0f32, 0f32, 1f32]
        }
    }

    fn index(&self, x: usize, y: usize) -> f32 {
        self.data[x * 3 + y]
    }

    fn index_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        &mut self.data[x * 3 + y]
    }

    pub fn transpose(&self) -> Transform {
        Transform { data:
            [self.index(0, 0), self.index(0, 1), self.index(0, 2),
            self.index(1, 0), self.index(1, 1), self.index(1, 2),
            self.index(2, 0), self.index(2, 1), self.index(2, 2)]
        }
    }

    fn submatrix(&self, x: usize, y: usize) -> [f32; 4] {
        let mut matrix = [0f32, 0f32, 0f32, 0f32];
        let mut index = 0;
        for i in 0..3 {
            for j in 0..3 {
                if i != x && j != y {
                    matrix[index] = self.index(i, j);
                    index += 1;
                }
            }
        }
        matrix
    }

    fn sub_determinant(&self, x: usize, y: usize) -> f32 {
        let sub = self.submatrix(x, y);
        sub[0] * sub[3] - sub[1] * sub[2]
    }

    fn determinant(&self) -> f32 {
        let mut sum = 0f32;
        for i in 0..3 {
            sum += self.index(i, 0) * self.sub_determinant(i, 0);
        }
        sum
    }

    pub fn inverse(&self) -> Transform {
        let mut other = *self;
        //Find the matrix of minors
        for i in 0..3 {
            for j in 0..3 {
                *other.index_mut(i, j) = self.sub_determinant(i, j);
            }
        }
        //Find the matrix of cofactors
        for i in 0..3 {
            for j in 0..3 {
                if i != j && !(i == 0 && j == 2) && !(i == 2 && j == 0) {
                    *other.index_mut(i, j) = -other.index(i, j);
                }
            }
        }
        //Find the adjutant
        other = other.transpose();
        other * (1f32 / self.determinant())
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
                    *returnval.index_mut(i, j) += self.index(k, j) * other.index(i, k);
                }
            }
        }
        returnval
    }
}

impl Mul<Vector> for Transform {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector::new(other.x * self.index(0, 0) + other.y * self.index(0, 1) + self.index(0, 2),
            other.x * self.index(1, 0) + other.y * self.index(1, 1) + self.index(1, 2))
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
        let vec = Vector::new(2f32, 4f32);
        let translate = Transform::scale(Vector::new(0.5f32, 0.5f32));
        let inverse = translate.inverse();
        let transformed = inverse * vec;
        let expected = Vector::new(4f32, 8f32);
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
        let trans = Transform::scale(Vector::new(2f32, 2f32));
        let vec = Vector::new(2f32, 5f32);
        let scaled = trans * vec;
        let expected = Vector::new(4f32, 10f32);
        assert_eq!(scaled, expected);
    }

    #[test]
    fn translate() {
        let trans = Transform::translate(Vector::new(3f32, 4f32));
        let vec = Vector::new(1f32, 1f32);
        let translated = trans * vec;
        let expected = Vector::new(4f32, 5f32);
        assert_eq!(translated, expected);
    }

    #[test]
    fn transformation() {
        let trans = Transform::identity()
            * Transform::translate(Vector::new(0f32, 0f32))
            * Transform::rotate(0f32)
            * Transform::scale(Vector::new(1f32, 1f32));
        let vec = Vector::new(15f32, 12f32);
        assert_eq!(vec, trans * vec);
    }
}

