#[cfg(not(target_arch="wasm32"))]
extern crate rodio;


use asset::{Loadable, LoadingAsset};
#[cfg(target_arch="wasm32")]
use asset::LoadingHandle;
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

#[cfg(target_arch="wasm32")]
extern "C" {
    fn load_sound(path: *mut i8) -> u32;
    fn play_sound(index: u32, volume: f32);
    fn set_music_track(index: u32);
    fn play_music();
    fn pause_music();
    fn get_music_volume() -> f32;
    fn set_music_volume(volume: f32);
}


#[derive(Clone)]
#[cfg(not(target_arch="wasm32"))]
pub struct Sound {
    val: Arc<Vec<u8>>,
    volume: f32
}

#[derive(Clone)]
#[cfg(target_arch="wasm32")]
pub struct Sound {
    index: u32,
    volume: f32
}


impl Sound {
    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> Result<Sound, SoundError> {
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
    
    #[cfg(target_arch="wasm32")]
    fn load_impl<P: AsRef<Path>>(path: P) -> u32 {
        use std::ffi::CString;
        unsafe { load_sound(CString::new(path.as_ref().to_str().unwrap()).unwrap().into_raw()) }
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    #[cfg(not(target_arch="wasm32"))]
    fn get_source(&self) -> SamplesConverter<Amplify<Decoder<Cursor<Sound>>>, f32> {
        Decoder::new(Cursor::new(self.clone())).unwrap().amplify(self.volume).convert_samples()
    }


    #[cfg(not(target_arch="wasm32"))]
    #[allow(deprecated)]
    pub fn play(&self) {
        let endpoint = rodio::get_default_endpoint().unwrap();
        rodio::play_raw(&endpoint, self.get_source());
    }
    
    #[cfg(target_arch="wasm32")]
    pub fn play(&self) {
        unsafe { play_sound(self.index, self.volume); }
    }
}

impl Loadable for Sound {
    type Error = SoundError;

    #[cfg(not(target_arch="wasm32"))]
    fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Self> {
        match Sound::load_impl(path) {
            Ok(snd) => LoadingAsset::Loaded(snd),
            Err(err) => LoadingAsset::Errored(err)
        }
    }

    #[cfg(target_arch="wasm32")]
    fn load<P: AsRef<Path>>(path: P) -> LoadingAsset<Self> {
        LoadingAsset::Loading(LoadingHandle(Sound::load_impl(path)))
    }

    #[cfg(target_arch="wasm32")]
    fn parse_result(handle: LoadingHandle, loaded: bool, errored: bool) -> LoadingAsset<Self> {
        if loaded {
            if errored {
                LoadingAsset::Errored(SoundError::IOError)
            } else {
                LoadingAsset::Loaded(Sound {
                    index: handle.0,
                    volume: 1.0
                })
            }
        } else {
            LoadingAsset::Loading(handle)
        }
    }
}

#[cfg(not(target_arch="wasm32"))]
impl AsRef<[u8]> for Sound {
    fn as_ref(&self) -> &[u8] {
        self.val.as_ref().as_ref()
    }
}

#[cfg(not(target_arch="wasm32"))]
pub struct MusicPlayer {
    sink: Sink
}

#[cfg(target_arch="wasm32")]
pub struct MusicPlayer;

#[cfg(not(target_arch="wasm32"))]
impl MusicPlayer {
    #[allow(deprecated)]
    pub fn new() -> MusicPlayer {
        MusicPlayer {
            sink: Sink::new(&rodio::get_default_endpoint().unwrap())
        }
    }

    pub fn set_track(&mut self, sound: &Sound) {
        self.sink.stop();
        self.sink.append(sound.get_source().repeat_infinite());
    }

    pub fn play(&self) {
        self.sink.play();
    }


    pub fn pause(&self) {
        self.sink.pause();
    }
    
    pub fn finished(&self) -> bool {
        self.sink.empty()
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }
}

#[cfg(target_arch="wasm32")]
impl MusicPlayer {
    pub fn new() -> MusicPlayer { MusicPlayer }

    pub fn set_track(&mut self, sound: &Sound) {
        unsafe { set_music_track(sound.index) };
    }

    pub fn play(&self) {
        unsafe { play_music() };
    }


    pub fn pause(&self) {
        unsafe { pause_music() };
    }
    
    pub fn volume(&self) -> f32 {
        unsafe { get_music_volume() }
    }

    pub fn set_volume(&mut self, volume: f32) {
        unsafe { set_music_volume(volume) };
    }
}


#[derive(Clone, Debug)]
pub enum SoundError {
     UnrecognizedFormat,
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
