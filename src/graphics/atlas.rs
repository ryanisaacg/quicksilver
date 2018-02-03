extern crate futures;

use futures::{Async, Future, Poll};
use futures::future::{JoinAll, join_all, Map, Then};
use geom::{Rectangle, Vector};
use graphics::{Image, ImageLoader, ImageError};
use util::FileLoader;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::path::Path;
use std::str::{FromStr, ParseBoolError, Split};

#[derive(Clone)]
struct Region {
    name: String,
    rotate: bool,
    region: Rectangle,
    center: Vector,
    index: i32
}

#[derive(Clone)]
/// An image atlas that allows a single image file to represent multiple individual images
///
/// It uses the libgdx / spine atlas format and groups them by name and index if applicable 
pub struct Atlas {
    data: HashMap<String, AtlasItem>
}

impl Atlas {
    /// Load an atlas at a given path
    pub fn load<P: AsRef<Path>>(path: P) -> AtlasLoader {
        AtlasLoader(FileLoader::load(path)
            .then(parse as fn(Result<String, ()>) -> ManifestLoader)
            .map(create as fn((Vec<Image>, Vec<Vec<Region>>)) -> Atlas))
    }
}

type ManifestContents = Result<(JoinAll<Vec<ImageLoader>>, Vec<Vec<Region>>), AtlasError>;
struct ManifestLoader(ManifestContents);

type Parser = Then<FileLoader, ManifestLoader, fn(Result<String, ()>) -> ManifestLoader>;
type Creator = Map<Parser, fn((Vec<Image>, Vec<Vec<Region>>)) -> Atlas>;

/// The Future used to load an Atlas object
pub struct AtlasLoader(Creator);

impl Future for ManifestLoader {
    type Item = (Vec<Image>, Vec<Vec<Region>>);
    type Error = AtlasError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0 {
            Ok(ref mut item) => match item.0.poll() {
                Ok(Async::Ready(images)) => Ok(Async::Ready((images, item.1.clone()))),
                Ok(Async::NotReady) => Ok(Async::NotReady),
                Err(err) => Err(err.into())
            }
            Err(ref err) => Err(err.clone())
        }
    }
}

impl Future for AtlasLoader {
    type Item = Atlas;
    type Error = AtlasError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

fn create(data: (Vec<Image>, Vec<Vec<Region>>)) -> Atlas {
    let (images, regions) = data;
    let mut images = images.into_iter()
        //Match each image with its list of regions
        .zip(regions.into_iter())
        //Convert each region into a subimage
        .map(|(image, region_list)| region_list.into_iter()
             //TODO: Take into account the center and rotation
             .map(|region| (region.name, region.index, image.subimage(region.region)))
             .collect())
        //Flatten all the subimages into a single list
        .fold(Vec::new(), |mut master_list, single_image_list: Vec<(String, i32, Image)>| {
            master_list.extend(single_image_list);
            master_list
        });
    //Sort the images by name, and then sub-sort them by index
    images.sort_by(|a: &(String, i32, Image), b: &(String, i32, Image)| match a.0.cmp(&b.0) { Ordering::Equal => a.1.cmp(&b.1), x => x });
    let data = images.into_iter()
        .fold(Vec::new(), |mut list: Vec<(String, AtlasItem)>, item: (String, i32, Image)| {
            let len = list.len();
            if len == 0 || list[len].0 != item.0 {
                list.push((item.0, AtlasItem::Image(item.2)));
            } else {
                let is_still = match list[len].1 { AtlasItem::Image(_) => true, _ => false };
                if is_still {
                    let image = match list[len].1 {
                        AtlasItem::Image(ref img) => img.clone(),
                        _ => unreachable!()
                    };
                    list[len] = (item.0, AtlasItem::Animation(vec![image]));
                }
                match list[len].1 {
                    AtlasItem::Animation(ref mut vec) => vec.push(item.2),
                    _ => unreachable!()
                }
            }
            list
        }).into_iter().collect();
    Atlas { data }
}

fn parse(data: Result<String, ()>) -> ManifestLoader {
    use std::iter::Map as IterMap;
    fn get_values_from_line<'a, T: FromStr>(line: &'a str) -> IterMap<Split<'a, &'static str>, fn(&'a str) -> T> 
        where <T as FromStr>::Err: Debug {
        fn parse<'a, T: FromStr>(item: &'a str) -> T where <T as FromStr>::Err: Debug { item.parse().unwrap() }
        let mut split = line.split(": ");
        split.next();
        split.next().unwrap().split(", ").map(parse)
    }

    fn getval<T, I: Iterator<Item = T>>(lines: &mut I) -> Result<T, AtlasError> {
        match lines.next() {
            Some(val) => Ok(val),
            None => Err(AtlasError::ParseError("Unexpected end of data"))?
        }
    }
    fn parse_body(data: String) -> ManifestContents {
        let mut lines = data.lines();
        let mut images = Vec::new();
        let mut regions = Vec::new();
        while let Some(line) = lines.next() {
            //TODO: make relative to atlas location
            images.push(Image::load(line));
            regions.push(Vec::new());
            //Skip some lines the loader doesn't use
            for _ in 0..4 {
                getval(&mut lines)?;
            }
            while let Some(line) = lines.next() {
                //If there's an empty line, move onto a different page
                if line.len() == 0 { break; }
                let name = line.to_owned();
                let mut rotate = get_values_from_line(getval(&mut lines)?);
                let mut xy = get_values_from_line::<i32>(getval(&mut lines)?);
                let mut size = get_values_from_line(getval(&mut lines)?);
                //Skip more &mut lines the loader doesn't use
                for _ in 0..2 {
                    getval(&mut lines)?;
                }
                let mut orig = get_values_from_line::<i32>(getval(&mut lines)?);
                let mut offset = get_values_from_line::<i32>(getval(&mut lines)?);
                let index = getval(&mut get_values_from_line(getval(&mut lines)?))?;
                let rotate = getval(&mut rotate)?;
                let region = Rectangle::new(getval(&mut xy)?, getval(&mut xy)?, getval(&mut size)?, getval(&mut size)?);
                let original_size = Vector::new(getval(&mut orig)?, getval(&mut orig)?);
                let offset = Vector::new(getval(&mut offset)?, getval(&mut offset)?);
                let center = region.center() + (original_size - region.size() - offset.x_comp() + offset.y_comp());
                regions.last_mut().unwrap().push(Region { name, region, rotate, center, index });
            }
        }
        Ok((join_all(images), regions))
    }
    match data {
        Ok(data) => ManifestLoader(parse_body(data)),
        Err(err) => ManifestLoader(Err(err.into()))
    }
}


#[derive(Clone)]
pub enum AtlasItem {
    Image(Image),
    Animation(Vec<Image>)
}

#[derive(Clone, Debug)]
pub enum AtlasError {
    ImageError(ImageError),
    ParseError(&'static str),
    IoError
}

impl From<ImageError> for AtlasError {
    fn from(err: ImageError) -> AtlasError {
        AtlasError::ImageError(err)
    }
}

impl From<ParseIntError> for AtlasError {
    fn from(_: ParseIntError) -> AtlasError {
        AtlasError::ParseError("Failed to parse an int")
    }
}

impl From<ParseBoolError> for AtlasError {
    fn from(_: ParseBoolError) -> AtlasError {
        AtlasError::ParseError("Failed to parse an bool")
    }
}

impl From<()> for AtlasError {
    fn from(_: ()) -> AtlasError {
        AtlasError::IoError
    }
}
