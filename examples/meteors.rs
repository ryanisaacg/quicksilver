// A tribute to a certain Atari arcade game
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Positioned, Rectangle, Vector},
    graphics::{Color, Sprite, Window, WindowBuilder, View},
    input::Key
};


struct Entity {
    bounds: Circle,
    facing: f32,
    velocity: Vector
}

impl Entity {
    fn new(bounds: Circle) -> Entity {
        Entity {
            bounds,
            facing: 0.0,
            velocity: Vector::zero()
        }
    }

    fn tick(&mut self) {
        self.bounds = self.bounds.translate(self.velocity);
    }
}

struct GameState {
    player: Entity,
    camera: Rectangle
}


impl State for GameState {
    fn new() -> GameState {
        GameState {
            player: Entity::new(Circle::newv(Vector::zero(), 16)),
            camera: Rectangle::new_sized(SCREEN_WIDTH, SCREEN_HEIGHT)
        }
    }

    fn update(&mut self, window: &mut Window) {
        if window.keyboard()[Key::A].is_down() {
            self.player.facing -= PLAYER_ROTATION;
        }
        if window.keyboard()[Key::D].is_down() {
            self.player.facing += PLAYER_ROTATION;
        }
        if window.keyboard()[Key::W].is_down() {
            self.player.velocity += Vector::from_angle(self.player.facing) * PLAYER_IMPULSE;
        }
        if window.keyboard()[Key::S].is_down() {
            self.player.velocity *= PLAYER_BREAK_FACTOR;
        }
        self.player.tick();
        self.camera = self.camera.with_center((self.camera.center() + self.player.bounds.center()) / 2);
        window.set_view(View::new(self.camera));
    }

    fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        window.draw(&Sprite::circle(self.player.bounds).with_color(Color::white()));
        window.present();
    }
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PLAYER_ROTATION: f32 = 5.0;
const PLAYER_IMPULSE: f32 = 0.1;
const PLAYER_BREAK_FACTOR: f32 = 0.95;

fn main() {
    run::<GameState>(WindowBuilder::new("GameState", SCREEN_WIDTH, SCREEN_HEIGHT));
}
