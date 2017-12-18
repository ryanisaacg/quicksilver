extern crate quicksilver;

use quicksilver::Sound;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let sound = Sound::load("examples/boop.ogg").unwrap();
    sound.play();
    sleep(Duration::from_secs(1));
}
