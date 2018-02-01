extern crate futures;
#[macro_use]
extern crate quicksilver;

use futures::{Async, Future, Poll};
use quicksilver::sound::{Sound, SoundLoader};
use std::time::Duration;

pub struct State {
    sound: SoundLoader,
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
            if let Async::Ready(sound) = self.sound.poll().unwrap() {
                self.played = true;
                sound.play();
            }
        }
    }
}

game_loop!(State);
