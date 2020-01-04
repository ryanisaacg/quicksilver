use crate::geom::*;
use mint::*;
use vek::{Mat3, Vec2, Vec3};

pub fn orthographic(rect: Rect) -> ColumnMatrix3<f32> {
    let Rect { min, max } = rect;
    let min: Vec2<f32> = min.into();
    let max: Vec2<f32> = max.into();
    let mut size: Vec3<f32> = (max - min).into();
    size.z = 1.0;

    Mat3::translation_2d(-min)
        .scaled_3d(size.recip())
        .translated_2d(-Vec2::one() / 2.0)
        .scaled_3d(Vec3::new(2.0, -2.0, 1.0))
        .into()
}
