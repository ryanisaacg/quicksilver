use asset::{Asset, LoadingAsset};
use geom::Rectangle;
use graphics::{Animation, Image, ImageError};
use std::collections::HashMap;
use std::io::{Error, File, Read};

pub struct Atlas {
    data: HashMap<String, AtlasItem>
}

impl Atlas {
    fn parse(data: &str) -> Result<Atlas, AtlasError> {
        let mut lines = data.lines();
        let data = HashMap::new();
        while let Some(line) = lines.next() {
            let image = Image::load
        }
        Ok(Atlas { data })
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Atlas, AtlasError> {
        Atlas::load_impl(path)
    }

    #[cfg(not(target_arch="wasm32"))]
    fn load_impl<P: AsRef<Path>>(path: P) -> Result<Atlas, AtlasError> {
        let data = String::new();
        File::open(path).read_to_string(&mut data)?;
        Atlas::parse(data.to_str())
    }    
}

pub enum AtlasItem {
    Image(Image),
    Animation(Animation)
}

pub enum AtlasError {
    ImageError(ImageError),
    IoError
}

impl Asset for Atlas {
    type Loading = (Vec<String>, Vec<LoadingAsset<AtlasItem>>);
    type Error = Vec<AtlasError>;

    fn update(loading: Self::Loading) -> LoadingAsset<Self> {

    }
}
