use crate::{
    geom::Rectangle,
    graphics::Image,
    lifecycle::Window,
};
use std::rc::Rc;

#[derive(Debug)]
struct AnimationData {
    frames: Vec<Image>,
    len: usize,
}

#[derive(Clone, Debug)]
/// A linear series of images with a constant frame delay
///
/// Animation advance by time delta, which should be run in the `update` section of a
/// quicksilver application loop rather than the `draw` section. Draws may happen as
/// often as possible, whereas updates will have consistent rates
///
/// To restart an animation:
///
/// ```rust,ignore
/// anim.reset();
/// anim.play();
/// ```
pub struct Animation {
    data: Rc<AnimationData>,
    /// duration in secs
    duration: f64,
    /// current time
    current_time: f64,
    /// if animation is stopped
    stopped: bool,
    /// if animation is paused
    paused: bool,
}

impl Animation {
    /// Create a new animation from a series of images and a duration.
    pub fn new<I>(images: I, duration: f64) -> Animation
        where I: IntoIterator<Item = Image> {
        let frames: Vec<_> = images.into_iter().collect();
        let len = frames.len();
        Animation {
            data: Rc::new(AnimationData { frames, len }),
            duration,
            current_time: 0.,
            stopped: false,
            paused: false,
        }
    }

    /// Create a new animation from regions of images from a spritesheet.
    pub fn from_spritesheet<R>(sheet: Image, regions: R, duration: f64) -> Animation
        where R: IntoIterator<Item = Rectangle> {
        Animation::new(regions.into_iter()
                       .map(|region| sheet.subimage(region)), duration)
    }

    /// Update the internal time of the animation. Must be called.
    pub fn update(&mut self, window: &mut Window) {
        if self.paused { return; }
        self.current_time += window.update_rate() * 0.001;
        if self.current_time >= self.duration {
            self.current_time -= self.duration
        }
        if self.current_frame_index() + 1 == self.data.len {
            self.stopped = true;
        }
    }

    /// Play the animation.
    pub fn play(&mut self) {
        self.stopped = false;
        self.paused = false;
    }

    /// Pause the animation.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Unpause the animation.
    pub fn unpause(&mut self) {
        self.paused = false;
    }

    /// Reset the animation.
    pub fn reset(&mut self) {
        self.current_time = 0.;
        self.stopped = true;
    }

    /// Returns true if the animation is not stopped and not paused.
    pub fn is_playing(&mut self) -> bool {
        !self.stopped && !self.paused
    }

    /// Set the duration of the animation. Unit in secs
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    /// Get the current frame of the animation
    pub fn current_frame(&self) -> Option<&Image> {
        if self.stopped {
            None
        } else {
            Some(&self.data.frames[self.current_frame_index()])
        }
    }

    /// Get current frame index.
    #[inline(always)]
    fn current_frame_index(&self) -> usize {
        let frame = (self.current_time / self.duration * self.data.len as f64).floor() as usize + 1;
        let nth = frame % self.data.len;
        nth
    }

}

