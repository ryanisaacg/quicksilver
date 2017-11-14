extern crate tiled;

pub use tiled::{Properties, TiledError};

use assets::AssetManager;
use geom::{Rectangle, Tile, Tilemap, Vector};
use graphics::{Color, TextureRegion};

use std::path::{Path, PathBuf};
use std::env;

pub struct Object {
    pub id: u32,
    pub texture: Option<TextureRegion>,
    pub name: String,
    pub obj_type: String,
    pub position: Vector,
    pub visible: bool,
    pub properties: Properties
}

pub struct ObjectGroup {
    pub name: String,
    pub visible: bool,
    pub objects: Vec<Object>,
    pub color: Option<Color>
}

pub struct Layer {
    pub name: String,
    pub opacity: f32,
    pub visible: bool,
    pub map: Tilemap<TextureRegion>,
    pub properties: Properties
}

pub struct Level {
    pub layers: Vec<Layer>,
    pub object_groups: Vec<ObjectGroup>,
    pub properties: Properties,
    pub background_color: Option<Color>
}

fn convert_col_opt(col: Option<tiled::Colour>, a: f32) -> Option<Color> {
    match col {
        Some(c) => Some(Color { r: c.red as f32 / 255.0, g: c.green as f32 / 255.0, b: c.blue as f32 / 255.0, a }),
        None => None
    }
}

impl Level {
    pub fn load(path: &Path, assets: &mut AssetManager) -> Result<Level, TiledError> {
        let current_dir = env::current_dir().unwrap();
        let tiled_map = tiled::parse_file(path)?;
        let mut search_dir = PathBuf::new();
        search_dir.push(current_dir.clone());
        search_dir.push(path);
        env::set_current_dir(search_dir.as_path().parent().unwrap()).unwrap();
        let mut textures: Vec<Option<TextureRegion>> = Vec::new();
        for tileset in tiled_map.tilesets.iter() {
            let mut current = tileset.first_gid as usize;
            for image in tileset.images.iter() {
                let margin = tileset.margin as i32;
                let tile_width = tileset.tile_width as i32;
                let tile_height = tileset.tile_height as i32;
                let mut x = margin;
                let mut y = margin;
                while x < image.width - margin {
                    while y < image.height - margin {
                        while textures.len() <= current {
                            textures.push(None);
                        }
                        let texture = assets.load_texture(&image.source);
                        let region =  Rectangle::newi(x, y, tile_width, tile_height);
                        textures[current] = Some(texture.region().subregion(region));
                        current += 1;
                        y += tile_height + tileset.spacing as i32;
                    }
                    x += tile_width + tileset.spacing as i32;
                }
            }
        }
        env::set_current_dir(current_dir).unwrap();
        Ok(Level {
            object_groups: tiled_map.object_groups.iter()
                .map(|group| ObjectGroup {
                    name: group.name.clone(),
                    visible: group.visible,
                    color: convert_col_opt(group.colour, group.opacity),
                    objects: group.objects.iter()
                        .map(|object| Object {
                            id: object.id,
                            texture: textures[object.gid as usize],
                            name: object.name.clone(),
                            obj_type: object.obj_type.clone(),
                            position: Vector::new(object.x, object.y),
                            visible: object.visible,
                            properties: object.properties.clone()
                        }).collect()
                }).collect(),
            layers: tiled_map.layers.iter()
                .map(|layer| Layer {
                    name: layer.name.clone(),
                    opacity: layer.opacity,
                    visible: layer.visible,
                    properties: layer.properties.clone(),
                    map: Tilemap::with_data(layer.tiles.iter()
                        .map(|vec| vec.iter().map(|n| if *n == 0 { Tile::empty(None) } else { Tile::solid(textures[*n as usize]) }).collect())
                        .fold(Vec::new(), |mut a, mut b| { a.append(&mut b); a }),
                        (tiled_map.width * tiled_map.tile_width) as f32, (tiled_map.height * tiled_map.tile_height) as f32, tiled_map.tile_width as f32, tiled_map.tile_height as f32)
                }).collect(),
            properties: tiled_map.properties,
            background_color: convert_col_opt(tiled_map.background_colour, 1.0)
        })
    }
}


