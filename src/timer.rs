use instant::Instant;
use std::time::Duration;
///A timer that you can use to fix the time between actions, for example updates or draw calls.
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

    ///Look if the time has elapsed and if so, starts the countdown for the next tick.
    ///
    ///You can use a while loop instead of an if to catch up in the event that you where late
    pub fn tick(&mut self) -> bool {
        if self.init.elapsed() >= self.period {
            self.init += self.period;
            true
        } else {
            false
        }
    }

    ///look how much time is still left before its time for next tick.
    pub fn remaining(&self) -> Option<Duration> {
        self.init.elapsed().checked_sub(self.period)
    }
}
