use core::num::NonZeroUsize;
use instant::Instant;
use std::time::Duration;
/// A timer that you can use to fix the time between actions, for example updates or draw calls.
pub struct Timer {
    period: Duration,
    init: Instant,
}

impl Timer {
    pub fn time_per_second(times: f32) -> Timer {
        Timer::with_duration(Duration::from_secs_f32(1.0 / times))
    }

    pub fn with_duration(period: Duration) -> Timer {
        Timer {
            period,
            init: Instant::now(),
        }
    }

    /// Look if the time has elapsed and if so, starts the countdown for the next tick.
    ///
    /// You can use a while loop instead of an if to catch up in the event that you where late
    pub fn tick(&mut self) -> bool {
        if self.init.elapsed() >= self.period {
            self.init += self.period;
            true
        } else {
            false
        }
    }

    /// Similar to Self::tick() but tells you how many ticks have passed, rather than just if a tick has passed.
    /// This is usefull in situations where catching up isn't needed or possible
    pub fn exhaust(&mut self) -> Option<NonZeroUsize> {
        let mut count = 0;
        while self.tick() {
            count += 1;
        }
        NonZeroUsize::new(count)
    }

    /// Resets the timer to count from this moment.
    /// This is the same as creating a new Timer with the same period
    pub fn reset(&mut self) {
        self.init = Instant::now();
    }

    /// Look how much time is still left before its time for next tick.
    pub fn remaining(&self) -> Option<Duration> {
        self.period.checked_sub(self.init.elapsed())
    }
}
