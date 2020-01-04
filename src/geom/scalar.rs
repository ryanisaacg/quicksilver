/// A Scalar type that can be converted to a component of a Vector
///
/// It does not necessarily have to be a lossless conversion, because the geometry module focuses
/// on ease of use and speed over extreme precision
pub trait Scalar: Copy {
    /// Cast the scalar into an f32, which is how a Vector represents it
    fn float(self) -> f32;
}

impl Scalar for u8 {
    fn float(self) -> f32 {
        self as f32
    }
}
impl Scalar for u16 {
    fn float(self) -> f32 {
        self as f32
    }
}
impl Scalar for u32 {
    fn float(self) -> f32 {
        self as f32
    }
}
impl Scalar for i8 {
    fn float(self) -> f32 {
        self as f32
    }
}
impl Scalar for i16 {
    fn float(self) -> f32 {
        self as f32
    }
}
impl Scalar for i32 {
    fn float(self) -> f32 {
        self as f32
    }
}
impl Scalar for f32 {
    fn float(self) -> f32 {
        self
    }
}
