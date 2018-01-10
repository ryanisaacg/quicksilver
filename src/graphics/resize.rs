use geom::Vector;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
///The way to adjust the content when the size of the window changes
pub enum ResizeStrategy {
    ///Use black bars to keep the size exactly the same
//    Maintain,
    ///Fill the screen, possiby cutting off content in the process
    Fill,
    ///Take up as much of the screen as possible, but use letterboxing if necessary
    Fit
}

impl ResizeStrategy {
    ///Calculate the content offset and the content size
    pub(crate) fn resize(self, target_ratio: f32, new_width: u32, new_height: u32) -> (Vector, Vector) {
        let window_ratio = new_width as f32 / new_height as f32;
        match self {
            ResizeStrategy::Fill | ResizeStrategy::Fit => {
                let (w, h) = if (self == ResizeStrategy::Fill) == (window_ratio < target_ratio) {
                    ((target_ratio * new_height as f32) as i32, new_height as i32)
                } else {
                    (new_width as i32, (new_width as f32 / target_ratio) as i32)
                }; 
                let offset_x = (new_width as i32 - w) / 2;
                let offset_y = (new_height as i32 - h) / 2;
                (Vector::newi(offset_x, offset_y), Vector::newi(w, h))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize() {
        //Format: (target_ratio, new_width, new_height)
        let inputs = [
            (16.0 / 9.0, 16, 9),
            (16.0 / 9.0, 32, 9),
            (16.0 / 9.0, 16, 18),
        ];
        //Fomat: (offset, size)
        let fill = [
            (Vector::zero(), Vector::newi(16, 9)),
            (Vector::newi(0, -4), Vector::newi(32, 18)),
            (Vector::newi(-8, 0), Vector::newi(32, 18))
        ];
        let fit = [
            (Vector::zero(), Vector::newi(16, 9)),
            (Vector::newi(8, 0), Vector::newi(16, 9)),
            (Vector::newi(0, 4), Vector::newi(16, 9))
        ];
        for i in 0..inputs.len() {
            let (target, width, height) = inputs[i];
            assert_eq!(ResizeStrategy::Fill.resize(target, width, height), fill[i]);
            assert_eq!(ResizeStrategy::Fit.resize(target, width, height), fit[i]);
        }
    }
}
