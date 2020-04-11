use super::*;

use crate::error::FontError;
use elefont::{CacheError, FontCache, FontProvider, PixelType, Texture, TextureGlyph};
use std::iter;
use std::path::Path;

#[cfg(feature = "ttf")]
pub struct VectorFont(rusttype::Font<'static>);

#[cfg(feature = "ttf")]
impl VectorFont {
    pub fn from_slice(data: &[u8]) -> Self {
        VectorFont(
            rusttype::FontCollection::from_bytes(data.to_vec())
                .unwrap()
                .into_font()
                .unwrap(),
        )
    }

    pub fn from_bytes(data: Vec<u8>) -> Self {
        VectorFont(
            rusttype::FontCollection::from_bytes(data)
                .unwrap()
                .into_font()
                .unwrap(),
        )
    }

    pub async fn load(path: impl AsRef<Path>) -> crate::Result<Self> {
        let file_contents = platter::load_file(path).await?;
        Ok(Self::from_bytes(file_contents))
    }

    pub fn to_renderer(&self, gfx: &Graphics, font_size: f32) -> crate::Result<FontRenderer> {
        let provider = elefont::rusttype_provider::SizedFont::new(self.0.clone(), font_size);
        FontRenderer::from_font(gfx, Box::new(provider))
    }
}

pub struct FontRenderer(FontCache<FontImage>);

impl FontRenderer {
    pub fn from_font(gfx: &Graphics, source: Box<dyn FontProvider>) -> crate::Result<Self> {
        let cache = FontCache::new(source, FontImage::new(gfx)?);

        Ok(Self(cache))
    }

    pub fn draw(
        &mut self,
        gfx: &mut Graphics,
        text: &str,
        color: Color,
        offset: Vector,
    ) -> crate::Result<Vector> {
        self.draw_wrapping(gfx, text, None, color, offset)
    }

    pub fn draw_wrapping(
        &mut self,
        gfx: &mut Graphics,
        text: &str,
        max_width: Option<f32>,
        color: Color,
        offset: Vector,
    ) -> crate::Result<Vector> {
        self.layout_glyphs(gfx, text, max_width, |gfx, layout| {
            let LayoutGlyph {
                position,
                glyph,
                image,
            } = layout;

            let tex_bounds = glyph.bounds;
            let glyph_size = Vector::new(tex_bounds.width as f32, tex_bounds.height as f32);
            let region = Rectangle::new(
                Vector::new(tex_bounds.x as f32, tex_bounds.y as f32),
                glyph_size,
            );
            let location = Rectangle::new(offset + position, glyph_size);
            gfx.draw_subimage_tinted(&image, region, location, color);
        })
    }

    /// Lay out the given text at a given font size, with a given maximum width, returning its
    /// extents
    ///
    /// Each glyph (and the font) is passed into the callback as it is layed out, giving the option
    /// to render right away, examine and move on, etc.
    pub fn layout_glyphs(
        &mut self,
        gfx: &mut Graphics,
        text: &str,
        max_width: Option<f32>,
        mut callback: impl FnMut(&mut Graphics, LayoutGlyph),
    ) -> crate::Result<Vector> {
        let mut cursor = Vector::ZERO;
        let mut extents = Vector::ZERO;
        let space_glyph = self.0.font().single_glyph(' ');
        let space_metrics = self.0.font().metrics(space_glyph);
        let mut glyphs = Vec::new();
        let line_height = self.0.font().line_height();

        for line in text.split('\n') {
            for word in line.split(' ') {
                match self.0.cache_string(word) {
                    Ok(()) => {}
                    Err(CacheError::OutOfSpace) => {
                        // If the cache is out of space, clear it and insert a new page
                        self.0.replace_texture(FontImage::new(&gfx)?);
                    }
                    Err(CacheError::NonRenderableGlyph(g)) => {
                        return Err(FontError::NonRenderableGlyph(g).into());
                    }
                    Err(CacheError::TextureTooSmall) => {
                        return Err(FontError::StringTooLarge.into());
                    }
                }
                // Retrieve the glyphs from the font
                glyphs.extend(
                    self.0.render_string(word).map(|glyph| {
                        glyph.expect("A character failed to be rendered unexpectedly")
                    }),
                );

                // If we're word wrapping, look ahead to the total width of the word. In the case
                // where the word would overflow the line, move down a line
                if let Some(width) = max_width {
                    let mut word_width = 0.0;
                    let mut it = glyphs.iter().peekable();
                    while let Some((metrics, glyph)) = it.next() {
                        word_width += metrics.advance_x;
                        // If there's a next glyph, try kerning
                        if let Some((_, next)) = it.peek() {
                            word_width += self.0.font().kerning(glyph.glyph, next.glyph)
                        }
                    }
                    if cursor.x + word_width > width {
                        cursor.x = 0.0;
                        cursor.y += line_height;
                    }
                }

                // Send each glyph to the callback
                let mut it = glyphs.drain(..).peekable();
                while let Some((metrics, glyph)) = it.next() {
                    let glyph_position = metrics
                        .bounds
                        .map_or(Vector::ZERO, |b| Vector::new(b.x as f32, b.y as f32));

                    callback(
                        gfx,
                        LayoutGlyph {
                            position: cursor + glyph_position,
                            glyph,
                            image: self.0.texture().image.clone(),
                        },
                    );

                    let bounds = Vector::new(glyph.bounds.width as f32, glyph.bounds.height as f32);
                    extents = extents.max(cursor + glyph_position + bounds);

                    cursor.x += metrics.advance_x;
                    // If there's a next glyph, try kerning
                    if let Some((_, next)) = it.peek() {
                        cursor.x += self.0.font().kerning(glyph.glyph, next.glyph)
                    }
                }
                cursor.x += space_metrics.advance_x;
            }
            cursor.x = 0.0;
            cursor.y += line_height;
        }

        Ok(extents)
    }
}

pub struct LayoutGlyph {
    pub position: Vector,
    pub glyph: TextureGlyph,
    pub image: Image,
}

struct FontImage {
    pub image: Image,
    pub buffer: Vec<u8>,
}

const CACHE_SIZE: usize = 2048;
const CACHE_DIM: u32 = CACHE_SIZE as u32;
static CACHE_DATA: [u8; CACHE_SIZE * CACHE_SIZE * 4] = [0u8; CACHE_SIZE * CACHE_SIZE * 4];

impl FontImage {
    fn new(gfx: &Graphics) -> crate::Result<Self> {
        let image = Image::from_raw(
            gfx,
            Some(&CACHE_DATA[..]),
            CACHE_DIM,
            CACHE_DIM,
            PixelFormat::RGBA,
        )?;
        Ok(FontImage {
            image,
            buffer: Vec::new(),
        })
    }
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
                for (i, alpha) in data.iter().copied().enumerate() {
                    self.buffer[i * 4 + 3] = alpha;
                }
            }
            PixelType::RGBA => {
                self.buffer.extend_from_slice(data);
            }
        }
        let bounds = gpu.bounds;
        self.image.set_sub_data(
            &self.buffer[..],
            bounds.x as u32,
            bounds.y as u32,
            bounds.width,
            bounds.height,
            ColorFormat::RGBA,
        );
    }
}
