use crate::{
    load_file,
    Result,
    error::QuicksilverError,
    graphics::{Color, Image, PixelFormat},
};
use futures::{Future, future};
use rusttype::{Font as RTFont, FontCollection, PositionedGlyph, Scale, point};
use std::path::Path;

/// An in-memory TTF font that can render text on demand
pub struct Font {
    pub(crate) data: RTFont<'static>
}

impl Font {
    /// Load a font at a given file
    pub fn load(path: impl AsRef<Path>) -> impl Future<Item = Font, Error = QuicksilverError> {
        load_file(path)
            .map(Font::from_bytes)
            .and_then(future::result)
    }

    /// Creates font from bytes sequence.
    pub fn from_slice(data: &'static [u8]) -> Result<Self> {
        Ok(Font {
            data: FontCollection::from_bytes(data)?.into_font()?
        })
    }

    /// Creates font from owned bytes sequence.
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        Ok(Font {
            data: FontCollection::from_bytes(data)?.into_font()?
        })
    }

    /// Render a text string to an Image
    ///
    /// This function handles line breaks but it does not take into account unicode
    /// normalization or other text formatting.
    pub fn render(&self, text: &str, style: &FontStyle) -> Result<Image> {
        let scale = Scale { x: style.size, y: style.size };
        let line_count = text.lines().count();
        let glyphs_per_line = text
            .lines()
            .map(|text| {
                //Avoid clipping
                let offset = point(0.0, self.data.v_metrics(scale).ascent);
                let glyphs = self.data.layout(text.trim_end(), scale, offset)
                    .collect::<Vec<PositionedGlyph>>();
                let width = glyphs.iter().rev()
                    .map(|g|
                        g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
                    .next().unwrap_or(0.0).ceil() as usize;
                (glyphs, width)
            })
            .collect::<Vec<_>>();
        let max_width = *glyphs_per_line.iter().map(|(_, width)| width).max().unwrap_or(&0);
        let mut pixels = vec![0 as u8; 4 * line_count * max_width * style.size as usize];
        for (line_index, (glyphs, width)) in glyphs_per_line.iter().enumerate() {
            let width = *width;
            let line_offset = line_index * 4 * max_width * style.size as usize;
            for glyph in glyphs {
                if let Some(bounds) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        // `bounds.min` can contain negative numbers:
                        let bound_min_x = std::cmp::max(0, bounds.min.x) as u32;
                        let bound_min_y = std::cmp::max(0, bounds.min.y) as u32;
                        let x = x + bound_min_x;
                        let y = y + bound_min_y;
                        // x or y can be greater than our pixels area:
                        if x < width as u32 && y < style.size as u32 {
                            let index = line_offset + (4 * (x + y * max_width as u32)) as usize;
                            let red = (255.0 * style.color.r) as u8;
                            let green = (255.0 * style.color.g) as u8;
                            let blue = (255.0 * style.color.b) as u8;
                            let alpha = (255.0 * v) as u8;
                            let bytes = [red, green, blue, alpha];
                            for i in 0..bytes.len() {
                                pixels[index + i] = bytes[i];
                            }
                        }
                    });
                }
            }
        }
        Image::from_raw(pixels.as_slice(), max_width as u32,
                        line_count as u32 * style.size as u32, PixelFormat::RGBA)
    }
}

/// The way text should appear on the screen
#[derive(Clone, Copy, Debug)]
pub struct FontStyle {
    pub(crate) size: f32,
    pub(crate) color: Color
}

impl FontStyle {
    /// Create a new instantce of a font style
    pub fn new(size: f32, color: Color) -> FontStyle {
        FontStyle {
            size,
            color
        }
    }
}
