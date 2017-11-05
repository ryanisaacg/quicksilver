#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub mod Colors {
    use super::Color;
    pub const WHITE: Color = Color {
        r: 1f32,
        g: 1f32,
        b: 1f32,
        a: 1f32,
    };
    pub const BLACK: Color = Color {
        r: 0f32,
        g: 0f32,
        b: 0f32,
        a: 1f32,
    };
    pub const RED: Color = Color {
        r: 1f32,
        g: 0f32,
        b: 0f32,
        a: 1f32,
    };
    pub const ORANGE: Color = Color {
        r: 1f32,
        g: 0.5f32,
        b: 0f32,
        a: 1f32,
    };
    pub const YELLOW: Color = Color {
        r: 1f32,
        g: 1f32,
        b: 0f32,
        a: 1f32,
    };
    pub const GREEN: Color = Color {
        r: 0f32,
        g: 1f32,
        b: 0f32,
        a: 1f32,
    };
    pub const CYAN: Color = Color {
        r: 0f32,
        g: 1f32,
        b: 1f32,
        a: 1f32,
    };
    pub const BLUE: Color = Color {
        r: 0f32,
        g: 0f32,
        b: 1f32,
        a: 1f32,
    };
    pub const PURPLE: Color = Color {
        r: 1f32,
        g: 0f32,
        b: 1f32,
        a: 1f32,
    };
    pub const INDIGO: Color = Color {
        r: 0.5f32,
        g: 0f32,
        b: 1f32,
        a: 1f32,
    };
}
