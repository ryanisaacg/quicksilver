extern crate qs;
extern crate gl;

use qs::*;
use qs::geom::*;
use qs::graphics::*;
use qs::input::*;
use std::time::Duration;

struct Entity {
    pub bounds: Rectangle,
    pub speed: Vector
}

impl Entity {
    pub fn step<T: Clone>(&mut self, map: &Tilemap<T>) {
        let (bounds, speed) = map.move_until_contact(self.bounds, self.speed);
        self.bounds = bounds;
        self.speed = speed;
    }
}

struct Screen {
    player: Entity,
    map: Tilemap<i32>
}

impl State for Screen {
    fn new(_: &mut AssetManager) -> Screen {
        Screen {
            player: Entity {
                bounds: Rectangle::newi(0, 0, 30, 30),
                speed: Vector::zero()
            },
            map: Tilemap::new(800f32, 600f32, 40f32, 40f32)
        }
    }

    fn tick(&mut self, keys: &Keyboard, _: &Mouse) -> Duration {
        self.player.speed += Vector::y() * 0.5;
        self.player.speed.x = 0f32;
        if keys[Key::A].is_down() {
            self.player.speed.x = -2f32;
        } else if keys[Key::D].is_down() {
            self.player.speed.x = 2f32;
        }
        self.player.step(&self.map);
        println!("{}", self.player.speed);
        Duration::from_millis(10)
    }

    fn draw(&mut self, draw: &mut Frontend) {
        draw.clear(WHITE);
        draw.draw_rect(self.player.bounds, Transform::identity(), BLUE);
        draw.present();
    }
}

fn main() {
    run::<Screen>("Window", 800, 600);
}
