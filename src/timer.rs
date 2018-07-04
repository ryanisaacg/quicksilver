use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
///A structure that allows accumulation of actions in a loop
pub struct Timer {
    previous_tick: Instant,
    wait_time: Duration,
}

impl Timer {
    ///Create a timer that will tick at the first opportunity
    pub fn new() -> Timer {
        Timer { previous_tick: Instant::now(),
                wait_time: Duration::from_millis(0), }
    }

    ///For as many times as appopriate, update
    pub fn tick<E>(&mut self, mut action: impl FnMut() -> Result<Duration, E>) -> Result<(), E> {
        if self.wait_time.subsec_nanos() == 0 {
            self.wait_time = action()?;
            self.previous_tick = Instant::now();
        } else {
            let iterations =
                self.previous_tick.elapsed().subsec_nanos() / self.wait_time.subsec_nanos();
            for _ in 0..iterations {
                self.wait_time = action()?;
                self.previous_tick = Instant::now();
            }
        }
        Ok(())
    }
}
