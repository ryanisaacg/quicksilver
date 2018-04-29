// A tribute to a certain Atari arcade game
extern crate futures;
extern crate quicksilver;
extern crate rand;

use futures::{
    Future, Async,
    future::{JoinAll, join_all}
};
use quicksilver::{
    State, run,
    geom::{Circle, Positioned, Rectangle, Transform, Vector},
    graphics::{Draw, Color, Image, ImageLoader, Window, WindowBuilder, View},
    input::Key
};
use rand::Rng;

enum Meteors {
    Loading(JoinAll<Vec<ImageLoader>>),
    Loaded(GameState)
}

impl State for Meteors {
    fn new() -> Meteors {
        let assets = vec![
            Image::load("examples/assets/spaceship.png"),
            Image::load("examples/assets/space.png")
        ];
        Meteors::Loading(join_all(assets))
    }

    fn update(&mut self, window: &mut Window) {
       let result = match self {
           &mut Meteors::Loading(ref mut loader) => loader.poll().unwrap(),
           _ => Async::NotReady
       };
       if let Async::Ready(asset) = result {
           *self = Meteors::Loaded(GameState::new(asset));
       }
       if let &mut Meteors::Loaded(ref mut state) = self {
           state.update(window);
       }
    }

    fn draw(&mut self, window: &mut Window) {
        if let &mut Meteors::Loaded(ref mut state) = self {
           state.draw(window);
       }
    }
}

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

    fn draw(&self, image: &Image, window: &mut Window) {
        window.draw(&Draw::image(image, self.bounds.center())
            .with_transform(Transform::rotate(self.facing)));
    }
}

struct GameState {
    player: Entity,
    camera: Rectangle,
    player_image: Image,
    space_image: Image,
    meteors: Vec<Entity>,
    aliens: Vec<Entity>
}

impl GameState {
    fn new(images: Vec<Image>) -> GameState {
        let player_image = images[0].clone();
        let space_image = images[1].clone();
        GameState {
            player: Entity::new(Circle::newv(Vector::zero(), 16)),
            camera: Rectangle::new_sized(SCREEN_WIDTH, SCREEN_HEIGHT),
            player_image,
            space_image,
            meteors: Vec::new(),
            aliens: vec![Entity::new(Circle::new(50, 50, 20))]
        }
    }

    fn update(&mut self, window: &mut Window) {
        let mut rng = rand::thread_rng();
        for meteor in self.meteors.iter_mut() {
            meteor.facing += meteor.velocity.x + meteor.velocity.y;
            meteor.tick();
        }
        for alien in self.aliens.iter_mut() {
            alien.facing += 1.5;
            alien.velocity = if (alien.bounds.center() - self.player.bounds.center()).len() > 1500.0 { alien.velocity / 2 } else { alien.velocity };
            alien.velocity += (self.player.bounds.center() - alien.bounds.center()).with_len(ENEMY_IMPULSE);
            alien.tick();
        }
        let player_center = self.player.bounds.center();
        self.meteors.retain(|meteor| (meteor.bounds.center() - player_center).len() < 1500.0);
        while self.meteors.len() < 20 {
            let diff_x = rng.gen_range(450, 650) * rng.choose(&[-1, 1]).unwrap();
            let diff_y = rng.gen_range(350, 550) * rng.choose(&[-1, 1]).unwrap();
            let center = self.player.bounds.center()  + Vector::new(diff_x, diff_y);
            let mut entity = Entity::new(Circle::newv(center, 20));
            entity.velocity = Vector::new(rng.gen_range(-5, 5), rng.gen_range(-5, 5));
            self.meteors.push(entity);
        }
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
        let camera = self.camera.top_left();
        let scroll_offset = Vector::new(camera.x % BACKGROUND_TILE_SIZE as f32, camera.y % BACKGROUND_TILE_SIZE as f32);
        for x in 0..(SCREEN_WIDTH / BACKGROUND_TILE_SIZE) + 3 {
            for y in 0..(SCREEN_HEIGHT / BACKGROUND_TILE_SIZE) + 3 {
                let location = camera + Vector::new(x as i32 - 1, y as i32 - 1) * BACKGROUND_TILE_SIZE as i32 - scroll_offset;
                window.draw(&Draw::image(&self.space_image, location).with_z(-10));
            }
        }
        self.player.draw(&self.player_image, window);
        for alien in self.aliens.iter() {
            window.draw(&Draw::circle(alien.bounds).with_color(Color::green()));
        }
        for meteor in self.meteors.iter() {
            window.draw(&Draw::circle(meteor.bounds).with_color(Color { r: 0.5, g: 0.5, b: 0.0, a: 1.0 }));
        }
        window.present();
    }
}


const BACKGROUND_TILE_SIZE: u32 = 64;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PLAYER_ROTATION: f32 = 5.0;
const PLAYER_IMPULSE: f32 = 0.1;
const PLAYER_BREAK_FACTOR: f32 = 0.95;
const ENEMY_IMPULSE: f32 = 0.05;

fn main() {
    run::<Meteors>(WindowBuilder::new("Meteors", SCREEN_WIDTH, SCREEN_HEIGHT));
}
