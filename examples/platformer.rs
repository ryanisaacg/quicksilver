// Draw a pulsing circle in the middle of the window
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Vector},
    graphics::{Color, Sprite, Window, WindowBuilder},
    input::{Button, GamepadAxis, GamepadButton, Key}
};

struct Platformer {
    player: Circle
}

const PLAYER_SPEED: f32 = 3.0;

fn between(value: f32, min: f32, max: f32) -> bool {
    value >= min && value <= max
}

impl State for Platformer {
    fn configure() -> Window {
        WindowBuilder::new().build("Basic Platformer", 800, 600)
    }

   fn new() -> Platformer { 
       Platformer { 
           player: Circle::new(100, 100, 50)
       }
   }

   fn update(&mut self, window: &mut Window) {
       let mut velocity = Vector::zero();
       let left = window.keyboard()[Key::A].is_down()
            || window.gamepads().iter().any(|gamepad| gamepad[GamepadButton::DpadLeft].is_down() || between(gamepad[GamepadAxis::LeftStickX], -1.0, -0.2));
       let right = window.keyboard()[Key::D].is_down()
            || window.gamepads().iter().any(|gamepad| gamepad[GamepadButton::DpadRight].is_down() || between(gamepad[GamepadAxis::LeftStickX], 0.2, 1.0));
       if left && !right {
           velocity.x -= PLAYER_SPEED;
       }
       if !left && right {
           velocity.x += PLAYER_SPEED;
       }
       self.player = self.player.translate(velocity);
   }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        window.draw(&Sprite::circle(self.player).with_color(Color::blue()));
        window.present();
   }
}

fn main() {
    run::<Platformer>();
}
