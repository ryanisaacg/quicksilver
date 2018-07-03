// Play a sound when a button is clicked
extern crate futures;
extern crate quicksilver;

use quicksilver::{
    Asset, Result, State, run,
    geom::Rectangle,
    graphics::{Color, Sprite, Window, WindowBuilder},
    input::{ButtonState, MouseButton},
    sound::Sound
};

struct SoundPlayer {
    asset: Asset<Sound>
}

const BUTTON_AREA: Rectangle = Rectangle { x: 350.0, y: 250.0, width: 100.0, height: 100.0 };

impl State for SoundPlayer {
   fn new() -> Result<SoundPlayer> {
       let asset = Asset::new(Sound::load("examples/assets/boop.ogg"));
       Ok(SoundPlayer {
           asset
       })
    }

   fn update(&mut self, window: &mut Window) -> Result<()> {
       self.asset.execute(|sound| {
            if window.mouse()[MouseButton::Left] == ButtonState::Pressed && BUTTON_AREA.contains(window.mouse().pos()) {
                sound.play();
            }
            Ok(())
       })
   }

   fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::white());
        // If the sound is loaded, draw the button
        self.asset.execute(|_| {
            window.draw(&Sprite::rectangle(BUTTON_AREA).with_color(Color::blue()));
            Ok(())
        })?;
        window.present()
   }
}

fn main() {
    run::<SoundPlayer>(WindowBuilder::new("Sound Example", 800, 600)).unwrap();
}
