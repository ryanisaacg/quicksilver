use super::*;

use crate::error::FontError;
use elefont::{CacheError, FontCache, FontProvider, PixelType, Texture, TextureGlyph};
use std::iter;
#[cfg(feature = "ttf")]
use std::path::Path;

#[cfg(feature = "ttf")]
/// VectorFonts allow drawing characters from TTF files
///
/// They store the glyphs as a collection of triangles, allowing them to be scaled up or down
/// without losing quality. To draw characters to the screen, use the [`to_renderer`] method to
/// create a [`FontRenderer`].
///
/// [`to_renderer`]: VectorFont::to_renderer
pub struct VectorFont(rusttype::Font<'static>);

#[cfg(feature = "ttf")]
impl VectorFont {
    /// Create a VectorFont from a slice of binary TTF data
    pub fn from_slice(data: &[u8]) -> Self {
        VectorFont(
            rusttype::FontCollection::from_bytes(data.to_vec())
                .unwrap()
                .into_font()
                .unwrap(),
        )
    }

    /// Create a VectorFont from an owned Vec of TTF data
    pub fn from_bytes(data: Vec<u8>) -> Self {
        VectorFont(
            rusttype::FontCollection::from_bytes(data)
                .unwrap()
                .into_font()
                .unwrap(),
        )
    }

    /// Load a VectorFont from a TTF file at the given path
    pub async fn load(path: impl AsRef<Path>) -> crate::Result<Self> {
        let file_contents = platter::load_file(path).await?;
        Ok(Self::from_bytes(file_contents))
    }

    /// Convert a VectorFont to a [`FontRenderer`] for actual use
    pub fn to_renderer(&self, gfx: &Graphics, font_size: f32) -> crate::Result<FontRenderer> {
        let provider = elefont::rusttype_provider::SizedFont::new(self.0.clone(), font_size);
        FontRenderer::from_font(gfx, Box::new(provider))
    }
}

/// A FontRenderer pairs a font source (typically a [`VectorFont`] or bitmap font) and a GPU cache
/// for efficient rendering
///
/// Instead of uploading glyphs to the GPU every time they're drawn, this method allows for future
/// draws to reference these glyphs multiple times.
pub struct FontRenderer(FontCache<FontImage>);

impl FontRenderer {
    /// Create a font from an arbitrary [`FontProvider`]
    ///
    /// If you want to load a TTF file, consider [`VectorFont::load`] and [`VectorFont::to_renderer`]
    /// instead.
    pub fn from_font(gfx: &Graphics, source: Box<dyn FontProvider>) -> crate::Result<Self> {
        let cache = FontCache::new(source, FontImage::new(gfx)?);

        Ok(Self(cache))
    }

    /// Draw some text to the screen with a given color at a given position, returning the text
    /// extents
    ///
    /// This method will not wrap but will respect newlines. To wrap, use
    /// [`FontRenderer::draw_wrapping`]. The returned value is how far the text extended past the
    /// offset, e.g. the furthest right and furthest down position.
    #[inline]
    pub fn draw(
        &mut self,
        gfx: &mut Graphics,
        text: &str,
        color: Color,
        offset: Vector,
    ) -> crate::Result<Vector> {
        self.draw_wrapping(gfx, text, None, color, offset)
    }

    /// Draw some text to the screen with a given color at a given position, returning the text
    /// extents
    ///
    /// If a maximum width is provided, the text will not extend beyond it. If a word encounters
    /// the maximum width, it will be wrapped down to a newline. The returned value is how far the
    /// text extended past the offset, e.g. the furthest right and furthest down position.
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

            extents = extents.max(cursor);
        }

        Ok(extents)
    }
}

/// A glyph that has been laid-out and uploaded to the GPU, making it ready to render
pub struct LayoutGlyph {
    /// What glyph this is, and what region of the image it takes up
    pub glyph: TextureGlyph,
    /// Where the glyph should be drawn, relative to the beginning of its block of text
    pub position: Vector,
    /// The GPU texture where this glyph is stored
    ///
    /// It is not the whole image! Use [`LayoutGlyph::glyph`] to find the region that the glyph
    /// lies in
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
    #[inline]
    fn width(&self) -> u32 {
        self.image.raw().width()
    }

    #[inline]
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
