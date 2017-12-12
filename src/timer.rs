use std::time::{Duration, Instant};

pub struct Timer {
    previous_tick: Instant,
    wait_time: Duration
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            previous_tick: Instant::now(),
            wait_time: Duration::from_millis(0)
        }
    }

    pub fn tick<F>(&mut self, action: F) where F: FnOnce() -> Duration {
        if self.previous_tick.elapsed() >= self.wait_time {
            self.wait_time = action();
            self.previous_tick = Instant::now();
        }
    }
}
