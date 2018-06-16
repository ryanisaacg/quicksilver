//! A sound API that allows playing clips at given volumes
//!
//! On the desktop, currently all sounds are loaded into memory, but streaming sounds may be
//! introduced in the future. On the web, it can be different from browser to browser

extern crate futures;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;

use error::QuicksilverError;
use futures::{Async, Future, Poll};
use std::{
    error::Error,
    fmt,
    io::Error as IOError,
    path::Path
};
#[cfg(not(target_arch="wasm32"))]
use {
    rodio::{
        Decoder, 
        Sink, 
        Source,
        decoder::DecoderError,
        source::{SamplesConverter, Amplify},
    },
    std::{
        fs::File,
        io::{BufReader, Cursor, Read},
        path::PathBuf,
        sync::Arc
    }
};
#[cfg(target_arch="wasm32")]
use {
    std::io::ErrorKind,
    stdweb::{
        unstable::TryInto,
        Reference
    }
};


/// A clip of sound, which may be streamed from disc or stored in memory
///
/// It can be played an arbitrary amount of times and concurrently with itself, meaning you don't
/// need more than one instance of a clip. However, if you want different clips with different
/// volumes, you can clone the Sound.
#[derive(Clone, Debug)]
pub struct Sound {
    #[cfg(not(target_arch="wasm32"))]
    val: Arc<Vec<u8>>,
    #[cfg(target_arch="wasm32")]
    sound: Reference,
    volume: f32
}

impl Sound {
    /// Start loading a sound from a given path
    pub fn load<P: AsRef<Path>>(path: P) -> SoundLoader {
        Sound::load_impl(path)
    }

    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> SoundLoader {
        SoundLoader {
            path: PathBuf::from(path.as_ref())
        }
    }

    #[cfg(target_arch="wasm32")]
    fn load_impl<P: AsRef<Path>>(path: P) -> SoundLoader {
        let sound = js! ( new Audio(@{path.as_ref().to_str().unwrap()}); );
        let sound = sound.try_into().unwrap();
        SoundLoader { sound }
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
        #[cfg(not(target_arch="wasm32"))] {
            let endpoint = rodio::default_endpoint().unwrap();
            rodio::play_raw(&endpoint, self.get_source());
        }
        #[cfg(target_arch="wasm32")] js! {
            @{&self.sound}.play();
        }
    }
    
    #[cfg(not(target_arch="wasm32"))]
    //Play a silent sound so rodio startup doesn't interfere with application
    //Unfortunately this means even apps that don't use sound eat the startup penalty but it's not a
    //huge one
    pub(crate) fn initialize() {
        if let Some(ref endpoint) = rodio::default_endpoint() {
            rodio::play_raw(endpoint, rodio::source::Empty::new())
        }
    }
}

/// A future for loading images
pub struct SoundLoader { 
    #[cfg(not(target_arch="wasm32"))]
    path: PathBuf,
    #[cfg(target_arch="wasm32")]
    sound: Reference
}

impl Future for SoundLoader {
    type Item = Sound;
    type Error = QuicksilverError;

    #[cfg(not(target_arch="wasm32"))]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut bytes = Vec::new();
        BufReader::new(File::open(&self.path)?).read_to_end(&mut bytes)?;
        let val = Arc::new(bytes);
        let sound = Sound {
            val,
            volume: 1f32
        };
        Decoder::new(Cursor::new(sound.clone()))?;
        Ok(Async::Ready(sound))
    }

    #[cfg(target_arch="wasm32")]
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if js! ( @{&self.sound}.networkState == 3 ).try_into().unwrap() {
            let error = IOError::new(ErrorKind::NotFound, "Sound not found");
            let error = SoundError::IOError(error);
            let error = QuicksilverError::SoundError(error);
            Err(error)
        } else if js! ( @{&self.sound}.readyState == 4).try_into().unwrap() {
            let sound = self.sound.clone();
            Ok(Async::Ready(Sound {
                sound,
                volume: 1f32
            }))
        } else {
            Ok(Async::NotReady)
        }
    }
}

#[doc(hidden)]
#[cfg(not(target_arch="wasm32"))]
impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        self.val.as_ref().as_ref()
    }
}

//TODO: Wasm music player
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
    }

    /// Resume the player if it is paused
    pub fn play(&self) {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.play();
    }

    /// Pause the player
    pub fn pause(&self) {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.pause();
    }

    #[cfg(not(target_arch="wasm32"))]
    fn volume_impl(&self) -> f32 { self.sink.volume() }

    #[cfg(target_arch="wasm32")]
    fn volume_impl(&self) -> f32 { 
        1.0
    }

    /// Get the volume the song is playing at, see Sound::volume for more
    pub fn volume(&self) -> f32 {
        self.volume_impl()
    }

    /// Set the volume the song is playing at, changing the currently playing song
    pub fn set_volume(&mut self, volume: f32) {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.set_volume(volume);
    }
}

#[derive(Debug)]
///An error generated when loading a sound
pub enum SoundError {
    ///The sound file is not in an format that can be played
    UnrecognizedFormat,
    ///The Sound was not found or could not be loaded
    IOError(IOError)
}

impl fmt::Display for SoundError  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for SoundError {
    fn description(&self) -> &str {
        match self {
            &SoundError::UnrecognizedFormat => "The sound file format was not recognized",
            &SoundError::IOError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &SoundError::UnrecognizedFormat => None,
            &SoundError::IOError(ref err) => Some(err)
        }
    }

}

#[doc(hidden)]
#[cfg(not(target_arch="wasm32"))]
impl From<DecoderError> for SoundError {
    fn from(err: DecoderError) -> SoundError {
        match err {
            DecoderError::UnrecognizedFormat => SoundError::UnrecognizedFormat
        }
    }
}

#[doc(hidden)]
impl From<IOError> for SoundError {
    fn from(err: IOError) -> SoundError {
        SoundError::IOError(err)
    }
}

