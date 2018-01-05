#[macro_use]
extern crate quicksilver;

use quicksilver::graphics::{Window, WindowBuilder};
use std::time::Duration;

pub struct State {
    window: Window
}

impl State {
    pub fn new() -> State {
        let (window, _canvas) = WindowBuilder::new().build("Basic Window", 800, 600);
        State { window }
    }

    pub fn events(&mut self) -> bool {
       self.window.poll_events()
    }

    pub fn update(&self) -> Duration {
        Duration::from_millis(0)
    }

    pub fn draw(&self) {}
}

game_loop!(State);
