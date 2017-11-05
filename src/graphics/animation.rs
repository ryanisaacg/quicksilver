use graphics::TextureRegion;

pub struct Frame<'a> {
    pub image: TextureRegion<'a>,
    pub delay: usize,
}

impl<'a> Frame<'a> {
    pub fn new(image: TextureRegion<'a>, delay: usize) -> Frame<'a> {
        Frame {
            image: image,
            delay: delay,
        }
    }
}

pub struct Animation<'a> {
    frames: &'a [Frame<'a>],
    current_frame: usize,
    current_time: usize,
}

impl<'a> Animation<'a> {
    ///Create an animation from an array of frames
    pub fn new(frames: &'a [Frame]) -> Animation<'a> {
        Animation {
            frames: frames,
            current_frame: 0,
            current_time: 0,
        }
    }

    ///Tick the animation forward one frame
    pub fn tick(&mut self) -> TextureRegion {
        self.current_frame += 1;
        if self.current_time >= self.frames[self.current_frame].delay {
            self.current_time = 0;
            self.current_frame = (self.current_frame + 1) % self.frames.len();
        }
        self.current()
    }

    ///Get the current frame
    pub fn current(&self) -> TextureRegion {
        self.frames[self.current_frame].image
    }
}
