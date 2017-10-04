use graphics::Texture;
use std::collections::HashMap;
use std::ops::Index;

pub struct AssetManager<'a> {
    loaded: HashMap<&'a str, Texture>
}

impl<'a> AssetManager<'a> {
    pub fn new() -> AssetManager<'a> {
        AssetManager {
            loaded: HashMap::new()
        }
    }
}

impl<'a> Index<&'a str> for AssetManager<'a> {
    type Output = &'a Texture;

    fn index(&mut self, path: &str) -> &'a Texture {
        match self.loaded.get(path) {
            Some(tex) => tex,
            None => {
                let texture = Texture::load(path);
                self.loaded.insert(path, texture);
                self[path]
            }
        }
    }
}
