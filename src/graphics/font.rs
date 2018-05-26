extern crate futures;
extern crate rusttype;

use graphics::{Color, Image, PixelFormat};
use error::QuicksilverError;
use FileLoader;

use futures::{Async, Future, Map, Poll};
use rusttype::{Font as RTFont, FontCollection, PositionedGlyph, Scale, point};
use std::path::Path;

/// An in-memory TTF font that can render text on demand
pub struct Font {
    data: RTFont<'static>
}

type LoadFunction = fn(Vec<u8>) -> Result<Font, QuicksilverError>;
/// A future to load a font
pub struct FontLoader(Map<FileLoader, LoadFunction>);

impl Future for FontLoader {
    type Item = Font;
    type Error = QuicksilverError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(match self.0.poll()? {
            Async::Ready(data) => Async::Ready(data?),
            Async::NotReady => Async::NotReady
        })
    }
}

impl Font {
    /// Load a font at a given file
    pub fn load<P: AsRef<Path>>(path: P) -> FontLoader {
        FontLoader(FileLoader::load(path)
                   .map(parse as LoadFunction))
    }

    /// Render a text string to an Image
    ///
    /// This function does not take into account unicode normalization or vertical layout
    pub fn render(&self, text: &str, size: f32, color: Color) -> Image {
        let scale = Scale { x: size, y: size };
        //Avoid clipping
        let offset = point(0.0, self.data.v_metrics(scale).ascent);
        let glyphs = self.data.layout(text, scale, offset).collect::<Vec<PositionedGlyph>>();
        let width = glyphs.iter().rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next().unwrap_or(0.0).ceil() as usize;
        let mut pixels = vec![0 as u8; 4 * width * size as usize];
        for glyph in glyphs {
            if let Some(bounds) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x + bounds.min.x as u32;
                    let y = y + bounds.min.y as u32;
                    //let height = size as u32;
                    let index = (4 * (x + y * width as u32)) as usize;
                    let bytes = [(255.0 * color.r) as u8, (255.0 * color.g) as u8, (255.0 * color.b) as u8, (255.0 * v) as u8];
                    for i in 0..bytes.len() {
                        pixels[index + i] = bytes[i];
                    }
                });
            }
        }
        Image::from_raw(pixels.as_slice(), width as u32, size as u32, PixelFormat::RGBA)
    }
}

fn parse(data: Vec<u8>) -> Result<Font, QuicksilverError> {
    if let Some(data) = FontCollection::from_bytes(data).into_font() {
        Ok(Font { data })
    } else {
        Err(QuicksilverError::InvalidFont)
    }
}
