extern crate quicksilver;

use quicksilver::*;
use std::time::Duration;

struct Entity {
    pub bounds: Shape,
    pub speed: Vector,
    pub facing: Vector,
}

impl Entity {
    pub fn new(bounds: Rectangle) -> Entity {
        Entity {
            bounds: Shape::Rect(bounds),
            speed: Vector::zero(),
            facing: Vector::zero(),
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
    map: Tilemap<i32>,
}

impl State for Screen {
    fn prepare_assets<'a>() -> AssetList<'a> {
        AssetList::new()
    }

    fn configure(builder: GraphicsBuilder) -> GraphicsBuilder {
        builder
            .with_show_cursor(false)
            .with_clear_color(Colors::WHITE)
    }

    fn new(_: Assets) -> Screen {
        Screen {
            player: Entity::new(Rectangle::newi(16, 16, 32, 32)),
            map: Tilemap::new(800f32, 600f32, 40f32, 40f32),
        }
    }

    fn tick(&mut self, input: InputBuilder) -> Duration {
        let (keyboard, _, _) = input.build(Rectangle::new_sized(800f32, 600f32));
        self.player.speed += Vector::y() * 0.5;
        if self.player.speed.x.abs() < 0.3 {
            self.player.speed.x = 0.0;
        } else {
            self.player.speed.x *= 0.9;
        }
        if keyboard[Key::A].is_down() {
            self.player.speed.x -= 0.4;
            self.player.facing = -Vector::x();
        } else if keyboard[Key::D].is_down() {
            self.player.speed.x += 0.4;
            self.player.facing = Vector::x();
        }
        if keyboard[Key::Space].is_down() {
            if !self.map.shape_empty(self.player.bounds.translate(Vector::y())) {
                self.player.speed.y = -8f32;
            } else if !self.map.shape_empty(self.player.bounds.translate(self.player.facing)) {
                self.player.speed.y = -8f32;
                self.player.speed += -self.player.facing * 8;
            }
        }
        self.player.step(&self.map);
        Duration::from_millis(10)
    }

    fn draw(&mut self, draw: &mut Graphics) {
        draw.draw_line(Line::new(Vector::zero(), Vector::one() * 100), Colors::BLACK);
        draw.draw_shape(self.player.bounds, Colors::BLUE);
        draw.draw_shape_trans(self.player.bounds, Colors::BLUE, Transform::translate(Vector::one() * 16) 
                * Transform::rotate(45.0) 
                * Transform::translate(Vector::one() * -16));
    }
}

fn main() {
    run::<Screen>("Window", 800, 600);
}
