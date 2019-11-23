use mint::Vector2;

pub struct Rect {
    pub min: Vector2<f32>,
    pub max: Vector2<f32>
}

pub struct Circle {
    pub center: Vector2<f32>,
    pub radius: f32
}

pub struct Line {
    pub a: Vector2<f32>,
    pub b: Vector2<f32>,
    pub thickness: f32,
}

