use crate::geom::{Transform, Vector};

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
    /// calculate the projection with this method, and set it via [`Graphics::set_projection`].
    /// This transform should be applied after your 'normal' projection, which is probably a call
    /// to `Transform::orthographic`. If you wanted to draw your content at 800x600, and keep it at
    /// a 4:3 aspect ratio with optional letterboxing, it would look something like:
    ///
    /// ```no_run
    /// use quicksilver::graphics::{Graphics, ResizeHandler};
    /// use quicksilver::geom::{Rectangle, Vector, Transform};
    /// fn handle_resizes(gfx: &mut Graphics, my_screen_size: Vector) {
    ///     let handler = ResizeHandler::Fit {
    ///         aspect_width: 4.0,
    ///         aspect_height: 3.0,
    ///     };
    ///     let camera = Rectangle::new_sized(Vector::new(800.0, 600.0));
    ///     let projection = Transform::orthographic(camera);
    ///     let resize_adjustment = handler.projection(my_screen_size);
    ///     gfx.set_projection(resize_adjustment * projection);
    /// }
    /// ```
    ///
    /// [resize events]: crate::input::ResizedEvent
    /// [`Graphics::set_projection`]: super::Graphics::set_projection
    pub fn projection(self, size: Vector) -> Transform {
        ResizeHandler::transform_for_size(self.content_size(size), size)
    }

    /// Determine the size of the content given a window size
    ///
    /// This depends on which ResizeStrategy is in use; check the documentation for each enum
    /// variant for more.
    pub fn content_size(self, size: Vector) -> Vector {
        use ResizeHandler::*;

        let is_fill = match self { Fill { .. } => true, _ => false };

        // First find the size we actually want to draw to, given the total size
        // For example, for stretching, we just always use the entire screen
        // For Maintain, we always use the size provided
        match self {
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
                let aspect_width = aspect_width as f32;
                let aspect_height = aspect_height as f32;
                // Find the integer scale that fills the most amount of screen with no cut off
                // content
                Vector::new(aspect_width, aspect_height) * int_scale(size.x / aspect_width).min(int_scale(size.y / aspect_height))
            }
        }
    }

    /// Given a content size and a screen size, find a transformation that can be applied in
    /// GL-space to properly position it on screen.
    ///
    /// You can use this with the value returned from [`content_size`], but in that case you're
    /// better off just using [`projection`]. This is most useful if you are creating a custom
    /// resize strategy; it saves you having to muck around with the linear algebra. Just provide
    /// the size and this method will do the hard parts.
    ///
    /// [`content_size`]: ResizeHandler::content_size
    /// [`projection`]: ResizeHandler::projection
    pub fn transform_for_size(content_size: Vector, size: Vector) -> Transform {
        // We can easily calculate the position to offset our content_size window relative to the
        // larger window
        // However, this is is 'screen-space' coordinates. If we want to letterbox with 3 pixels of
        // space on each side of the content, we can't just translate with a Vector equal to <3, 0>
        // because the letterbox has to be applied *after* the initial projection. The letterbox
        // has to operate in GL-coordinates, which range from (-1, -1) to (1, 1). The code below
        // finds the offset and scale in GL coordinates necessary to provide our resize strategy
        let r_size = size.recip();
        let offset = (size - content_size).times(r_size).times(Vector::new(1.0, -1.0));
        let scale = content_size.times(r_size);

        // Once we have our offset and scale, we translate the scene so it stretches from (0, 0) to
        // (2, 2). This allows us to scale it without repositioning it. After scaling, we apply our
        // offset and undo our earlier translation. This forms a matrix that can be applied after
        // any projection that will letterbox correctly.
        let zero_start = Vector::new(-1.0, 1.0);
        Transform::translate(zero_start + offset) * Transform::scale(scale) * Transform::translate(-zero_start)
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

    #[test]
    fn transform_for_size() {
        use crate::geom::Rectangle;

        let screen = Vector::new(1600.0, 1200.0);
        let resize = ResizeHandler::Fit {
            aspect_width: 4.0,
            aspect_height: 3.0,
        };
        let content = Rectangle::new_sized(Vector::new(800.0, 600.0));
        let projection = Transform::orthographic(content);
        let projection = resize.projection(screen) * projection;

        assert_eq!(projection * Vector::ZERO, Vector::new(-1.0, 1.0));
        assert_eq!(projection * content.size(), Vector::new(1.0, -1.0));
    }
}
