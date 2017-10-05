use graphics::Texture;
use std::collections::HashMap;
use std::ops::Index;
use std::path::Path;

pub struct AssetManager {
    loaded: HashMap<String, Texture>
}

impl AssetManager {
    pub fn new() -> AssetManager {
        AssetManager {
            loaded: HashMap::new()
        }
    }

    fn load_texture(&mut self, path: &str) -> &Texture {
        if self.loaded.contains_key(path) {
            match self.loaded.get(path) {
                Some(tex) => &tex,
                None => panic!("Failed to retrieve texture")
            }
        } else {
            let texture = Texture::load(Path::new(path)).unwrap();
            self.loaded.insert(path.to_string(), texture);
            self.load_texture(&path)
        }
    }
}

