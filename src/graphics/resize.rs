use geom::{Rectangle, Vector};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
///The way to adjust the content when the size of the window changes
pub enum ResizeStrategy {
    ///Use black bars to keep the size exactly the same
    Maintain,
    ///Fill the screen while maintaing aspect ratio, possiby cutting off content in the process
    Fill,
    ///Take up as much of the screen as possible while maintaing aspect ratio, but use letterboxing if necessary
    Fit,
    ///Ignore aspect ratio and just stretch the content
    Stretch
}

impl ResizeStrategy {
    ///Calculate the content offset and the content size
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
        };
        Rectangle::new((new_size - content_area) / 2, content_area)
    }

    pub(crate) fn get_window_size(self, screen_region: Rectangle) -> Vector {
        screen_region.size() + screen_region.top_left() * 2
    }
}

impl Default for ResizeStrategy {
    fn default() -> ResizeStrategy {
        ResizeStrategy::Fit
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
            BASE,
            BASE * 2,
            BASE.x_comp() * 2 + BASE.y_comp(),
            BASE.x_comp() + BASE.y_comp() * 2
        ];
        let maintain = [
            Rectangle::new_sized(BASE),
            Rectangle::new(BASE / 2, BASE),
            Rectangle::new(BASE.x_comp() / 2, BASE),
            Rectangle::new(BASE.y_comp() / 2, BASE),
        ];
        let fill = [
            Rectangle::new_sized(BASE),
            Rectangle::new_sized(BASE * 2),
            Rectangle::new(-BASE.y_comp() / 2, BASE * 2),
            Rectangle::new(-BASE.x_comp() / 2, BASE * 2)
        ];
        let fit = [
            Rectangle::new_sized(BASE),
            Rectangle::new_sized(BASE * 2),
            Rectangle::new(BASE.x_comp() / 2, BASE),
            Rectangle::new(BASE.y_comp() / 2, BASE)
        ];
        for i in 0..new.len() {
            test(ResizeStrategy::Maintain, new[i], maintain[i]);
            test(ResizeStrategy::Fill, new[i], fill[i]);
            test(ResizeStrategy::Fit, new[i], fit[i]);
            test(ResizeStrategy::Stretch, new[i], Rectangle::new_sized(new[i]));
        }
    }
}
