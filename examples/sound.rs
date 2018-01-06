#[macro_use]
extern crate quicksilver;

use quicksilver::asset::{Loadable, LoadingAsset};
use quicksilver::sound::Sound;
use std::time::Duration;
use std::thread::sleep;

pub struct State {
    sound: LoadingAsset<Sound>,
    played: bool
}

impl State {
    pub fn new() -> State {
        State {
            sound: Sound::load("examples/boop.ogg"),
            played: false
        }
    }

    pub fn events(&mut self) -> bool {
        true
    }

    pub fn update(&mut self) -> Duration {
        Duration::from_millis(10)
    }

    pub fn draw(&mut self) {
        if !self.played {
            match self.sound {
                LoadingAsset::Loaded(ref sound) => {
                    self.played = true;
                    sound.play()
                },
                _ => ()
            }
        }
    }
}

game_loop!(State);
