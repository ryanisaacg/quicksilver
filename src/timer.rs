use std::time::{Duration, Instant};
pub struct Timer {
	period: Duration,
	init: Instant,
}
impl Timer {
	pub fn time_per_second(times: f32) -> Timer {
		Timer::new(Duration::from_secs_f32(1.0 / times))
	}
	pub fn new(period: Duration) -> Timer {
		Timer {
			period,
			init: Instant::now(),
		}
	}
	pub fn tick(&mut self) -> bool {
		if self.init.elapsed() >= self.period {
			self.init = Instant::now();
			true
		} else {
			false
		}
	}
	pub fn ms_remaining(&self) -> Duration {
		self.period - self.init.elapsed()
	}
}
