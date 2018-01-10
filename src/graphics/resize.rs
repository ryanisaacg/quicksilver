use geom::Vector;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
///The way to adjust the content when the size of the window changes
pub enum ResizeStrategy {
    ///Use black bars to keep the size exactly the same
    Maintain,
    ///Fill the screen, possiby cutting off content in the process
    Fill,
    ///Take up as much of the screen as possible, but use letterboxing if necessary
    Fit
}

impl ResizeStrategy {
    ///Calculate the content offset and the content size
    pub(crate) fn resize(self, old_size: Vector, new_size: Vector) -> (Vector, Vector) {
        let target_ratio = old_size.x / old_size.y; 
        let window_ratio = new_size.x / new_size.y;
        let content_area = match self {
            ResizeStrategy::Maintain => old_size,
            ResizeStrategy::Fill | ResizeStrategy::Fit => {
                if (self == ResizeStrategy::Fill) == (window_ratio < target_ratio) {
                    Vector::new((target_ratio * new_size.y), new_size.y)
                } else {
                    Vector::new(new_size.x, (new_size.x / target_ratio))
                } 
            }
        };
        ((new_size - content_area) / 2, content_area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        //Format: (old_size, new_size)
        let base = Vector::newi(16, 9);
        let inputs = [
            (base, base),
            (base, base.x_comp() * 2 + base.y_comp()),
            (base, base.x_comp() + base.y_comp() * 2),
        ];
        //Fomat: (offset, size)
        let maintain = [
            (Vector::zero(), base),
            (base.x_comp() / 2, base),
            (base.y_comp() / 2, base),
        ];
        let fill = [
            (Vector::zero(), base),
            (-base.y_comp() / 2, base * 2),
            (-base.x_comp() / 2, base * 2)
        ];
        let fit = [
            (Vector::zero(), base),
            (base.x_comp() / 2, base),
            (base.y_comp() / 2, base)
        ];
        for i in 0..inputs.len() {
            let (old, new) = inputs[i];
            assert_eq!(ResizeStrategy::Maintain.resize(old, new), maintain[i]);
            assert_eq!(ResizeStrategy::Fill.resize(old, new), fill[i]);
            assert_eq!(ResizeStrategy::Fit.resize(old, new), fit[i]);
        }
    }
}
