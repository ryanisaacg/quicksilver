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

    pub fn tick<F>(&mut self, mut action: F) where F: FnMut() -> Duration {
        if self.wait_time.subsec_nanos() == 0 {
            self.wait_time = action();
            self.previous_tick = Instant::now();
        } else {
            let iterations = self.previous_tick.elapsed().subsec_nanos() / self.wait_time.subsec_nanos();
            for _ in 0..iterations {
                self.wait_time = action();
                self.previous_tick = Instant::now();
            }
        }
    }
}
