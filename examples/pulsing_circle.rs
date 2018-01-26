#[macro_use]
extern crate quicksilver;

use quicksilver::geom::{Circle, Transform, Vector};
use quicksilver::graphics::{Canvas, Color, Window, WindowBuilder};
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
    scale: Vector
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new()
            .with_show_cursor(false)
            .build("Circle", 800, 600);
        let scale = Vector::one();
        State { window, canvas, scale }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        self.scale = self.scale.normalize() * ((self.scale.len() + 0.05) % 1.0 + 1.0);
        Duration::from_millis(16)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::black());
        self.canvas.draw_circle_trans(Circle::new(400, 300, 50), Color::white(), Transform::scale(self.scale));
        self.canvas.present(&self.window);
    }
}

game_loop!(State);

