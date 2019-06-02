//! A sound API that allows playing clips at given volumes
//!
//! On the desktop, currently all sounds are loaded into memory, but streaming sounds may be
//! introduced in the future. On the web, it can be different from browser to browser
mod bucket;
mod error;
mod player;
mod instance;
mod play_state;

use crate::{
    Result, load_file,
    error::QuicksilverError,
};
use self::{
    bucket::Bucket,
    player::{SoundData, Player, get_player}
};
pub use self::{
    error::SoundError,
    instance::SoundInstance,
    play_state::PlayState,
};
use futures::{Future, future};
use std::{
    path::Path,
    rc::Rc
};


/// A clip of sound, which may be streamed from disc or stored in memory
///
/// It can be played an arbitrary amount of times and concurrently with itself, meaning you don't
/// need more than one instance of a clip. However, if you want different clips with different
/// volumes, you can clone the Sound.
#[derive(Clone, Debug)]
pub struct Sound {
    data: Rc<SoundData>,
    volume: f32,
    balance: f32, // left / right balancing
}

impl Sound {
    pub fn from_bytes(raw: &[u8]) -> Result<Sound> {
        let data = Rc::new(get_player().from_bytes(raw)?);
        Ok(Sound {
            data,
            volume: 1.0,
            balance: 0.0
        })
    }

    pub fn load(path: impl AsRef<Path>) -> impl Future<Item = Sound, Error = QuicksilverError> {
        load_file(path)
            .map(|data| Sound::from_bytes(data.as_slice()))
            .and_then(future::result)
    }

    /// Get the volume of the sound clip instance
    ///
    /// The volume is multiplicative, meaing 1 is the identity, 0 is silent, 2 is twice the
    /// amplitude, etc. Note that sound is not perceived linearly so results may not correspond as
    /// expected.
    pub fn volume(&self) -> f32 {
        self.volume
    }

    /// Set the volume of the sound clip instance
    ///
    /// The volume is multiplicative, meaing 1 is the identity, 0 is silent, 2 is twice the
    /// amplitude, etc. Note that sound is not perceived linearly so results may not correspond as
    /// expected.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn balance(&self) -> f32 {
        self.balance
    }
    
    pub fn set_balance(&mut self, balance: f32) {
        self.balance = balance;
    }

    /// Play the sound clip at its current volume
    ///
    /// The sound clip can be played over itself.
    ///
    /// Future changes in volume will not change the sound emitted by this method.
    pub fn play(&self) -> Result<SoundInstance> {
        get_player().play(self, false)
    }

    pub fn repeat(&self) -> Result<SoundInstance> {
        get_player().play(self, true)
    }
}

