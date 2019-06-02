use crate::Result;
use super::{Bucket, PlayState, Sound, SoundInstance};
#[cfg(target_arch = "wasm32")]
use stdweb::{
    Value,
    web::TypedArray,
};

pub struct Player {
    buckets: Vec<Bucket>,
    #[cfg(target_arch = "wasm32")]
    context: Value
}

impl Player {
    pub fn new() -> Player {
        Player {
            buckets: Vec::new(),
            // TODO
            #[cfg(target_arch = "wasm32")]
            context: js! {
                return null;
            }
        }
    }

    pub fn from_bytes(&self, raw: &[u8]) -> Result<SoundData> {
        #[cfg(not(target_arch = "wasm32"))] {
            Ok(SoundData {
                val: raw.to_vec()
            })
        }
        #[cfg(target_arch = "wasm32")] {
            let array: TypedArray<u8> = raw.into();
            let buffer = array.buffer();
            let context = &self.context;
            Ok(SoundData {
                sound: js! {
                    var data = { sound: null, error: null };
                    @{context}.decodeAudioData(@{buffer})
                        .then(sound => data.sound = sound)
                        .catch(error => data.error = error)
                }
            })
        }
    }

    pub fn state(&self, sound: &SoundInstance) -> PlayState {
        self.buckets[sound.bucket].state(sound)
    }

    pub fn play(&mut self, sound: &Sound, repeat: bool) -> Result<SoundInstance> {
        let index = self.buckets.iter()
            .enumerate()
            .filter(|(index, bucket)| (*bucket).available())
            .next()
            .map(|(idx, _)| Ok(idx))
            .unwrap_or_else(|| {
                self.buckets.push(Bucket::new()?);
                Ok(self.buckets.len() - 1)
            })?;
        self.buckets[index].play(sound.clone(), repeat)?;
        Ok(SoundInstance {
            bucket: index,
            generation: self.buckets[index].generation()
        })
    }

    pub fn pause(&mut self, sound: &SoundInstance) -> Result<()> {
        self.buckets[sound.bucket].pause(sound)
    }

    pub fn resume(&mut self, sound: &SoundInstance) -> Result<()> {
        self.buckets[sound.bucket].pause(sound)
    }

    pub fn stop(&mut self, sound: &SoundInstance) -> Result<()> {
        self.buckets[sound.bucket].stop(sound)
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

#[derive(Debug)]
pub struct SoundData {
    #[cfg(not(target_arch = "wasm32"))]
    pub val: Vec<u8>,
    #[cfg(target_arch = "wasm32")]
    pub sound: Value,
}
