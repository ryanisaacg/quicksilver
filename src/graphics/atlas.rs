use {Result, load_file};
use error::QuicksilverError;
use futures::{Future, future};
use geom::{Positioned, Rectangle, Vector};
use graphics::{Image, ImageError};
use std::{
    cmp::Ordering,
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    io::Error as IOError,
    num::ParseIntError,
    path::{Path, PathBuf},
    str::{FromStr, ParseBoolError}
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

#[derive(Clone, Debug)]
/// An image atlas that allows a single image file to represent multiple individual images
///
/// It uses the libgdx / spine atlas format and groups them by name and index if applicable 
pub struct Atlas {
    data: HashMap<String, AtlasItem>
}

impl Atlas {
    /// Load an atlas at a given path
    pub fn load<'a, P: 'static + AsRef<Path>>(path: P) -> impl Future<Item = Atlas, Error = QuicksilverError> {
        load_file(PathBuf::from(path.as_ref()))
            .map(move |data| {
                let path = path.as_ref();

                let data = match String::from_utf8(data) {
                    Ok(string) => string,
                    Err(_) => return Err(AtlasError::ParseError("Failed to parse provided file as UTF8 text").into())
                };

                let mut lines = data.lines();
                let mut images = Vec::new();
                let mut regions = Vec::new();
                let directory: &Path = if let Some(parent) = path.parent() { parent } else { path.as_ref() };
                while let Some(line) = lines.next() {
                    use std::path::PathBuf;
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
                        let mut rotate = get_values_from_line(getval(&mut lines)?)?;
                        let mut xy = get_values_from_line::<i32>(getval(&mut lines)?)?;
                        let mut size = get_values_from_line(getval(&mut lines)?)?;
                        let mut line = getval(&mut lines)?;
                        while !line.contains("orig") {
                            line = getval(&mut lines)?;
                        } 
                        let mut orig = get_values_from_line::<i32>(line)?;
                        let mut offset = get_values_from_line::<i32>(getval(&mut lines)?)?;
                        let index = getval(&mut get_values_from_line(getval(&mut lines)?)?)??;
                        let rotate = getval(&mut rotate)??;
                        let region = Rectangle::new(getval(&mut xy)??, getval(&mut xy)??, getval(&mut size)??, getval(&mut size)??);
                        let original_size = Vector::new(getval(&mut orig)??, getval(&mut orig)??);
                        let offset = Vector::new(getval(&mut offset)??, getval(&mut offset)??);
                        let center = region.center() + (original_size - region.size() - offset.x_comp() + offset.y_comp());
                        let image = images.len() - 1;
                        regions.push(Region { image, name, region, rotate, center, index });
                    }
                }
                Ok((future::join_all(images), regions))
            })
            .and_then(future::result)
            .and_then(|(image_loader, regions)| image_loader.map(|images| (images, regions)))
            .map(create)
    }

    /// Get an image or animation with a given name
    pub fn get(&self, name: &str) -> Option<AtlasItem> {
        Some(self.data.get(name)?.clone())
    }
}

// Split a line into its right-hand values
fn get_values_from_line<'a, T: FromStr>(line: &'a str) -> Result<impl 'a + Iterator<Item = Result<T>>> {
    let mut split = line.split(": ");
    // Discard the name label
    getval(&mut split)?;
    Ok(getval(&mut split)?.split(", ").map(|item| match item.parse() {
        Ok(item) => Ok(item),
        Err(_) => Err(AtlasError::ParseError("Failed to parse item in manifest").into())
    }))
}

// Get a value from the iterator (repackages Iterator::next into a Result rather than an
// Option)
fn getval<T>(lines: &mut impl Iterator<Item = T>) -> Result<T> {
    match lines.next() {
        Some(val) => Ok(val),
        None => Err(AtlasError::ParseError("Unexpected end of data").into())
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



#[derive(Clone, Debug)]
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

#[derive(Debug)]
/// An error generated when trying to load an atlas
pub enum AtlasError {
    /// An error created when loading one of the images
    ImageError(ImageError),
    /// An error created parsing the Atlas manifest
    ParseError(&'static str),
    /// An error created loading the atlas manifest
    IOError(IOError)
}

impl Display for AtlasError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.description())
    }
}

impl Error for AtlasError {
    fn description(&self) -> &str {
        match self {
            &AtlasError::ImageError(ref err) => err.description(),
            &AtlasError::ParseError(string) => string,
            &AtlasError::IOError(ref err) => err.description()
        }
    }
    
    fn cause(&self) -> Option<&Error> {
        match self {
            &AtlasError::ImageError(ref err) => Some(err),
            &AtlasError::ParseError(_) => None,
            &AtlasError::IOError(ref err) => Some(err)
        }
    }
}


#[doc(hidden)]
impl From<ImageError> for AtlasError {
    fn from(err: ImageError) -> AtlasError {
        AtlasError::ImageError(err)
    }
}

#[doc(hidden)]
impl From<ParseIntError> for AtlasError {
    fn from(_: ParseIntError) -> AtlasError {
        AtlasError::ParseError("Failed to parse an int")
    }
}

#[doc(hidden)]
impl From<ParseBoolError> for AtlasError {
    fn from(_: ParseBoolError) -> AtlasError {
        AtlasError::ParseError("Failed to parse an bool")
    }
}

#[doc(hidden)]
impl From<IOError> for AtlasError {
    fn from(err: IOError) -> AtlasError {
        AtlasError::IOError(err)
    }
}
