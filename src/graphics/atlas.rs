extern crate futures;

use error::QuicksilverError;
use futures::{Async, Future, Poll};
use futures::future::{JoinAll, join_all};
use geom::{Positioned, Rectangle, Vector};
use graphics::{Image, ImageLoader, ImageError};
use util::FileLoader;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Debug,
    io::ErrorKind as IOError,
    num::ParseIntError,
    path::Path,
    str::{FromStr, ParseBoolError, Split}
};

#[derive(Clone)]
struct Region {
    image: usize,
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
    pub fn load<'a, P: 'static + AsRef<Path>>(path: P) -> AtlasLoader {
        AtlasLoader(Box::new(FileLoader::load(path.as_ref())
            .map(|bytes| String::from_utf8(bytes).unwrap())
            .then(|data| parse(data, path))
            .map(create)))
    }

    /// Get an image or animation with a given name
    pub fn get(&self, name: &str) -> Option<AtlasItem> {
        Some(self.data.get(name)?.clone())
    }
}

type ManifestContents = Result<(JoinAll<Vec<ImageLoader>>, Vec<Region>), QuicksilverError>;
struct ManifestLoader(ManifestContents);

/// A Future to load an Atlas
pub struct AtlasLoader(Box<Future<Item=Atlas, Error=QuicksilverError>>);

impl Future for ManifestLoader {
    type Item = (Vec<Image>, Vec<Region>);
    type Error = QuicksilverError;

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
    type Error = QuicksilverError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

// Turn the pages and regions into an Atlas
fn create(data: (Vec<Image>, Vec<Region>)) -> Atlas {
    let (images, regions) = data;
    let mut images = regions.into_iter()
        .map(|region| 
             (region.name, region.index, images[region.image].subimage(region.region)))
        .collect::<Vec<(String, i32, Image)>>();
    //Sort the images by name, and then sub-sort them by index
    images.sort_by(|a: &(String, i32, Image), b: &(String, i32, Image)| match a.0.cmp(&b.0) { Ordering::Equal => a.1.cmp(&b.1), x => x });
    let data = images.into_iter()
        .fold(Vec::new(), |mut list: Vec<(String, AtlasItem)>, item: (String, i32, Image)| {
            let len = list.len();
            // There are no previous items or the previous item is a different name
            if len == 0 || list[len - 1].0 != item.0 {
                list.push((item.0, AtlasItem::Image(item.2)));
            } else {
                // If the previous entry is a still frame, convert it into a 1-sized animation
                let is_still = match list[len - 1].1 { AtlasItem::Image(_) => true, _ => false };
                if is_still {
                    let image = match list[len - 1].1 {
                        AtlasItem::Image(ref img) => img.clone(),
                        _ => unreachable!()
                    };
                    list[len - 1] = (item.0, AtlasItem::Animation(vec![image]));
                }
                // Add the new frame to the animation
                match list[len - 1].1 {
                    AtlasItem::Animation(ref mut vec) => vec.push(item.2),
                    _ => unreachable!()
                }
            }
            list
        }).into_iter().collect::<HashMap<String, AtlasItem>>();
    Atlas { data }
}

// Parse a manifest file into a future to load the contents of the atlas
fn parse<P: AsRef<Path>>(data: Result<String, QuicksilverError>, path: P) -> ManifestLoader {
    // Either parse the data or repackage the error
    return match data {
        Ok(data) => ManifestLoader(parse_body(data, path.as_ref())),
        Err(err) => ManifestLoader(Err(err.into()))
    };
    fn parse_body(data: String, path: &Path) -> ManifestContents {
        use std::path::PathBuf;
        use std::iter::Map as IterMap;
        // Split a line into its right-hand values
        fn get_values_from_line<'a, T: FromStr>(line: &'a str) -> IterMap<Split<'a, &'static str>, fn(&'a str) -> T> 
            where <T as FromStr>::Err: Debug {
            fn parse<'a, T: FromStr>(item: &'a str) -> T where <T as FromStr>::Err: Debug { item.parse().unwrap() }
            let mut split = line.split(": ");
            split.next();
            split.next().unwrap().split(", ").map(parse)
        }

        // Get a value from the iterator (repackages Iterator::next into a Result rather than an
        // Option)
        fn getval<T, I: Iterator<Item = T>>(lines: &mut I) -> Result<T, AtlasError> {
            match lines.next() {
                Some(val) => Ok(val),
                None => Err(AtlasError::ParseError("Unexpected end of data"))?
            }
        }

        let mut lines = data.lines();
        let mut images = Vec::new();
        let mut regions = Vec::new();
        let directory: &Path = if let Some(parent) = path.parent() { parent } else { path.as_ref() };
        while let Some(line) = lines.next() {
            //Create a path relative to the atlas location
            let path: PathBuf = [directory, &Path::new(line)].iter().collect();
            images.push(Image::load(path));
            //Skip some lines the loader doesn't use
            for _ in 0..4 {
                getval(&mut lines)?;
            }
            //Parse each region
            while let Some(line) = lines.next() {
                //If there's an empty line, move onto a different page
                if line.len() == 0 { break; }
                let name = line.to_owned();
                let mut rotate = get_values_from_line(getval(&mut lines)?);
                let mut xy = get_values_from_line::<i32>(getval(&mut lines)?);
                let mut size = get_values_from_line(getval(&mut lines)?);
                let mut line = getval(&mut lines)?;
                while !line.contains("orig") {
                    line = getval(&mut lines)?;
                } 
                let mut orig = get_values_from_line::<i32>(line);
                let mut offset = get_values_from_line::<i32>(getval(&mut lines)?);
                let index = getval(&mut get_values_from_line(getval(&mut lines)?))?;
                let rotate = getval(&mut rotate)?;
                let region = Rectangle::new(getval(&mut xy)?, getval(&mut xy)?, getval(&mut size)?, getval(&mut size)?);
                let original_size = Vector::new(getval(&mut orig)?, getval(&mut orig)?);
                let offset = Vector::new(getval(&mut offset)?, getval(&mut offset)?);
                let center = region.center() + (original_size - region.size() - offset.x_comp() + offset.y_comp());
                let image = images.len() - 1;
                regions.push(Region { image, name, region, rotate, center, index });
            }
        }
        Ok((join_all(images), regions))
    }
}


