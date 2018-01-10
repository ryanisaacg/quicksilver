use geom::Vector;

#[derive(Copy, Clone, Debug)]
///The way to adjust the content when the size of the window changes
pub enum ResizeStrategy {
    ///Use black bars to keep the size exactly the same
    MaintainSize,
    ///Fill the screen, possiby cutting off content in the process
    Stretch,
    ///Take up as much of the screen as possible, but use letterboxing if necessary
    Letterbox
}

impl ResizeStrategy {
    ///Calculate the content offset and the content size
    pub(crate) fn resize(self, target_ratio: f32, new_width: u32, new_height: u32) -> (Vector, Vector) {
        let window_ratio = new_width as f32 / new_height as f32;
        let (w, h) = if window_ratio > target_ratio {
            ((target_ratio * new_height as f32) as i32, new_height as i32)
        } else if window_ratio < target_ratio {
            (new_width as i32, (new_width as f32 / target_ratio) as i32)
        } else {
            (new_width as i32, new_height as i32)
        };
        let offset_x = (new_width as i32 - w) / 2;
        let offset_y = (new_height as i32 - h) / 2;
        (Vector::newi(offset_x, offset_y), Vector::newi(w, h))
    }
}
