use crate::geom::Vector;

#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
/// The way to adjust the content when the size of the window changes
pub enum ResizeHandler {
    /// Use the entire area and stretch to fill it
    Stretch,
    /// Keep the content area exactly the same size. If necessary, content will be cut off
    Maintain { width: f32, height: f32 },
    /// Fill the screen while maintaing aspect ratio, possiby cutting off content in the process
    Fill {
        aspect_width: f32,
        aspect_height: f32,
    },
    /// Take up as much of the screen as possible while maintaing aspect ratio, but use letterboxing if necessary
    Fit {
        aspect_width: f32,
        aspect_height: f32,
    },
    /// Only scale as integer multiple of the given width and height
    ///
    /// 16, 9, for example, will allow any 16:9 viewport; 160, 90 will only allow 16:9 viewports
    /// that are divisible by 10
    IntegerScale {
        aspect_width: u32,
        aspect_height: u32,
    },
}

impl ResizeHandler {
    /// Determine the size of the content given a window size
    ///
    /// This depends on which ResizeStrategy is in use; check the documentation for each enum
    /// variant for more.
    pub fn content_size(self, size: Vector) -> Vector {
        use ResizeHandler::*;

        let is_fill = match self {
            Fill { .. } => true,
            _ => false,
        };

        // First find the size we actually want to draw to, given the total size
        // For example, for stretching, we just always use the entire screen
        // For Maintain, we always use the size provided
        match self {
            Stretch => size,
            Maintain { width, height } => Vector::new(width, height),
            Fill {
                aspect_width,
                aspect_height,
            }
            | Fit {
                aspect_width,
                aspect_height,
            } => {
                let aspect_ratio = aspect_width / aspect_height;
                let window_ratio = size.x / size.y;
                if is_fill == (window_ratio < aspect_ratio) {
                    Vector::new(aspect_ratio * size.y, size.y)
                } else {
                    Vector::new(size.x, size.x / aspect_ratio)
                }
            }
            IntegerScale {
                aspect_width,
                aspect_height,
            } => {
                let aspect_width = aspect_width as f32;
                let aspect_height = aspect_height as f32;
                // Find the integer scale that fills the most amount of screen with no cut off
                // content
                Vector::new(aspect_width, aspect_height)
                    * int_scale(size.x / aspect_width).min(int_scale(size.y / aspect_height))
            }
        }
    }
}

// Find either the n or 1 / n where n is an integer
fn int_scale(value: f32) -> f32 {
    if value >= 1.0 {
        value.floor()
    } else {
        value.recip().floor().recip()
    }
}
