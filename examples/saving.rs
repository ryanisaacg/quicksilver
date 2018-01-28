#[macro_use]
extern crate quicksilver;

use quicksilver::geom::Rectangle;
use quicksilver::graphics::{ Canvas, Color, Window, WindowBuilder };
use quicksilver::saving::{ load, save };
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
    rect: Rectangle
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new().build("Saving / loading", 800, 600);
        save("quicksilver-save-example", "example-profile", &Rectangle::new(50, 50, 100, 100)).unwrap();
        let rect = load("quicksilver-save-example", "example-profile").unwrap();
        State { window, canvas, rect }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        Duration::from_millis(100)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::white());
        self.canvas.draw_rect(self.rect, Color::black());
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
