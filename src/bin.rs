extern crate qs;
extern crate gl;

use qs::*;
use qs::geom::*;
use qs::graphics::*;
use qs::input::*;
use std::time::Duration;

struct Screen {
    player: Rectangle,
    map: Tilemap<i32>
}

impl State for Screen {
    fn new(_: &mut AssetManager) -> Screen {
        Screen {
            player: Rectangle::newi(0, 0, 30, 30),
            map: Tilemap::new(800f32, 600f32, 40f32, 40f32)
        }
    }

    fn tick(&mut self, draw: &mut Frontend, keys: &Keyboard, mouse: &Mouse) {
        draw.clear(WHITE);
        if keys[Key::A].is_down() {
            self.player.x -= 1f32;
        }
        draw.draw_rect(self.player, Transform::identity(), BLUE);
        draw.present();
    }

    fn get_tick_delay(&self) -> Duration {
        Duration::from_millis(10)
    }
}

fn main() {
    run::<Screen>("Window", 800, 600);
}
