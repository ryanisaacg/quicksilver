use core::num::NonZeroUsize;
use instant::Instant;
use std::time::Duration;

/// A timer that you can use to fix the time between actions, for example updates or draw calls.
///
/// See the article [Fix Your Timestep](https://gafferongames.com/post/fix_your_timestep/) for more
/// on how to use timers to ensure framerate-independence.
pub struct Timer {
    period: Duration,
    init: Instant,
}

impl Timer {
    /// Create a timer that ticks n many times per second
    pub fn time_per_second(times: f32) -> Timer {
        Timer::with_duration(Duration::from_secs_f32(1.0 / times))
    }

    /// Create a timer with a given period (time between ticks)
    pub fn with_duration(period: Duration) -> Timer {
        Timer {
            period,
            init: Instant::now(),
        }
    }

    /// Look if the time has elapsed and if so, starts the countdown for the next tick.
    ///
    /// You can use a while loop instead of an if to catch up in the event that you were late. Each
    /// tick will only 'consume' one period worth of time.
    pub fn tick(&mut self) -> bool {
        if self.init.elapsed() >= self.period {
            self.init += self.period;
            true
        } else {
            false
        }
    }

    /// Similar to Self::tick() but tells you how many ticks have passed, rather than just if a tick has passed.
    ///
    /// This is useful in situations where catching up isn't needed or possible, like rendering to
    /// the screen. If you've missed rendering three frames, there's no point in drawing them now:
    /// just render the current state and move on.
    pub fn exhaust(&mut self) -> Option<NonZeroUsize> {
        let mut count = 0;
        while self.tick() {
            count += 1;
        }
        NonZeroUsize::new(count)
    }

    /// Resets the timer to count from this moment.
    ///
    /// This is the same as creating a new Timer with the same period
    pub fn reset(&mut self) {
        self.init = Instant::now();
    }

    /// Gets the time in between ticks
    pub fn period(&self) -> Duration {
        self.period
    }

    /// How much time has passed since the timer was last ticked
    pub fn elapsed(&self) -> Duration {
        self.init.elapsed()
    }

    /// Look how much time is still left before its time for next tick.
    pub fn remaining(&self) -> Option<Duration> {
        self.period.checked_sub(self.init.elapsed())
    }

    /// Look how late you are with calling Timer::tick() if you would call it right now
    pub fn late_by(&self) -> Option<Duration> {
        self.init.elapsed().checked_sub(self.period)
    }
}
