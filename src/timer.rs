use instant::Instant;
use std::time::Duration;

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
	pub fn tick(&mut self) -> bool {
		if self.init.elapsed() >= self.period {
			self.init = Instant::now();
			true
		} else {
			false
		}
	}
	pub fn remaining(&self) -> Option<Duration> {
		self.init.elapsed().checked_sub(self.period)
	}
}
