use crate::geom::{Transform, Vector};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
#[non_exhaustive]
/// The way to adjust the content when the size of the window changes
pub enum ResizeHandler {
    /// Use the entire area and stretch to fill it
    Stretch,
    /// Keep the content area exactly the same size. If necessary, content will be cut off
    Maintain {
        width: u32,
        height: u32,
    },
    /// Fill the screen while maintaing aspect ratio, possiby cutting off content in the process
    Fill {
        target_width: u32,
        target_height: u32,
    },
    /// Take up as much of the screen as possible while maintaing aspect ratio, but use letterboxing if necessary
    Fit {
        target_width: u32,
        target_height: u32
    },
    /// Only scale as integer multiple of the given width and height
    ///
    /// 16, 9, for example, will allow any 16:9 viewport; 160, 90 will only allow 16:9 viewports
    /// that are divisible by 10
    IntegerScale {
        width: u32,
        height: u32,
    },
}

impl ResizeHandler {
    /// Create the projection to handle the given render target size
    ///
    /// When using a ResizeHandler, generally it is a good idea to listen for [resize events],
    /// calculate the projection with this method, and set it via [`Graphics::set_projection`]
    ///
    /// [resize events]: blinds::Event::Resized
    /// [`Graphics::set_projection`]: super::Graphics::set_projection
    pub fn projection(&self, size: Vector) -> Transform {
        use ResizeHandler::*;

        let content_size = match self {
            Stretch => size,
            Maintain { width, height } => Vector::new(width as f32, height as f32),
            Fill { target_width, target_height } | Fit { target_width, target_height } => {
                let target_ratio = old_size.x / old_size.y;
                let window_ratio = new_size.x / new_size.y;
                if (self == ResizeStrategy::Fill) == (window_ratio < target_ratio) {
                    Vector::new(target_ratio * new_size.y, new_size.y)
                } else {
                    Vector::new(new_size.x, new_size.x / target_ratio)
                }
            },
            ResizeStrategy::IntegerScale { width, height } => {
                // Find the integer scale that fills the most amount of screen with no cut off
                // content
                Vector::new(width, height) * int_scale(new_size.x / width as f32).min(int_scale(new_size.y / height as f32))
            }
        };
        let region = Rectangle::new((size - content_size) / 2, content_size);

        Transform::orthographic(region)
    }
}
