#[derive(Clone, Copy, Debug, Default, PartialEq, Deserialize, Serialize)]
/// An RGBA color represented by normalized floats
pub struct Color {
    ///The red component of the color
    pub r: f32,
    ///The green component of the color
    pub g: f32,
    ///The blue component of the color
    pub b: f32,
    ///The alpha component of the color
    pub a: f32,
}

impl Color {
    ///Create an identical color with a different red component
    pub fn with_red(self, r: f32) -> Color {
        Color { r, ..self }
    }

    ///Create an identical color with a different green component
    pub fn with_green(self, g: f32) -> Color {
        Color { g, ..self }
    }

    ///Create an identical color with a different blue component
    pub fn with_blue(self, b: f32) -> Color {
        Color { b, ..self }
    }

    ///Create an identical color with a different alpha component
    pub fn with_alpha(self, a: f32) -> Color {
        Color { a, ..self }
    }

    /// Blend two colors by multiplying their components
    pub fn multiply(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
            a: self.a * other.a
        }
    }
}

#[allow(missing_docs)]
impl Color {
    pub const WHITE: Color =    Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Color =    Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Color =      Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const ORANGE: Color =   Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 };
    pub const YELLOW: Color =   Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const GREEN: Color =    Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Color =     Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLUE: Color =     Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Color =  Color { r: 1.0, g: 0.0, b: 0.5, a: 1.0 };
    pub const PURPLE: Color =   Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const INDIGO: Color =   Color { r: 0.5, g: 0.0, b: 1.0, a: 1.0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        let colors = [Color::WHITE, Color::BLACK, Color::RED, Color::ORANGE, Color::YELLOW,
            Color::GREEN, Color::CYAN, Color::BLUE, Color::PURPLE, Color::INDIGO];
        for i in 0..colors.len() {
            for j in 0..colors.len() {
                assert_eq!(i == j, colors[i].clone() == *&colors[j]);
            }
        }
        assert_eq!(Color::BLACK.with_red(1.0), Color::RED);
        assert_eq!(Color::BLACK.with_green(1.0), Color::GREEN);
        assert_eq!(Color::BLACK.with_blue(1.0), Color::BLUE);
    }
}
