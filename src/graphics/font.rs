use super::*;

use std::iter;
use std::path::Path;
use elefont::{FontCache, FontProvider, PixelType, Texture, TextureGlyph};

// TODO: size?
const CACHE_SIZE: usize = 2048;
const CACHE_DIM: u32 = CACHE_SIZE as u32;
static CACHE_DATA: [u8; CACHE_SIZE * CACHE_SIZE * 4] = [0u8; CACHE_SIZE * CACHE_SIZE * 4];

pub struct Font(FontCache<FontImage>);

impl Font {
    pub fn from_font_source(gfx: &Graphics, source: Box<dyn FontProvider>) -> crate::Result<Self> {
        let image = Image::from_raw(gfx, Some(&CACHE_DATA[..]), CACHE_DIM, CACHE_DIM, PixelFormat::RGBA)?;
        let backing_texture = FontImage {
            image,
            buffer: Vec::new(),
        };
        let cache = FontCache::new(source, backing_texture);

        Ok(Font(cache))
    }

    #[cfg(feature="ttf")]
    pub fn from_ttf_slice(gfx: &Graphics, data: &'static [u8]) -> crate::Result<Self> {
        use rusttype::FontCollection;
        let font = FontCollection::from_bytes(data).unwrap().into_font().unwrap();
        Self::from_font_source(gfx, Box::new(font))
    }
    
    #[cfg(feature="ttf")]
    pub fn from_ttf_bytes(gfx: &Graphics, data: Vec<u8>) -> crate::Result<Self> {
        use rusttype::FontCollection;
        let font = FontCollection::from_bytes(data).unwrap().into_font().unwrap();
        Self::from_font_source(gfx, Box::new(font))
    }

    #[cfg(feature="ttf")]
    pub async fn load_ttf(gfx: &Graphics, path: impl AsRef<Path>) -> crate::Result<Self> {
        let file_contents = platter::load_file(path).await?;
        Font::from_ttf_bytes(gfx, file_contents)
    }

    pub(crate) fn cache(&mut self) -> &mut FontCache<FontImage> {
        &mut self.0
    }

    /// Lay out the given text at a given font size, with a given maximum width
    ///
    /// Each glyph (and the font) is passed into the callback as it is layed out, giving the option
    /// to render right away, examine and move on, etc.
    pub fn layout_glyphs(&mut self, text: &str, size: f32, max_width: Option<f32>, mut callback: impl FnMut(&mut Font, LayoutGlyph)) {
        let mut cursor = Vector::ZERO;
        let space_glyph = self.0.font().single_glyph(' ');
        let space_metrics = self.0.font().metrics(elefont::GlyphKey::new(space_glyph, size));
        let mut glyphs = VecDeque::new();
        // TODO: handle max width
        for line in text.split('\n') {
            for word in line.split(' ') {
                glyphs.extend(self.0.render_string(word, size));
                while let Some(glyph) = glyphs.pop_front() {
                    let (metrics, glyph) = glyph.expect("TODO: Failed to fit character in cache");
                    let glyph_position = metrics.bounds.map_or(Vector::ZERO, |b| Vector::new(b.x as f32, b.y as f32));

                    callback(self, LayoutGlyph {
                        position: cursor + glyph_position,
                        glyph,
                    });

                    cursor.x += metrics.advance_x;
                    // If there's a next glyph, try kerning
                    if let Some(Ok((_, next))) = glyphs.front() {
                        if let Some(kerning) = self.0.font().kerning(glyph.key.glyph, next.key.glyph, size) {
                            cursor.x += kerning;
                        }
                    }
                }
                cursor.x += space_metrics.advance_x;
            }
            cursor.x = 0.0;
            cursor.y += self.0.font().line_height(size);
        }
    }

    /// Find the extents of the text layed out with the given parameters
    ///
    /// Retrieves the furthest right extend and furthest bottom extend of the text layout
    pub fn text_extents(&mut self, text: &str, size: f32, max_width: Option<f32>) -> Vector {
        let mut extents = Vector::ZERO;
        self.layout_glyphs(text, size, max_width, |_, LayoutGlyph { position, glyph }| {
            let right = position.x + glyph.bounds.width as f32;
            let bottom= position.y + glyph.bounds.height as f32;
            extents.x = extents.x.max(right);
            extents.y = extents.y.max(bottom);
        });

        extents
    }
}

pub struct LayoutGlyph {
    pub position: Vector,
    pub glyph: TextureGlyph,
}

pub(crate) struct FontImage {
    pub image: Image,
    pub buffer: Vec<u8>,
}

impl Texture for FontImage {
    fn width(&self) -> u32 {
        self.image.raw().width()
    }

    fn height(&self) -> u32 {
        self.image.raw().height()
    }

    /// Write the data from a font into a texture
    fn put_rect(&mut self, pixel: PixelType, data: &[u8], gpu: &TextureGlyph) {
        self.buffer.clear();
        match pixel {
            PixelType::Alpha => {
                self.buffer.extend(iter::repeat(255).take(data.len() * 4));
                for i in 0..data.len() {
                    self.buffer[i * 4 + 3] = data[i];
                }
            }
            PixelType::RGBA => {
                self.buffer.extend_from_slice(data);
            }
        }
        let bounds = gpu.bounds;
        self.image.set_sub_data(&self.buffer[..], bounds.x as u32, bounds.y as u32, bounds.width, bounds.height, ColorFormat::RGBA);
    }
}


/*use crate::{
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
}*/
