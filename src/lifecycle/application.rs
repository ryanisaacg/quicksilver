use {
    Result,
    backend::Backend,
    lifecycle::{Event, State, Window},
};
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(target_arch = "wasm32")]
use stdweb::web::Date;

pub struct Application<T: State> {
    pub state: T,
    pub window: Window,
    pub event_buffer: Vec<Event>,
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
        let delta_update = current - self.last_update;
        if delta_update >= self.window.update_rate() {
            self.state.update(&mut self.window, delta_update)?;
            self.window.clear_temporary_states();
            self.last_update = current;
        }
        Ok(())
    }

    pub fn draw(&mut self) -> Result<()> {
        let current = current_time();
        let delta_draw = current - self.last_draw;
        if delta_draw >= self.window.draw_rate() {
            self.state.draw(&mut self.window, delta_draw)?;
            self.window.flush()?;
            self.window.backend.present()?;
            self.window.log_framerate(delta_draw);
            self.last_draw = current;
        }
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as f64 * 1000.0 + since_the_epoch.subsec_nanos() as f64 / 1e6
}

#[cfg(target_arch = "wasm32")]
fn current_time() -> f64 {
    Date::now()
}
