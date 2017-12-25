#[derive(Clone, Copy, Debug, Default)]
///An RGBA color represented by normalized floats
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn white() -> Color {
        Color {
            r: 1f32,
            g: 1f32,
            b: 1f32,
            a: 1f32,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0f32,
            g: 0f32,
            b: 0f32,
            a: 1f32,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1f32,
            g: 0f32,
            b: 0f32,
            a: 1f32,
        }
    }

    pub fn orange() -> Color {
        Color {
            r: 1f32,
            g: 0.5f32,
            b: 0f32,
            a: 1f32,
        }
    }

    pub fn yellow() -> Color {
        Color {
            r: 1f32,
            g: 1f32,
            b: 0f32,
            a: 1f32,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0f32,
            g: 1f32,
            b: 0f32,
            a: 1f32,
        }
    }

    pub fn cyan() -> Color {
        Color {
            r: 0f32,
            g: 1f32,
            b: 1f32,
            a: 1f32,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0f32,
            g: 0f32,
            b: 1f32,
            a: 1f32,
        }
    }

    pub fn purple() -> Color {
        Color {
            r: 1f32,
            g: 0f32,
            b: 1f32,
            a: 1f32,
        }
    }

    pub fn indigo() -> Color {
        Color {
            r: 0.5f32,
            g: 0f32,
            b: 1f32,
            a: 1f32,
        }
    }
}
