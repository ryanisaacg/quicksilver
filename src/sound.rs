//! A sound API that allows playing clips at given volumes
//!
//! On the desktop, currently all sounds are loaded into memory, but streaming sounds may be
//! introduced in the future. On the web, it can be different from browser to browser

extern crate futures;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;

use futures::{Async, Future, Poll};
#[cfg(not(target_arch="wasm32"))]
use rodio::{Decoder, Sink, Source};
#[cfg(not(target_arch="wasm32"))]
use rodio::decoder::DecoderError;
#[cfg(not(target_arch="wasm32"))]
use rodio::source::{SamplesConverter, Amplify};
#[cfg(not(target_arch="wasm32"))]
use std::fs::File;
use std::path::Path;
#[cfg(not(target_arch="wasm32"))]
use std::io::{BufReader, Cursor, Error as IOError, Read};
#[cfg(not(target_arch="wasm32"))]
use std::sync::Arc;


/// A clip of sound, which may be streamed from disc or stored in memory
///
/// It can be played an arbitrary amount of times and concurrently with itself, meaning you don't
/// need more than one instance of a clip. However, if you want different clips with different
/// volumes, you can clone the Sound.
#[derive(Clone)]
pub struct Sound {
    #[cfg(not(target_arch="wasm32"))]
    val: Arc<Vec<u8>>,
    #[cfg(target_arch="wasm32")]
    index: u32,
    volume: f32
}

impl Sound {
    /// Start loading a sound from a given path
    pub fn load<P: AsRef<Path>>(path: P) -> SoundLoader {
        Sound::load_impl(path)
    }

    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> SoundLoader {
        fn load_impl_data<P: AsRef<Path>>(path: P) -> Result<Sound, SoundError> {
            let mut bytes = Vec::new();
            BufReader::new(File::open(path)?).read_to_end(&mut bytes)?;
            let val = Arc::new(bytes);
            let sound = Sound {
                val,
                volume: 1f32
            };
            Decoder::new(Cursor::new(sound.clone()))?;
            Ok(sound)
        }
        SoundLoader {
            sound: load_impl_data(path)
        }
    }

    #[cfg(target_arch="wasm32")]
    fn load_impl<P: AsRef<Path>>(path: P) -> SoundLoader {
        use ffi::wasm;
        use std::ffi::CString;
        SoundLoader {
            id: unsafe { wasm::load_sound(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
        }
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

    #[cfg(not(target_arch="wasm32"))]
    fn get_source(&self) -> SamplesConverter<Amplify<Decoder<Cursor<Sound>>>, f32> {
        Decoder::new(Cursor::new(self.clone())).unwrap().amplify(self.volume).convert_samples()
    }

    /// Play the sound clip at its current volume
    ///
    /// The sound clip can be played over itself.
    ///
    /// Future changes in volume will not change the sound emitted by this method.
    pub fn play(&self) {
        #[cfg(not(target_arch="wasm32"))]
        #[allow(deprecated)] {
            let endpoint = rodio::get_default_endpoint().unwrap();
            rodio::play_raw(&endpoint, self.get_source());
        }
        #[cfg(target_arch="wasm32")] {
            use ffi::wasm;
            unsafe { wasm::play_sound(self.index, self.volume); }
        }
    }
}

/// A future for loading images
pub struct SoundLoader { 
    #[cfg(not(target_arch="wasm32"))]
    sound: Result<Sound, SoundError>,
    #[cfg(target_arch="wasm32")]
    id: u32
}

impl Future for SoundLoader {
    type Item = Sound;
    type Error = SoundError;

    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<Sound, SoundError> {
        Ok(Async::Ready(self.sound.clone()?))
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<Sound, SoundError> {
        use ffi::wasm;
        if unsafe { wasm::is_loaded(self.id) } {
            if unsafe { wasm::is_errored(self.id) } {
                Err(SoundError::IOError)
            } else {
                Ok(Async::Ready(Sound {
                    index: self.id,
                    volume: 1.0
                }))
            }
        } else {
            Ok(Async::NotReady)
        }
    }
}

#[cfg(not(target_arch="wasm32"))]
impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        self.val.as_ref().as_ref()
    }
}

/// A music player that loops a single track indefinitely
///
/// The music player has its own internal volume and will adjust the sound of the music if its
/// volume is changed. 
pub struct MusicPlayer {
    #[cfg(not(target_arch="wasm32"))]
    sink: Sink
}

impl MusicPlayer {
    /// Create a new music player with the default volume of 1
    pub fn new() -> MusicPlayer {
        #[allow(deprecated)]
        MusicPlayer {
            #[cfg(not(target_arch="wasm32"))]
            sink: Sink::new(&rodio::get_default_endpoint().unwrap())
        }
    }

    /// Set the sound that should be playing
    ///
    /// If there already is a playing song, it will be stopped and replaced. The volume of the
    /// parameter is ignored, in favor of the volume from the player itself.
    pub fn set_track(&mut self, sound: &Sound) {
        #[cfg(not(target_arch="wasm32"))] {
            self.sink.stop();
            self.sink.append(sound.get_source().repeat_infinite());
        }
        #[cfg(target_arch="wasm32")] {
            use ffi::wasm;
            unsafe { wasm::set_music_track(sound.index) };
        }
    }

    /// Resume the player if it is paused
    pub fn play(&self) {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.play();
        #[cfg(target_arch="wasm32")] {
            use ffi::wasm;
            unsafe { wasm::play_music() };
        }
    }

    /// Pause the player
    pub fn pause(&self) {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.pause();
        #[cfg(target_arch="wasm32")] {
            use ffi::wasm;
            unsafe { wasm::pause_music() };
        }
    }

    #[cfg(not(target_arch="wasm32"))]
    fn volume_impl(&self) -> f32 { self.sink.volume() }

    #[cfg(target_arch="wasm32")]
    fn volume_impl(&self) -> f32 { 
        use ffi::wasm;
        unsafe { wasm::get_music_volume() } 
    }

    /// Get the volume the song is playing at, see Sound::volume for more
    pub fn volume(&self) -> f32 {
        self.volume_impl()
    }

    /// Set the volume the song is playing at, changing the currently playing song
    pub fn set_volume(&mut self, volume: f32) {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.set_volume(volume);
        #[cfg(target_arch="wasm32")] {
            use ffi::wasm;
            unsafe { wasm::set_music_volume(volume) };
        }
    }
}

#[derive(Clone, Debug)]
///An error generated when loading a sound
pub enum SoundError {
    ///The sound file is not in an format that can be played
    UnrecognizedFormat,
    ///The Sound was not found or could not be loaded
    IOError
}

#[cfg(not(target_arch="wasm32"))]
impl From<DecoderError> for SoundError {
    fn from(err: DecoderError) -> SoundError {
        match err {
            DecoderError::UnrecognizedFormat => SoundError::UnrecognizedFormat
        }
    }
}

#[cfg(not(target_arch="wasm32"))]
impl From<IOError> for SoundError {
    fn from(_: IOError) -> SoundError {
        SoundError::IOError
    }
}
