extern crate qs;
extern crate gl;
extern crate sdl2;

use qs::{AssetManager, State, run};
use qs::geom::{Rectangle, Vector, Transform};
use qs::graphics::{Frontend, Color, Texture, TextureRegion, PixelFormat, WHITE};
use std::time::Duration;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

struct Screen {
    white: Texture
}

impl State for Screen {
    fn new(assets: &mut AssetManager) -> Screen {
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
