pub struct AssetList<'a> {
    pub(crate) texture_prefix: &'a str,
    pub(crate) textures: Vec<&'a str>
}

impl<'a> AssetList<'a> {
    pub fn new() -> Self {
        AssetList::new_prefixed("")
    }

    pub fn new_prefixed(texture_prefix: &'a str) -> Self {
        AssetList {
            texture_prefix,
            textures: Vec::new()
        }
    }

    pub fn load_texture(&mut self, texture: &'a str) {
        if !self.textures.contains(&texture) {
            self.textures.push(texture)
        }
    }

    pub fn load_textures(&mut self, textures: &[&'a str]) {
        for texture in textures.iter() {
            self.load_texture(texture)
        }
    }
}
