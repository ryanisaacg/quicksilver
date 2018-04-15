// Play a sound when a button is clicked
extern crate futures;
extern crate quicksilver;

use futures::{Async, Future};
use quicksilver::{
    State, run,
    geom::Rectangle,
    graphics::{Color, Sprite, Window, WindowBuilder},
    input::{ButtonState, MouseButton},
    sound::{Sound, SoundLoader}
};

enum SoundPlayer {
    Loading(SoundLoader),
    Loaded(Sound)
}

const BUTTON_AREA: Rectangle = Rectangle { x: 350.0, y: 250.0, width: 100.0, height: 100.0 };

impl State for SoundPlayer {
    fn configure() -> WindowBuilder {
        WindowBuilder::new("Sound Example", 800, 600)
    }

   fn new() -> SoundPlayer { SoundPlayer::Loading(Sound::load("examples/assets/boop.ogg")) }

   fn update(&mut self, window: &mut Window) {
       // Check to see the progress of the loading sound 
       let result = match self {
           &mut SoundPlayer::Loading(ref mut loader) => loader.poll().unwrap(),
           _ => Async::NotReady
       };
       // If the sound has been loaded move to the loaded state
       if let Async::Ready(asset) = result {
           *self = SoundPlayer::Loaded(asset);
       }
       if let &mut SoundPlayer::Loaded(ref sound) = self {
            if window.mouse()[MouseButton::Left] == ButtonState::Pressed && BUTTON_AREA.contains(window.mouse().pos()) {
                sound.play();
            }
       }
   }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::white());
        // If the sound is loaded, draw the button
        if let &mut SoundPlayer::Loaded(_) = self {
            window.draw(&Sprite::rectangle(BUTTON_AREA).with_color(Color::blue()));
        }
        window.present();
   }
}

fn main() {
    run::<SoundPlayer>();
}
