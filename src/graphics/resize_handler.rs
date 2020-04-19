use crate::geom::{Rectangle, Transform, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
/// The way to adjust the content when the size of the window changes
pub enum ResizeHandler {
    /// Use the entire area and stretch to fill it
    Stretch,
    /// Keep the content area exactly the same size. If necessary, content will be cut off
    Maintain {
        width: f32,
        height: f32,
    },
    /// Fill the screen while maintaing aspect ratio, possiby cutting off content in the process
    Fill {
        aspect_width: f32,
        aspect_height: f32,
    },
    /// Take up as much of the screen as possible while maintaing aspect ratio, but use letterboxing if necessary
    Fit {
        aspect_width: f32,
        aspect_height: f32
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
    /// Create the projection to handle the given render target size
    ///
    /// When using a ResizeHandler, generally it is a good idea to listen for [resize events],
    /// calculate the projection with this method, and set it via [`Graphics::set_projection`]
    ///
    /// [resize events]: blinds::Event::Resized
    /// [`Graphics::set_projection`]: super::Graphics::set_projection
    pub fn projection(self, size: Vector) -> Transform {
        use ResizeHandler::*;

        let is_fill = match self { Fill { .. } => true, _ => false };

        let content_size = match self {
            Stretch => size,
            Maintain { width, height } => Vector::new(width, height),
            Fill { aspect_width, aspect_height } | Fit { aspect_width, aspect_height } => {
                let aspect_ratio = aspect_width / aspect_height;
                let window_ratio = size.x / size.y;
                if is_fill  == (window_ratio < aspect_ratio) {
                    Vector::new(aspect_ratio * size.y, size.y)
                } else {
                    Vector::new(size.x, size.x / aspect_ratio)
                }
            },
            IntegerScale { aspect_width, aspect_height } => {
                // Find the integer scale that fills the most amount of screen with no cut off
                // content
                Vector::new(aspect_width, aspect_height) * int_scale(size.x / aspect_width as f32).min(int_scale(size.y / aspect_height as f32))
            }
        };
        let offset = (size - content_size) / 2;
        let scale = content_size.times(size.recip());

        Transform::orthographic(Rectangle::new_sized(size)) * Transform::translate(offset) * Transform::scale(scale)
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

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: Vector = Vector { x: 16.0, y: 9.0 };

    fn test(resize: ResizeStrategy, new: Vector, expected: Rectangle) {
        assert_eq!(resize.resize(BASE, new), expected);
        assert_eq!(resize.resize(expected.size(), BASE), Rectangle::new_sized(BASE));
    }

    #[test]
    fn resize() {
        let new = [
            BASE / 2,
            BASE,
            BASE * 2,
            BASE.x_comp() * 2 + BASE.y_comp(),
            BASE.x_comp() + BASE.y_comp() * 2
        ];
        let maintain = [
            Rectangle::new(-BASE / 4, BASE),
            Rectangle::new_sized(BASE),
            Rectangle::new(BASE / 2, BASE),
            Rectangle::new(BASE.x_comp() / 2, BASE),
            Rectangle::new(BASE.y_comp() / 2, BASE),
        ];
        let fill = [
            Rectangle::new_sized(BASE / 2),
            Rectangle::new_sized(BASE),
            Rectangle::new_sized(BASE * 2),
            Rectangle::new(-BASE.y_comp() / 2, BASE * 2),
            Rectangle::new(-BASE.x_comp() / 2, BASE * 2)
        ];
        let fit = [
            Rectangle::new_sized(BASE / 2),
            Rectangle::new_sized(BASE),
            Rectangle::new_sized(BASE * 2),
            Rectangle::new(BASE.x_comp() / 2, BASE),
            Rectangle::new(BASE.y_comp() / 2, BASE)
        ];
        let scale = [
            Rectangle::new_sized(BASE / 2),
            Rectangle::new_sized(BASE),
            Rectangle::new_sized(BASE * 2),
            Rectangle::new(BASE.x_comp() / 2, BASE),
            Rectangle::new(BASE.y_comp() / 2, BASE),
        ];
        for i in 0..new.len() {
            test(ResizeStrategy::Maintain, new[i], maintain[i]);
            test(ResizeStrategy::Fill, new[i], fill[i]);
            test(ResizeStrategy::Fit, new[i], fit[i]);
            test(ResizeStrategy::Stretch, new[i], Rectangle::new_sized(new[i]));
            test(ResizeStrategy::IntegerScale { width: 16, height: 9 }, new[i], scale[i]);
        }
    }
}
