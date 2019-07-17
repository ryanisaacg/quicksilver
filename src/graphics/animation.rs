use crate::{
    geom::Rectangle,
    graphics::Image
};
use std::rc::Rc;

#[derive(Debug)]
struct AnimationData {
    frames: Vec<Image>
}

#[derive(Clone, Debug)]
/// A linear series of images with a constant frame delay
///
/// Frames advance by discrete ticks, which should be run in the `update` section of a 
/// quicksilver application loop rather than the `draw` section. Draws may happen as 
/// often as possible, whereas updates will have consistent rates
#[deprecated(since = "0.3.16", note = "Animation is too inflexible for most users; an external solution is a better option")]
pub struct Animation {
    data: Rc<AnimationData>,
    current: usize,
    current_time: u32,
    frame_delay: u32
}

impl Animation {
    /// Create a new animation from a series of images and a frame delay
    pub fn new<I>(images: I, frame_delay: u32) -> Animation 
        where I: IntoIterator<Item = Image> {
        let frames = images.into_iter().collect();
        Animation {
            data: Rc::new(AnimationData { frames }),
            current: 0,
            current_time: 0,
            frame_delay
        }
    }

    /// Create a new animation from regions of images from a spritesheet
    pub fn from_spritesheet<R>(sheet: Image, regions: R, frame_delay: u32) -> Animation 
        where R: IntoIterator<Item = Rectangle> {
        Animation::new(regions.into_iter()
                       .map(|region| sheet.subimage(region)), frame_delay)
    }

    /// Tick the animation forward by one step
    pub fn tick(&mut self) {
        self.current_time += 1;
        if self.current_time >= self.frame_delay {
            self.current = (self.current + 1) % self.data.frames.len();
            self.current_time = 0;
        }
    }

    /// Get the current frame of the animation
    pub fn current_frame(&self) -> &Image {
        &self.data.frames[self.current]
    }
}

