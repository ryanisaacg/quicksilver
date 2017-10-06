extern crate qs;
extern crate gl;

use qs::{AssetManager, State, run};
use qs::geom::{Rectangle, Transform};
use qs::graphics::{Frontend, Color, Texture, PixelFormat, WHITE};
use std::time::Duration;

struct Screen {
    white: Texture
}

impl State for Screen {
    fn new(_: &mut AssetManager) -> Screen {
        let tex = Texture::from_raw(&[255, 255, 255, 255], 1, 1, PixelFormat::RGBA);
        Screen {
            white: tex
        }
    }

    fn tick(&mut self, draw: &mut Frontend) {
        draw.clear(Color {r: 0f32, g: 1f32, b: 1f32, a: 1f32});
        draw.draw_image(self.white.region(), Rectangle::new_sized(32f32, 32f32), Transform::identity(), WHITE);
        draw.present();
    }

    fn get_tick_delay(&self) -> Duration {
        Duration::from_millis(10)
    }
}

fn main() {
    run::<Screen>("Window", 800, 600);
}
