use super::*;

use alloc::sync::Arc;
use fontdue::Font;

#[derive(Clone)]
pub struct SizedFont {
    font: Arc<Font>,
    size: f32,
}

impl SizedFont {
    pub fn new(font: Font, size: f32) -> Self {
        let font = Arc::new(font);
        SizedFont { font, size }
    }

    pub fn with_size(&self, size: f32) -> Self {
        SizedFont {
            font: self.font.clone(),
            size,
        }
    }

    pub fn font(&self) -> &Font {
        &self.font
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

impl FontProvider for SizedFont {
    fn line_height(&self) -> f32 {
        match self.font.vertical_line_metrics(self.size) {
            Some(m) => m.new_line_size,
            None => 0.0,
        }
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Alpha
    }

    fn single_glyph(&self, c: char) -> Glyph {
        // Note: fontdue uses a u16 internally
        Glyph(self.font.lookup_glyph_index(c) as u32)
    }

    fn glyphs(&self, string: &str, glyphs: &mut Vec<Glyph>) {
        glyphs.extend(string.chars().map(|c| self.single_glyph(c)));
    }

    fn metrics(&self, glyph: Glyph) -> Metrics {
        let metrics = self.font.metrics_indexed(glyph.0 as usize, self.size);
        let aabb = metrics.bounds;
        let bounds = Bounds {
            // TODO: examine truncation vs rounding
            x: aabb.xmin as i32,
            y: aabb.ymin as i32,
            width: metrics.width as u32,
            height: metrics.height as u32,
        };

        Metrics {
            bounds: Some(bounds),
            bearing_x: aabb.xmin + aabb.width,
            advance_x: metrics.advance_width,
            bearing_y: aabb.ymin + aabb.height,
            advance_y: metrics.advance_height,
        }
    }

    fn rasterize(&self, glyph: Glyph) -> Result<Vec<u8>, CacheError> {
        let (_, buffer) = self.font.rasterize_indexed(glyph.0 as usize, self.size);

        Ok(buffer)
    }

    fn kerning(&self, _a: Glyph, _b: Glyph) -> f32 {
        // Does not appear to be implemented in fontdue yet
        0.0
    }
}
