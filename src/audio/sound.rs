extern crate rodio;

use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Cursor, Read};
use rodio::Source;

pub struct Sound {
    val: Cursor<Vec<u8>>
}

impl Sound {
    pub fn load<P: AsRef<Path>>(path: P) -> Sound {
        let mut bytes = Vec::new();
        BufReader::new(File::open(path).unwrap()).read_to_end(&mut bytes).unwrap();
        let val = Cursor::new(bytes);
        Sound {
            val
        }
    }

    pub fn play(&self) {
        let source = rodio::Decoder::new(self.val.clone()).unwrap();
        let endpoint = rodio::get_default_endpoint().unwrap();
        rodio::play_raw(&endpoint, source.convert_samples());
    }
}

