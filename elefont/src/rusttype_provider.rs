use super::*;

use rusttype::{Font, GlyphId, Point, Scale};

#[derive(Clone)]
pub struct SizedFont<'a> {
    font: Font<'a>,
    size: f32,
}

impl<'a> SizedFont<'a> {
    pub fn new(font: Font<'a>, size: f32) -> Self {
        SizedFont { font, size }
    }

    pub fn with_size(&self, size: f32) -> Self {
        SizedFont {
            size,
            ..self.clone()
        }
    }

    pub fn font(&self) -> &Font<'a> {
        &self.font
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

impl FontProvider for SizedFont<'_> {
    fn line_height(&self) -> f32 {
        let metrics = self.font.v_metrics(Scale {
            x: self.size,
            y: self.size,
        });

        metrics.ascent - metrics.descent + metrics.line_gap
    }

    fn pixel_type(&self) -> PixelType {
        PixelType::Alpha
    }

    fn single_glyph(&self, c: char) -> Glyph {
        Glyph(self.font.glyph(c).id().0.into())
    }

    fn glyphs(&self, string: &str, glyphs: &mut Vec<Glyph>) {
        glyphs.extend(
            self.font
                .glyphs_for(string.chars())
                .map(|g| Glyph(g.id().0.into())),
        );
    }

    fn metrics(&self, glyph: Glyph) -> Metrics {
        let glyph = scaled_glyph(&self.font, glyph, self.size);
        let h_metrics = glyph.h_metrics();
        let bounds = glyph
            .positioned(Point { x: 0.0, y: 0.0 })
            .pixel_bounding_box()
            .map(|shape| Bounds {
                x: shape.min.x,
                y: shape.min.y,
                width: shape.width() as u32,
                height: shape.height() as u32,
            });

        Metrics {
            bounds,
            bearing_x: h_metrics.left_side_bearing,
            advance_x: h_metrics.advance_width,
            bearing_y: 0.0,
            advance_y: 0.0,
        }
    }

    fn rasterize(&self, glyph: Glyph) -> Result<Vec<u8>, CacheError> {
        let scaled_glyph =
            scaled_glyph(&self.font, glyph, self.size).positioned(Point { x: 0.0, y: 0.0 });
        let bounds = scaled_glyph
            .pixel_bounding_box()
            .ok_or(CacheError::NonRenderableGlyph(glyph))?;
        let mut buffer = alloc::vec![0u8; (bounds.width() * bounds.height()) as usize];
        let width = bounds.width() as u32;
        scaled_glyph.draw(|x, y, val| buffer[(x + y * width) as usize] = (val * 255.0) as u8);

        Ok(buffer)
    }

    fn kerning(&self, a: Glyph, b: Glyph) -> f32 {
        self.font.pair_kerning(
            Scale {
                x: self.size,
                y: self.size,
            },
            GlyphId(a.0 as u16),
            GlyphId(b.0 as u16),
        )
    }
}

fn scaled_glyph<'a>(font: &'a Font, glyph: Glyph, size: f32) -> rusttype::ScaledGlyph<'a> {
    let id = GlyphId(glyph.0 as u16);
    let glyph = font.glyph(id);
    glyph.scaled(Scale { x: size, y: size })
}
