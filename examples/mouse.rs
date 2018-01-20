#[macro_use]
extern crate quicksilver;

use quicksilver::geom::Rectangle;
use quicksilver::graphics::{Color, Canvas, Window, WindowBuilder};
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
    bounds: Rectangle
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new().build("Mouse example", 800, 600);
        State {
            window,
            canvas,
            bounds: Rectangle::newi_sized(32, 32)
        }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        self.window.clear_temporary_states();
        self.bounds = self.bounds.translate(self.window.mouse().wheel());
        Duration::from_millis(16)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::white());
        self.canvas.draw_rect(self.bounds, Color::blue());
        self.canvas.draw_rect(Rectangle::newi_sized(40, 40).translate(self.window.mouse().pos()), Color::green());
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
