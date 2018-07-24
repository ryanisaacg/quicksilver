use geom::{Vector, Transform};
use graphics::{Color, Font, FontStyle, GpuTriangle, Image, RenderTarget, Vertex};
use immi::{Draw, GlyphInfos, Matrix};
use rusttype::{Point, Scale};

/// A Quicksilver implementation of immi, which allows immediate GUI functionality
pub struct ImmiRender<'a, T: 'a + RenderTarget> {
    window: &'a mut T,
    font: &'a Font
}

impl<'a, T: 'a + RenderTarget> ImmiRender<'a, T> {
    /// Create an instance of the renderer, which should be done every frame
    ///
    /// The renderer is a short-lived object that should not be stored
    pub fn new(target: &'a mut T, font: &'a Font) -> ImmiRender<'a, T> {
        ImmiRender {
            window: target,
            font
        }
    }
}

fn matrix_to_trans(matrix: &Matrix) -> Transform {
    let array = matrix.0;
    Transform::from_array([
        [array[0][0], array[0][1], 0.0],
        [array[1][0], array[1][1], 0.0],
        [array[2][0], array[2][1], 1.0],
    ])
}


impl<'a, T: 'a + RenderTarget> Draw for ImmiRender<'a, T> {
    type ImageResource = Image;
    type TextStyle = FontStyle;

    fn draw_triangle(&mut self, texture: &Image, matrix: &Matrix, uv_coords: [[f32; 2]; 3]) {
        let transform = matrix_to_trans(matrix);
        let vertices = [(-1, 1), (-1, -1), (1, 1)]
            .iter()
            .map(|(x, y)| transform * Vector::new(*x, *y))
            .zip(uv_coords.iter().map(|[x, y]| Vector::new(*x, *y)))
            .map(|(pos, tex_coord)| Vertex::new_textured(pos, tex_coord, Color::WHITE));
        let indices = [ GpuTriangle::new_textured([0, 1, 2], 0.0, texture.clone()) ];
        self.window.add_vertices(vertices, indices.iter().cloned());
    }

    fn get_image_width_per_height(&mut self, image: &Image) -> f32 {
        image.area().width() / image.area().height()
    }

    fn draw_glyph(&mut self, text_style: &FontStyle, glyph: char, matrix: &Matrix) {
        let rendered = self.font.render(glyph.encode_utf8(&mut [0; 4]), text_style)
            .expect("The character must render correctly");
        self.draw_triangle(&rendered, matrix, [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 1.0]
        ]);
        self.draw_triangle(&rendered, matrix, [
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0]
        ]);
    }

    fn line_height(&self, _text_style: &FontStyle) -> f32 { 1.2 }

    fn glyph_infos(&self, text_style: &FontStyle, glyph: char) -> GlyphInfos {
        let mut buffer = [0; 4];
        let string = glyph.encode_utf8(&mut buffer);
        let scale = Scale::uniform(text_style.size);
        let start = Point { x: 0.0, y: 0.0 };
        let layout = self.font.data.layout(string, scale, start)
            .next()
            .expect("One character string in, one layout object out");
        let bounds = layout.pixel_bounding_box()
            .expect("Pixel bounding box must exit");
        let x_offset = bounds.min.y as f32;
        let y_offset = bounds.min.y as f32;
        let width = bounds.max.x  as f32 - x_offset;
        let height = bounds.max.y as f32 - y_offset;
        GlyphInfos {
            x_offset,
            y_offset,
            width,
            height,
            x_advance: x_offset + width
        }
    }

    fn kerning(&self, text_style: &FontStyle, first_char: char, second_char: char) -> f32 {
        let scale = Scale::uniform(text_style.size);
        self.font.data.pair_kerning(scale, first_char, second_char)
    }
}

