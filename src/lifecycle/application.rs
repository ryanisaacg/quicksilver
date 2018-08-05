use {
    Result,
    graphics::Window,
    lifecycle::{Event, State},
};
#[cfg(not(target_arch = "wasm32"))]
use {
    glutin::GlContext,
    std::time::{SystemTime, UNIX_EPOCH},
};
#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

pub struct Application<T: State> {
    pub state: T,
    pub window: Window,
    pub event_buffer: Vec<Event>,
    accumulator: f64,
    last_draw: f64,
    last_update: f64,
}

impl<T: State> Application<T> {
    pub fn new(window: Window) -> Result<Application<T>> {
        let time = current_time();
        Ok(Application {
            state: T::new()?,
            window,
            event_buffer: Vec::new(),
            accumulator: 0.0,
            last_draw: time,
            last_update: time,
        })
    }

    pub fn update(&mut self) -> Result<()> {
        self.window.update_gamepads(&mut self.event_buffer);
        for i in 0..self.event_buffer.len() {
            self.window.process_event(&self.event_buffer[i]);
            self.state.event(&self.event_buffer[i], &mut self.window)?;
        }
        self.event_buffer.clear();
        let current = current_time();
        self.accumulator += current - self.last_update;
        self.last_update = current;
        let mut ticks = 0;
        while self.accumulator > 0.0 && (self.window.max_ticks() == 0 || ticks < self.window.max_ticks()) {
            self.state.update(&mut self.window)?;
            self.window.clear_temporary_states();
            self.accumulator -= self.window.tick_rate();
            ticks += 1;
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.state.draw(&mut self.window)?;
        self.window.flush()?;
        #[cfg(not(target_arch = "wasm32"))]
        self.window.gl_window.swap_buffers()?;
        let current = current_time();
        self.window.log_framerate(current - self.last_draw);
        self.last_draw = current;
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as f64 * 1000.0 + since_the_epoch.subsec_millis() as f64
}

#[cfg(target_arch = "wasm32")]
fn current_time() -> f64 {
    Date::now()
}
