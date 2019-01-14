use crate::sound::{Bucket, Sound};
use std::{
    error::Error,
    fmt,
    io::Error as IOError,
};

pub struct PlayingSound {
    bucket: u32,
    generation: u32
}

pub enum PlayingState {
    Playing, Paused, Stopped
}

pub struct Player {
    buckets: Vec<Bucket>
}

impl Player {
    pub fn new() -> Player {
        Player {
            buckets: Vec::new()
        }
    }

    pub fn state(&self, sound: PlayingSound) -> PlayingState {
        let bucket = &self.buckets[sound.bucket];
        if sound.generation == bucket.get_generation() {
            if bucket.is_paused() {
                PlayingState::Paused
            } else {
                PlayingState::Playing
            }
        } else {
            PlayingState::Stopped
        }
    }

    pub fn play(&mut self, clip: Sound) -> Result<PlayingSound> {

    }

    pub fn repeat(&mut self, clip: Sound) -> Result<PlayingSound> {

    }

    pub fn pause(&mut self, sound: PlayingSound) -> Result<()> {

    }

    pub fn resume(&mut self, sound: PlayingSound) -> Result<()> {

    }

    pub fn stop(&mut self, sound: PlayingSound) -> Result<()> {

    }
}
    
#[cfg(not(target_arch="wasm32"))]
    //Play a silent sound so rodio startup doesn't interfere with application
    //Unfortunately this means even apps that don't use sound eat the startup penalty but it's not a
    //huge one
    pub(crate) fn initialize() {
        if let Some(ref device) = rodio::default_output_device() {
            rodio::play_raw(device, rodio::source::Empty::new())
        }
    }

pub fn get_player() -> Player {

}

#[cfg(target_arch="wasm32")]
fn wasm_sound_error(error: &str) -> QuicksilverError {
    let error = IOError::new(ErrorKind::NotFound, error);
    let error: SoundError = error.into();
    error.into()
}

pub struct SoundData {
    #[cfg(not(target_arch="wasm32"))]
    val: Vec<u8>>,
    #[cfg(target_arch="wasm32")]
    sound: Value,
}
