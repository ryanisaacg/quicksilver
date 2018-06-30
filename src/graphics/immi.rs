extern crate immi;

use graphics::{Font, FontStyle, Image, Window};
use immi::{Draw, Matrix};

struct ImmiRender<'a> {
    window: &'a mut Window,
    font: &'a Font
}

impl<'a> ImmiRender<'a> {
    fn new(window: &'a mut Window, font: &'a Font) -> ImmiRender<'a> {
        ImmiRender {
            window,
            font
        }
    }
}

impl<'a> Draw for ImmiRender<'a> {
    type ImageResource = Image;
    type TextStyle = FontSTyle;

    fn draw_triangle(&mut self, texture: &Image, matrix: &Matrix, uv_coords: [[f32; 2]; 3]) {

    }

    fn get_image_width_per_height(&mut self, name: &Image) -> f32 {
        image.area().width / image.area().height
    }

    fn draw_glyph(&mut self, text_style: &Self::TextStyle, glyph: char, matrix: &Matrix) {

    }

    fn line_height(&self, text_style: &Self::TextStyle) -> f32 {

    }

    fn glyph_infos(&self, text_style: &Self::TextStyle, glyph: char) -> GlyphInfos {

    }

    fn kerning(&self, text_style: &Self::TextStyle, first_char: char, second_char: char) -> f32 {
        self.font.data.pair_kerning
    }
}

