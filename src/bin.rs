extern crate qs;
extern crate gl;

use qs::*;
use qs::geom::*;
use qs::graphics::*;
use qs::input::*;
use std::time::Duration;

struct Entity {
    pub bounds: Rectangle,
    pub speed: Vector,
    pub facing: Vector
}

impl Entity {
    pub fn new(bounds: Rectangle) -> Entity {
        Entity {
            bounds: bounds,
            speed: Vector::zero(),
            facing: Vector::zero()
        }
    }

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
            player: Entity::new(Rectangle::newi(0, 0, 32, 32)),
            map: Tilemap::new(800f32, 600f32, 40f32, 40f32)
        }
    }

    fn tick(&mut self, keys: &Keyboard, _: &Mouse) -> Duration {
        self.player.speed += Vector::y() * 0.5;
        if self.player.speed.x.abs() < 0.3 {
            self.player.speed.x = 0.0;
        } else {
            self.player.speed.x *= 0.9;
        }
        if keys[Key::A].is_down() {
            self.player.speed.x -= 0.4;
            self.player.facing = -Vector::x();
        } else if keys[Key::D].is_down() {
            self.player.speed.x += 0.4;
            self.player.facing = Vector::x();
        }
        if keys[Key::Space].is_down() {
            if !self.map.region_empty(self.player.bounds.translate(Vector::y())) {
                self.player.speed.y = -8f32;
            } else if !self.map.region_empty(self.player.bounds.translate(self.player.facing)) {
                self.player.speed.y = -8f32;
                self.player.speed += -self.player.facing * 8;
            }
        }
        self.player.step(&self.map);
        Duration::from_millis(10)
    }

    fn draw(&mut self, draw: &mut Graphics) {
        draw.set_clear_color(WHITE);
        draw.draw_rect(self.player.bounds, Transform::identity(), BLUE);
        draw.draw_rect(self.player.bounds, Transform::translate(self.player.bounds.center()) * Transform::rotate(45.0) * Transform::translate(-self.player.bounds.center()), BLUE);
    }
}

fn main() {
    run::<Screen>("Window", 800, 600);
}
