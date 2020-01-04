// TODO: optional serde
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
            a: self.a * other.a,
        }
    }

    /// Create a color from a common RGBA definition
    pub fn from_rgba(red: u8, green: u8, blue: u8, a: f32) -> Color {
        Color {
            r: red as f32 / 255.0,
            g: green as f32 / 255.0,
            b: blue as f32 / 255.0,
            a,
        }
    }

    /// Create a color from a hexadecimal code
    pub fn from_hex(hex: &str) -> Color {
        let trimmed_hex = hex.trim_start_matches('#');
        match trimmed_hex.len() {
            3 => {
                let longer_hex: Vec<String> = trimmed_hex
                    .chars()
                    .map(|single_char| single_char.to_string().repeat(2))
                    .collect();
                Color::from_hex(&longer_hex.concat())
            }
            6 => {
                let red = u8::from_str_radix(&trimmed_hex[0..=1], 16).unwrap();
                let green = u8::from_str_radix(&trimmed_hex[2..=3], 16).unwrap();
                let blue = u8::from_str_radix(&trimmed_hex[4..=5], 16).unwrap();
                Color::from_rgba(red, green, blue, 1.0)
            }
            _ => panic!("Malformed hex string"),
        }
    }
}

#[allow(missing_docs)]
impl Color {
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const ORANGE: Color = Color {
        r: 1.0,
        g: 0.5,
        b: 0.0,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.5,
        a: 1.0,
    };
    pub const PURPLE: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const INDIGO: Color = Color {
        r: 0.5,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors() {
        let colors = [
            Color::WHITE,
            Color::BLACK,
            Color::RED,
            Color::ORANGE,
            Color::YELLOW,
            Color::GREEN,
            Color::CYAN,
            Color::BLUE,
            Color::PURPLE,
            Color::INDIGO,
        ];
        for i in 0..colors.len() {
            for j in 0..colors.len() {
                assert_eq!(i == j, colors[i].clone() == *&colors[j]);
            }
        }
        assert_eq!(Color::BLACK.with_red(1.0), Color::RED);
        assert_eq!(Color::BLACK.with_green(1.0), Color::GREEN);
        assert_eq!(Color::BLACK.with_blue(1.0), Color::BLUE);
    }

    #[test]
    fn colors_from_rgba() {
        assert_eq!(Color::BLACK.with_red(1.0), Color::from_rgba(255, 0, 0, 1.0));
        assert_eq!(
            Color::BLACK.with_green(1.0),
            Color::from_rgba(0, 255, 0, 1.0)
        );
        assert_eq!(
            Color::BLACK.with_blue(1.0),
            Color::from_rgba(0, 0, 255, 1.0)
        );
    }

    #[test]
    fn colors_from_hex() {
        assert_eq!(Color::BLACK.with_red(1.0), Color::from_hex("#FF0000"));
        assert_eq!(Color::BLACK.with_green(1.0), Color::from_hex("00FF00"));
        assert_eq!(Color::BLACK.with_blue(1.0), Color::from_hex("00f"));
        assert_eq!(Color::WHITE, Color::from_hex("#fff"));
    }

    #[test]
    #[should_panic(expected = "Malformed hex string")]
    fn colors_from_wrong_hex() {
        let _wrong_one = Color::from_hex("FF");
        let _wrong_two = Color::from_hex("FF00FF00");
    }
}
