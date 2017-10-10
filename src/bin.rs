extern crate qs;
extern crate gl;

use qs::{AssetManager, State, run};
use qs::geom::{Circle, Rectangle, Transform};
use qs::graphics::{Frontend, BLUE, Color, Texture, PixelFormat, WHITE};
use qs::input::Keyboard;
use std::time::Duration;

struct Screen {
    white: Texture,
    position: f32
}

impl State for Screen {
    fn new(_: &mut AssetManager) -> Screen {
        let tex = Texture::from_raw(&[255, 255, 255, 255], 1, 1, PixelFormat::RGBA);
        Screen {
            white: tex,
            position: 0f32
        }
    }

    fn tick(&mut self, draw: &mut Frontend, keys: &Keyboard) {
        draw.clear(Color {r: 0f32, g: 1f32, b: 1f32, a: 1f32});
        if keys[30].is_down() {
            self.position += 1f32;
        }
        draw.draw_image(self.white.region(), Rectangle::new_sized(32f32, 32f32), Transform::identity(), WHITE);
        draw.draw_circle(Circle::new(self.position, 100f32, 60f32), Transform::identity(), BLUE);
        draw.draw_rect(Rectangle::new(100f32, 100f32, 60f32, 60f32), Transform::identity(), WHITE);
        draw.present();
    }

    fn get_tick_delay(&self) -> Duration {
        Duration::from_millis(10)
    }
}

fn main() {
    run::<Screen>("Window", 800, 600);
}
