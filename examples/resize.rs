#[macro_use]
extern crate quicksilver;

use quicksilver::geom::*;
use quicksilver::graphics::*;

use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
}

impl State {
    fn new() -> State {
        let (window, canvas) = WindowBuilder::new()
            .with_show_cursor(false)
            .with_minimum_size(Vector::newi(400, 300))
            .with_maximum_size(Vector::newi(1600, 900))
            .build("Window", 800, 600);
        State { window, canvas }
    }

    fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    fn update(&mut self) -> Duration {
        Duration::from_millis(16)
    }

    fn draw(&mut self) {
        self.canvas.clear(Color::white());
        self.canvas.draw_rect(Rectangle::newi_sized(100, 100), Color::blue());
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
