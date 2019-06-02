use crate::Result;
use super::{PlayState, SoundInstance, Sound, SoundError};
#[cfg(not(target_arch="wasm32"))]
use {
    rodio::{
        Sink,
        decoder::Decoder,
        source::{Source, Spatial}
    },
    std::io::Cursor
};

pub struct Bucket {
    #[cfg(not(target_arch="wasm32"))]
    sink: Sink,
    generation: u32
}

impl Bucket {
    pub fn new() -> Result<Bucket> {
        #[cfg(not(target_arch="wasm32"))]
        Ok(Bucket {
            sink: Sink::new(&rodio::default_output_device().ok_or(SoundError::NoOutputAvailable)?),
            generation: 0,
        })
    }

    pub fn available(&self) -> bool {
        #[cfg(not(target_arch="wasm32"))]
        self.sink.empty()
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }

    pub fn play(&mut self, sound: Sound, repeat: bool) -> Result<()> {
        #[cfg(not(target_arch="wasm32"))] {
            let cursor = Cursor::new(sound.data.val.as_slice());
            let decoder = Decoder::new(cursor)?;
            let source = Spatial::new(decoder
                .convert_samples()
                .amplify(sound.volume),
                [sound.balance, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
            if repeat {
                self.sink.append(source.repeat_infinite());
            } else {
                self.sink.append(source);
            }
        }
        self.generation += 1;
        Ok(())
    }

    pub fn pause(&mut self, sound: &SoundInstance) -> Result<()> {
        self.check_generation(sound)?;
        #[cfg(not(target_arch="wasm32"))] {
            self.sink.pause()
        }
        Ok(())
    }
    
    pub fn resume(&mut self, sound: &SoundInstance) -> Result<()> {
        self.check_generation(sound)?;
        #[cfg(not(target_arch="wasm32"))] {
            self.sink.play()
        }
        Ok(())
    }
    
    pub fn stop(&mut self, sound: &SoundInstance) -> Result<()> {
        self.check_generation(sound)?;
        #[cfg(not(target_arch="wasm32"))] {
            self.sink.stop();
        }
        Ok(())
    }

    fn is_paused(&self) -> bool {
        #[cfg(not(target_arch="wasm32"))] {
            self.sink.is_paused()
        }
        #[cfg(target_arch="wasm32")] {
            false
        }
    }

    pub fn state(&self, sound: &SoundInstance) -> PlayState {
        if sound.generation == self.generation {
            if self.is_paused() {
                PlayState::Paused
            } else {
                PlayState::Playing
            }
        } else {
            PlayState::Stopped
        }
    }

    fn check_generation(&self, sound: &SoundInstance) -> Result<()> {
        if self.generation != sound.generation {
            return Err(SoundError::AlreadyDone.into())
        } else {
            return Ok(())
        }
    }
}
/*    #[cfg(not(target_arch="wasm32"))]
    fn get_source(&self) -> Result<SamplesConverter<Amplify<Decoder<Cursor<Sound>>>, f32>> {
        Ok(Decoder::new(Cursor::new(self.clone()))?.amplify(self.volume).convert_samples())
    }*/
