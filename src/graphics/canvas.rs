#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use geom::{Circle, Line, Rectangle, Shape, Transform, Vector};
#[cfg(not(target_arch="wasm32"))]
use glutin::{GlContext};
use graphics::{Backend, BlendMode, Color, Image, Surface, Vertex, View, Window};

///The way to draw items to the screen, produced by WindowBuilder
pub struct Canvas {
    pub(crate) backend: Backend,
    pub(crate) view: View,
}

const CIRCLE_POINTS: usize = 32; //the number of points in the poly to simulate the circle

impl Canvas {
    ///Set the camera view for the Canvas
    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    ///Get the camera view for the Canvas
    pub fn view(&self) -> View {
        self.view
    }

    fn camera(&self) -> Transform {
        self.view.opengl
    }

    ///Clear the screen to a given color
    pub fn clear(&mut self, color: Color) {
        self.backend.clear(color);
    }

    ///Draw the changes made to the screen
    pub fn present(&mut self, _window: &Window) {
        self.backend.flush();
        #[cfg(not(target_arch="wasm32"))]
        _window.gl_window.swap_buffers().unwrap();
    }
    
    ///Draw an image with a given center
    pub fn draw_image(&mut self, image: &Image, center: Vector) {
        self.draw_image_blend(image, center, Color::white());
    }

    ///Draw an image with a given center blended with a given color
    pub fn draw_image_blend(&mut self, image: &Image, center: Vector, col: Color) {
        self.draw_image_trans(image, center, col, Transform::identity());
    }

    ///Draw a transform image blended with a given color
    pub fn draw_image_trans(&mut self, image: &Image, center: Vector, col: Color, trans: Transform) {
        let area = image.area().with_center(center);
        let trans = self.camera() 
            * Transform::translate(area.top_left()) 
            * Transform::translate(area.size() / 2) 
            * trans 
            * Transform::translate(-area.size() / 2)
            * Transform::scale(area.size());
        let recip_size = image.source_size().recip();
        let normalized_pos = image.area().top_left().times(recip_size);
        let normalized_size = image.area().size().times(recip_size);
        let get_vertex = |v: Vector| {
            Vertex {
                pos: trans * v,
                tex_pos: normalized_pos + v.times(normalized_size),
                col: col,
                use_texture: true,
            }
        };
        self.backend.add(
            image.get_id(),
            &[
                get_vertex(Vector::zero()),
                get_vertex(Vector::zero() + Vector::x()),
                get_vertex(Vector::zero() + Vector::one()),
                get_vertex(Vector::zero() + Vector::y()),
            ],
            &[0, 1, 2, 2, 3, 0],
        );
    }

    ///Draw a rectangle filled with a given color
    pub fn draw_rect(&mut self, rect: Rectangle, col: Color) {
        self.draw_rect_trans(rect, col, Transform::identity());
    }

    ///Draw a rectangle filled with a given color with a given transform
    pub fn draw_rect_trans(&mut self, rect: Rectangle, col: Color, trans: Transform) {
        self.draw_polygon_trans(&[Vector::zero(), rect.size().x_comp(), rect.size(), rect.size().y_comp()], col, Transform::translate(rect.top_left()) * trans);
    }

    ///Draw a circled filled with a given color
    pub fn draw_circle(&mut self, circ: Circle, col: Color) {
        self.draw_circle_trans(circ, col, Transform::identity());
    }

    ///Draw a circle filled with a given color with a given transformation
    pub fn draw_circle_trans(&mut self, circ: Circle, col: Color, trans: Transform) {
        let mut points = [Vector::zero(); CIRCLE_POINTS];
        let rotation = Transform::rotate(360f32 / CIRCLE_POINTS as f32);
        let mut arrow = Vector::new(0f32, -circ.radius);
        for i in 0..CIRCLE_POINTS {
            points[i] = arrow;
            arrow = rotation * arrow;
        }
        self.draw_polygon_trans(&points, col, Transform::translate(circ.center()) * trans);
    }

    ///Draw a polygon filled with a given color
    pub fn draw_polygon(&mut self, vertices: &[Vector], col: Color) {
        self.draw_polygon_trans(vertices, col, Transform::identity());
    }

    ///Draw a polygon filled with a given color with a given transform
    pub fn draw_polygon_trans(&mut self, vertices: &[Vector], col: Color, trans: Transform) {
        let first_index = self.backend.num_vertices() as u32;
        let trans = self.camera() * trans;
        for vertex in vertices {
            self.backend.add_vertex(&Vertex {
                pos: trans * vertex.clone(),
                tex_pos: Vector::zero(),
                col: col,
                use_texture: false
            });
        }
        let mut current = 1;
        let mut i = 0;
        let indices = (vertices.len() - 2) * 3;
        while i < indices {
            self.backend.add_index(first_index);
            self.backend.add_index(first_index + current);
            self.backend.add_index(first_index + current + 1);
            current += 1;
            i += 3;
        }
    }

    ///Draw a line with a given color
    pub fn draw_line(&mut self, line: Line, col: Color) {
        self.draw_line_trans(line, col, Transform::identity());
    }

    ///Draw a transformed line
    pub fn draw_line_trans(&mut self, line: Line, col: Color, trans: Transform) {
        let start = Vector::zero();
        let end = line.end - line.start;
        if (end - start).normalize() == Vector::x() {
            self.draw_polygon_trans(&[start, start + Vector::y(), end + Vector::y(), end], col, 
                                Transform::translate(line.start) * trans);
        } else {
            self.draw_polygon_trans(&[start, start + Vector::x(), end + Vector::y(), end], col, 
                                    Transform::translate(line.start) * trans);
        }
    }

    ///Draw a point with a color
    pub fn draw_point(&mut self, vec: Vector, col: Color) {
        self.draw_polygon_trans(&[vec, vec + Vector::x(), vec + Vector::one(), vec + Vector::y()], 
                                col, Transform::identity());
    }

    ///Draw a shape filled with a given color
    pub fn draw_shape(&mut self, shape: Shape, col: Color) {
        self.draw_shape_trans(shape, col, Transform::identity());
    }

    ///Draw a translated shape filled with a given color
    pub fn draw_shape_trans(&mut self, shape: Shape, col: Color, trans: Transform) {
        match shape {
            Shape::Rect(r) => self.draw_rect_trans(r, col, trans),
            Shape::Circ(c) => self.draw_circle_trans(c, col, trans),
            Shape::Line(l) => self.draw_line_trans(l, col, trans),
            Shape::Vect(v) => self.draw_point(trans * v, col)
        }
    }

    ///Draw a surface with a given blend mode
    pub fn draw_surface(&mut self, surface: &Surface, position: Vector, blend: BlendMode) {
        self.backend.set_blend_mode(blend);
        self.draw_image(surface.image(), position);
        self.backend.reset_blend_mode();
    }
}

