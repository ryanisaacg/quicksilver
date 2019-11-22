use crate::{
    backend::Backend,
    lifecycle::{Event, FromEvent, State, Window},
    Result,
};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
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

impl<T: State> Application<T>
where
    <T as State>::Message: FromEvent,
{
    pub fn new<F: FnOnce() -> Result<T>>(window: Window, f: F) -> Result<Application<T>> {
        let time = current_time();
        Ok(Application {
            state: f()?,
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
            let message = <T::Message as FromEvent>::from(&self.event_buffer[i]);
            self.state.event(&message, &mut self.window)?;
        }
        self.event_buffer.clear();

        let current = current_time();
        self.accumulator += current - self.last_update;
        self.last_update = current;
        let mut ticks = 0;
        let update_rate = self.window.update_rate();
        while self.accumulator > 0.0
            && (self.window.max_updates() == 0 || ticks < self.window.max_updates())
        {
            self.state.update(&mut self.window)?;
            self.window.clear_temporary_states();
            self.accumulator -= update_rate;
            ticks += 1;
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn draw(&mut self) -> Result<()> {
        let current = current_time();
        let delta_draw = current - self.last_draw;
        if delta_draw >= self.window.draw_rate() {
            self.state.draw(&mut self.window)?;
            self.window.flush()?;
            self.window.backend().present()?;
            self.window.log_framerate(delta_draw);
            self.last_draw = current;
        } else {
            // Only sleep up to 1/10th of minimum of draw and update rate to make sure that we're definitely not sleeping longer than needed
            let max_sleep = self.window.draw_rate().min(self.window.update_rate()) * 0.1;
            let remaining_time = self.window.draw_rate() - delta_draw;
            if remaining_time >= max_sleep {
                thread::sleep(Duration::from_millis(max_sleep as u64));
            }
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub fn draw(&mut self) -> Result<()> {
        let current = current_time();
        let delta_draw = current - self.last_draw;
        if delta_draw >= self.window.draw_rate() {
            self.state.draw(&mut self.window)?;
            self.window.flush()?;
            self.window.backend().present()?;
            self.window.log_framerate(delta_draw);
            self.last_draw = current;
        }
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn current_time() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as f64 * 1000.0 + since_the_epoch.subsec_nanos() as f64 / 1e6
}

#[cfg(target_arch = "wasm32")]
fn current_time() -> f64 {
    Date::now()
}
