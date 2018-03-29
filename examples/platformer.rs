// Draw a pulsing circle in the middle of the window
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Vector, Transform},
    graphics::{Color, DrawCall, Window, WindowBuilder},
    input::{any, Button, Gamepad, GamepadAxis, GamepadButton, InputCheckable, Key, Keyboard}
};

struct Platformer {
    player: Circle
}

const PLAYER_SPEED: f32 = 3.0;

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
       let left = any(&[
           Button::Keyboard(Key::A),
           Button::GamepadButton((None, GamepadButton::DpadLeft)),
           Button::GamepadAxis((None, GamepadAxis::LeftStickX, -1.0, -0.2))
       ]);
       let right = any(&[
           Button::Keyboard(Key::D),
           Button::GamepadButton((None, GamepadButton::DpadRight)),
           Button::GamepadAxis((None, GamepadAxis::LeftStickX, 0.2, 1.5))
       ]);
       if left.satisfied(window) {
           velocity.x -= PLAYER_SPEED;
       }
       if right.satisfied(window) {
           velocity.x += PLAYER_SPEED;
       }
       self.player = self.player.translate(velocity);
   }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        window.draw(&[DrawCall::circle(self.player).with_color(Color::blue())]);
        window.present();
   }
}

fn main() {
    run::<Platformer>();
}
