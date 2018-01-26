#[macro_use]
extern crate quicksilver;

use quicksilver::geom::*;
use quicksilver::graphics::*;
use quicksilver::input::Key;

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

pub struct State {
    window: Window,
    canvas: Canvas,
    player: Entity,
    map: Tilemap<u8>
}

impl State {
    fn new() -> State {
        let (window, canvas) = WindowBuilder::new()
            .with_show_cursor(false)
            .build("Window", 800, 600);
        State {
            window,
            canvas,
            player: Entity::new(Rectangle::new(16, 16, 32, 32)),
            map: Tilemap::new(Vector::new(800, 600), Vector::new(40, 40))
        }
    }

    fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    fn update(&mut self) -> Duration {
        self.window.clear_temporary_states();
        let keyboard = self.window.keyboard();
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
        Duration::from_millis(8)
    }

    fn draw(&mut self) {
        self.canvas.clear(Color::white());
        self.canvas.draw_line(Line::new(Vector::zero(), Vector::one() * 100), Color::black());
        self.canvas.draw_shape(self.player.bounds, Color::blue());
        self.canvas.draw_shape_trans(self.player.bounds, Color::blue(), Transform::translate(Vector::one() * 16) 
                * Transform::rotate(45.0) 
                * Transform::translate(Vector::one() * -16));
        self.canvas.present(&self.window);
    }
}

game_loop!(State);
