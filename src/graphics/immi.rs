// TODO: in the next breaking change, this should possibly become its own module
use crate::{
    geom::{Rectangle, Vector, Transform},
    input::{ButtonState, MouseButton},
    graphics::{Background, Font, FontStyle, GpuTriangle, Image, Mesh, Vertex, View},
    lifecycle::Window,
};
use immi::{Draw, DrawContext, GlyphInfos, Matrix};
use rusttype::{Point, Scale};

/// Combine the ImmiStatus and the Render into a DrawContext
///
/// This is the main way to use Immi from within Quicksilver
pub fn create_immi_ctx<'a>(state: ImmiStatus, render: &'a mut ImmiRender<'a>) -> DrawContext<'a, ImmiRender<'a>> {
    immi::draw().draw(state.window_size.x, state.window_size.y, render, state.mouse_pos, state.pressed, state.released)
}

/// The current state of the world to pass to Immi
///
/// Contains information about mouse position, cursor, etc. Used with `create_immi_ctx`
pub struct ImmiStatus {
    window_size: Vector,
    mouse_pos: Option<[f32; 2]>,
    pressed: bool,
    released: bool,
}

impl ImmiStatus {
    /// Get the state from the window
    pub fn new(window: &Window) -> ImmiStatus {
        let window_size = window.screen_size();
        let mouse_pos = window.mouse().pos();
        // Scaled from -1 to 1. (-1 being the left of the window, 1 being the right of the window)
        let mouse_x_normalized = (mouse_pos.x / window_size.x) * 2f32 - 1f32;
        // Scaled from -1 to 1. (-1 being the bottom of the window, 1 being the top of the window)
        let mouse_y_normalized = (mouse_pos.y / window_size.y) * -2f32 + 1f32;
        let state = window.mouse()[MouseButton::Left];
        ImmiStatus {
            window_size,
            mouse_pos: Some([mouse_x_normalized, mouse_y_normalized]),
            pressed: state == ButtonState::Pressed,
            released: state == ButtonState::Released,
        }
    }
}

/// The implementation of the Immi rendering code for Quicksilver
///
/// This allows Immi to draw the UI to an arbitrary mesh. Used with `create_immi_ctx`
pub struct ImmiRender<'a> {
    window: &'a mut Mesh,
    view: View,
    font: &'a Font
}

// TODO: in the next breaking change, remove old 'new' function and only allow creation of ImmiRender through the
// create_immi_ctx or new_with_view/window
impl<'a> ImmiRender<'a> {
    /// Create an instance of the renderer, which should be done every frame
    ///
    /// The renderer is a short-lived object that should not be stored
    #[deprecated(since = "0.3.3", note = "please use new_with_view instead")]
    pub fn new(target: &'a mut Mesh, font: &'a Font) -> ImmiRender<'a> {
        ImmiRender::new_with_view(target, View::new(Rectangle::new((-1, -1), (2, 2))), font)
    }

    /// Create a new instance of the renderer with the given view
    pub fn new_with_view(target: &'a mut Mesh, view: View, font: &'a Font) -> ImmiRender<'a> {
        ImmiRender {
            window: target,
            view,
            font
        }
    }

    /// Create a renderer from a Window
    pub fn new_with_window(window: &'a mut Window, font: &'a Font) -> ImmiRender<'a> {
        let view = window.view();
        ImmiRender::new_with_view(window.mesh(), view, font)
    }
}

fn matrix_to_trans(matrix: &Matrix) -> Transform {
    let array = matrix.0;
    Transform::from_array([
        [array[0][0], array[1][0], array[2][0]],
        [array[0][1], array[1][1], array[2][1]],
        [0.0, 0.0, 1.0],
    ])
}


impl<'a> Draw for ImmiRender<'a> {
    type ImageResource = Image;
    type TextStyle = FontStyle;

    fn draw_triangle(&mut self, texture: &Image, matrix: &Matrix, uv_coords: [[f32; 2]; 3]) {
        let transform = self.view.opengl.inverse() * matrix_to_trans(matrix);
        let offset = self.window.vertices.len() as u32;
        self.window.vertices.extend([(-1, 1), (-1, -1), (1, 1)]
            .iter()
            .map(|(x, y)| transform * Vector::new(*x, *y))
            .zip(uv_coords.iter().map(|[x, y]| Vector::new(*x, 1.0 - *y)))
            .map(|(pos, tex_coord)| Vertex::new(pos, Some(tex_coord), Background::Img(texture))));
        self.window.triangles.push(GpuTriangle::new(offset, [0, 1, 2], 0.0, Background::Img(texture)));
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

