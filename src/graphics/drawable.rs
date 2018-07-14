use geom::{Scalar, Transform};
use graphics::{Color, Window};

/// Some object that can be drawn to the screen
pub trait Drawable {
    /// Draw the object to the window
    fn draw(&self, window: &mut Window, params: DrawAttributes);
}

/// The attributes of a draw call
#[derive(Clone, Debug)]
pub struct DrawAttributes {
    /// The color for a draw call
    pub color: Color,
    /// The transformation matrix to apply to a draw call
    pub transform: Transform,
    /// The z-ordering of a draw call
    pub z: f32,
}

impl DrawAttributes {
    /// Create a new default instance of DrawAttributes
    pub fn new() -> DrawAttributes {
        DrawAttributes {
            color: Color::WHITE,
            transform: Transform::IDENTITY,
            z: 0.0
        }
    }
    
    /// Change the color
    pub fn with_color(self, color: Color) -> DrawAttributes {
        DrawAttributes {
            color,
            ..self
        }
    }

    /// Change the transform
    pub fn with_transform(self, transform: Transform) -> DrawAttributes {
        DrawAttributes {
            transform,
            ..self
        }
    }

    /// Change the depth
    pub fn with_z<T: Scalar>(self, z: T) -> DrawAttributes {
        DrawAttributes {
            z: z.float(),
            ..self
        }
    }
}
