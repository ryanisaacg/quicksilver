#[macro_use]
extern crate quicksilver;

use quicksilver::asset::{Loadable, LoadingAsset};
use quicksilver::geom::Vector;
use quicksilver::graphics::{Canvas, Color, Image, Window, WindowBuilder};
use std::time::Duration;

pub struct State {
    window: Window,
    canvas: Canvas,
    image: LoadingAsset<Image>
}

impl State {
    pub fn new() -> State {
        let (window, canvas) = WindowBuilder::new()
            .build("Basic Window", 800, 600);
        let image = Image::load("examples/image.png");
        State { window, canvas, image }
    }

    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) -> Duration {
        Duration::from_millis(1000)
    }

    pub fn draw(&mut self) {
        self.image.update();
        match self.image {
            LoadingAsset::Loading(_) => {},
            LoadingAsset::Errored(_) => {},
            LoadingAsset::Loaded(ref image) => {
                self.canvas.clear(Color::white());
                self.canvas.draw_image(image, Vector::new(100.0, 100.0));
                self.canvas.present(&self.window);
            }
        }
    }
}

game_loop!(State);
