#[macro_use]
extern crate quicksilver;

use quicksilver::geom::Rectangle;
use quicksilver::graphics::{Color, Canvas, Window, WindowBuilder};
use quicksilver::input::Viewport;
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
    viewport: Viewport,
    bounds: Rectangle
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new().build("Mouse example", 800, 600);
        let viewport = window.viewport().build(Rectangle::newi_sized(800, 600));
        State {
            window,
            canvas,
            viewport,
            bounds: Rectangle::newi_sized(32, 32)
        }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        self.viewport = self.window.viewport().build(Rectangle::newi_sized(800, 600));
        self.bounds = self.bounds.translate(self.window.mouse(&self.viewport).wheel());
        Duration::from_millis(16)
    }

    pub fn draw(&mut self) {
        self.canvas.clear(Color::white());
        self.canvas.draw_rect(self.bounds, Color::blue());
        self.canvas.draw_rect(Rectangle::newi_sized(40, 40).translate(self.window.mouse(&self.viewport).pos()), Color::green());
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
