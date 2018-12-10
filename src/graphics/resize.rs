use geom::{Rectangle, Vector};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
/// The way to adjust the content when the size of the window changes
pub enum ResizeStrategy {
    /// Use black bars to keep the size exactly the same
    ///
    /// If necessary, content will be cut off
    Maintain,
    /// Fill the screen while maintaing aspect ratio, possiby cutting off content in the process
    Fill,
    /// Take up as much of the screen as possible while maintaing aspect ratio, but use letterboxing if necessary
    Fit,
    /// Ignore aspect ratio and just stretch the content
    Stretch,
    /// Only scale as integer multiple of the given width and height
    ///
    /// 16, 9, for example, will allow any 16:9 viewport; 160, 90 will only allow 16:9 viewports
    /// that are divisible by 10
    IntegerScale {
        #[allow(missing_docs)]
        width: u32,
        #[allow(missing_docs)]
        height: u32
    },
}

impl ResizeStrategy {
    /// Calculate the content offset and the content size
    pub(crate) fn resize(self, old_size: Vector, new_size: Vector) -> Rectangle {
        let content_area = match self {
            ResizeStrategy::Maintain => old_size,
            ResizeStrategy::Stretch => new_size,
            ResizeStrategy::Fill | ResizeStrategy::Fit => {
                let target_ratio = old_size.x / old_size.y;
                let window_ratio = new_size.x / new_size.y;
                if (self == ResizeStrategy::Fill) == (window_ratio < target_ratio) {
                    Vector::new(target_ratio * new_size.y, new_size.y)
                } else {
                    Vector::new(new_size.x, new_size.x / target_ratio)
                }
            }
            ResizeStrategy::IntegerScale { width, height } => {
                // Find the integer scale that fills the most amount of screen with no cut off
                // content
                Vector::new(width, height) * int_scale(new_size.x / width as f32).min(int_scale(new_size.y / height as f32))
            }
        };
        Rectangle::new((new_size - content_area) / 2, content_area)
    }

    pub(crate) fn get_window_size(self, screen_region: Rectangle) -> Vector {
        screen_region.size() + screen_region.top_left() * 2
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

impl Default for ResizeStrategy {
    fn default() -> ResizeStrategy {
        ResizeStrategy::Fit
    }
}

// TODO: Upgrade to Rust 2018 and use const fn for Replace::new so that the arrays can be const and
// outside the method
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