#[derive(Clone)]
/// An individual named item of an Atlas
///
/// If there is only one frame / no index for an Atlas item, it will be an Image, otherwise, it
/// will be an Animation. 
pub enum AtlasItem {
    /// A still image frame
    Image(Image),
    /// A list of image frames in order
    Animation(Vec<Image>)
}

impl AtlasItem {
    /// Unwrap the enum to a still frame, panicking if it is an animation
    pub fn unwrap_image(self) -> Image {
        match self {
            AtlasItem::Image(image) => image,
            AtlasItem::Animation(_) => panic!("called `AtlasItem::unwrap_image` on an Animation")
        }
    }

    /// Unwrap the enum to an animationo, panicking if it is an image
    pub fn unwrap_animation(self) -> Vec<Image> {
        match self {
            AtlasItem::Animation(anim) => anim,
            AtlasItem::Image(_) => panic!("called `AtlasItem::unwrap_animation` on an Image")
        }
    }
}

#[derive(Clone, Debug)]
/// An error generated when trying to load an atlas
pub enum AtlasError {
    /// An error created when loading one of the images
    ImageError(ImageError),
    /// An error created parsing the Atlas manifest
    ParseError(&'static str),
    /// An error created loading the atlas manifest
    IOError(IOError)
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

impl From<IOError> for AtlasError {
    fn from(err: IOError) -> AtlasError {
        AtlasError::IOError(err)
    }
}
