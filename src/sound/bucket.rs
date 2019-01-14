#[cfg(not(target_arch="wasm32"))]
use {
    rodio::{
        self,
        decoder::{Decoder, DecoderError},
        source::{SamplesConverter, Source,Amplify},
    },
    std::{
        fs::File,
        io::{Cursor, Read},
        sync::Arc
    }
};
#[cfg(target_arch="wasm32")]
use {
    futures::Async,
    std::io::ErrorKind,
    stdweb::{
        unstable::TryInto,
        Value
    }
};

pub struct Bucket {
}
    #[cfg(not(target_arch="wasm32"))]
    fn get_source(&self) -> Result<SamplesConverter<Amplify<Decoder<Cursor<Sound>>>, f32>> {
        Ok(Decoder::new(Cursor::new(self.clone()))?.amplify(self.volume).convert_samples())
    }
