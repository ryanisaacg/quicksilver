#[macro_use]
extern crate quicksilver;

use quicksilver::asset::{Loadable, LoadingAsset};
use quicksilver::graphics::{Canvas, Color, Image, WindowBuilder, Window};
use quicksilver::geom::Rectangle;
use quicksilver::input::{Key, Viewport};
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
    viewport: Viewport,
    image: LoadingAsset<Image>
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new()
            .build("WASM", 800, 600);
        let image = Image::load("image.png");
        let viewport = window.viewport().build(Rectangle::newi_sized(800, 600));
        State { window, canvas, viewport, image }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        Duration::from_millis(1000)
    }

    pub fn draw(&mut self) {
        let color = if self.window.keyboard()[Key::A].is_down() { Color::blue() } else { Color::white() };
        self.canvas.clear(color);
        self.canvas.draw_rect(Rectangle::newi_sized(100, 100), Color::green());
        match self.image {
            LoadingAsset::Loading(_) => {},
            LoadingAsset::Errored(_) => {},
            LoadingAsset::Loaded(ref image) => 
                self.canvas.draw_image(image, self.window.mouse(&self.viewport).pos())
        }
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
