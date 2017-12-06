use graphics::Texture;
use std::collections::HashMap;
use std::path::Path;
use super::AssetList;

pub struct Assets {
    loaded: HashMap<String, Texture>,
}

impl Assets {
    pub(crate) fn new<'a>(list: AssetList<'a>) -> Assets {
        let prefix = Path::new(list.texture_prefix);
        Assets {
            loaded: list.textures.iter()
                .map(|path| (path.to_string(), Texture::load(&prefix.join(Path::new(path))).unwrap()))
                .collect()
        }
    }

    pub fn get_texture<'a>(&'a self, path: &'a str) -> Option<&'a Texture> {
        self.loaded.get(path)
    }
}
