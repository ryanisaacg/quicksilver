use geom::{Scalar, Transform};
use graphics::{Color, GpuTriangle, Vertex};

/// Some object that can be drawn to the screen
pub trait Drawable {
    /// Draw the object to the window
    fn draw(&self, &mut impl RenderTarget, DrawAttributes);
}

/// A target to draw objects to
pub trait RenderTarget {
    /// Add vertices directly to the list without using a Drawable
    ///
    /// Each vertex has a position in terms of the current view. The indices
    /// of the given GPU triangles are specific to these vertices, so that
    /// the index must be at least 0 and at most the number of vertices.
    /// Other index values will have undefined behavior
    fn add_vertices(&mut self, vertices: impl IntoIterator<Item = Vertex>, triangles: impl IntoIterator<Item = GpuTriangle>);

    /// Draw an object to this target
    fn draw(&mut self, item: &impl Drawable, attr: DrawAttributes) where Self: Sized {
        item.draw(self, attr);
    }

    /// Draw an object to this target
    fn draw_ex(&mut self, item: &impl Drawable, transform: Transform, color: Color, z: impl Scalar) where Self: Sized {
        item.draw(self, DrawAttributes {
            color,
            transform,
            z: z.float()
        });
    }
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
