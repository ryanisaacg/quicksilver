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

fn main() {
    let (mut window, mut canvas) = WindowBuilder::new()
        .with_show_cursor(false)
        .with_clear_color(Color::white())
        .build("Window", 800, 600);
    let mut player = Entity::new(Rectangle::newi(16, 16, 32, 32));
    let map: Tilemap<u8> = Tilemap::new(800f32, 600f32, 40f32, 40f32);
    let mut timer = Timer::new();
    while window.poll_events() {
        timer.tick(|| {
            let keyboard = window.keyboard();
            player.speed += Vector::y() * 0.5;
            if player.speed.x.abs() < 0.3 {
                player.speed.x = 0.0;
            } else {
                player.speed.x *= 0.9;
            }
            if keyboard[Key::A].is_down() {
                player.speed.x -= 0.4;
                player.facing = -Vector::x();
            } else if keyboard[Key::D].is_down() {
                player.speed.x += 0.4;
                player.facing = Vector::x();
            }
            if keyboard[Key::Space].is_down() {
                if !map.shape_empty(player.bounds.translate(Vector::y())) {
                    player.speed.y = -8f32;
                } else if !map.shape_empty(player.bounds.translate(player.facing)) {
                    player.speed.y = -8f32;
                    player.speed += -player.facing * 8;
                }
            }
            player.step(&map);
            Duration::from_millis(10)
        });
        canvas.draw_line(Line::new(Vector::zero(), Vector::one() * 100), Color::black());
        canvas.draw_shape(player.bounds, Color::blue());
        canvas.draw_shape_trans(player.bounds, Color::blue(), Transform::translate(Vector::one() * 16) 
                * Transform::rotate(45.0) 
                * Transform::translate(Vector::one() * -16));
        canvas.present(&window);
    }
}
