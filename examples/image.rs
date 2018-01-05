#[macro_use]
extern crate quicksilver;

use quicksilver::geom::Vector;
use quicksilver::graphics::{Canvas, Color, Image, Window, WindowBuilder};
use std::time::Duration;

struct State {
    window: Window,
    canvas: Canvas,
    image: Image
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new()
            .build("Basic Window", 800, 600);
        let image = Image::load("examples/image.png").unwrap();
        State { window, canvas, image }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        Duration::from_millis(1000)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::white());
        self.canvas.draw_image(&self.image, Vector::new(100.0, 100.0));
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
