extern crate glutin;

use geom::{Circle, Line, Rectangle, Shape, Transform, Vector};
use graphics::{Backend, Camera, Color, Colors, TextureRegion, Vertex};

pub struct Graphics {
    backend: Box<Backend>,
    cam: Camera,
    clear_color: Color,
    show_cursor: bool
}

const CIRCLE_POINTS: usize = 32; //the number of points in the poly to simulate the circle

impl Graphics {
    pub fn new(backend: Box<Backend>, cam: Camera) -> Graphics {
        Graphics {
            backend,
            cam,
            clear_color: Colors::BLACK,
            show_cursor: true
        }
    }

    pub fn set_camera(&mut self, cam: Camera) {
        self.cam = cam;
    }

    pub fn set_show_cursor(&mut self, show_cursor: bool) {
        self.show_cursor = show_cursor;
    }

    fn camera(&self) -> Transform {
        self.cam.transform()
    }

    pub fn clear_color(&self) -> Color {
        self.clear_color
    }

    pub fn set_clear_color(&mut self, col: Color) {
        self.clear_color = col;
    }

    pub fn present(&mut self, window: &glutin::GlWindow) {
        window.set_cursor_state(if self.show_cursor { glutin::CursorState::Normal } else { glutin::CursorState::Hide }).unwrap();
        self.backend.display();
        glutin::GlContext::swap_buffers(window).unwrap();
        self.backend.clear(self.clear_color);
    }

    pub fn draw_image(&mut self, image: TextureRegion, area: Rectangle) {
        self.draw_image_blend(image, area, Colors::WHITE);
    }

    pub fn draw_image_blend(&mut self, image: TextureRegion, area: Rectangle, col: Color) {
        self.draw_image_trans(image, area, col, Transform::identity());
    }

    pub fn draw_image_trans(&mut self, image: TextureRegion, area: Rectangle, col: Color, trans: Transform) {
        let trans = self.camera() 
            * Transform::translate(area.top_left()) 
            * Transform::translate(area.size() / 2) 
            * trans 
            * Transform::translate(-area.size() / 2)
            * Transform::scale(area.size());
        let recip_size = image.source_size().recip();
        let normalized_pos = image.get_region().top_left().times(recip_size);
        let normalized_size = image.get_region().size().times(recip_size);
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

    pub fn draw_rect(&mut self, rect: Rectangle, col: Color) {
        self.draw_rect_trans(rect, col, Transform::identity());
    }

    pub fn draw_rect_trans(&mut self, rect: Rectangle, col: Color, trans: Transform) {
        self.draw_polygon_trans(&[Vector::zero(), rect.size().x_comp(), rect.size(), rect.size().y_comp()], col, Transform::translate(rect.top_left()) * trans);
    }

    pub fn draw_circle(&mut self, circ: Circle, col: Color) {
        self.draw_circle_trans(circ, col, Transform::identity());
    }

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

    pub fn draw_polygon(&mut self, vertices: &[Vector], col: Color) {
        self.draw_polygon_trans(vertices, col, Transform::identity());
    }

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

    pub fn draw_line(&mut self, line: Line, col: Color) {
        self.draw_line_trans(line, col, Transform::identity());
    }

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

    pub fn draw_point(&mut self, vec: Vector, col: Color) {
        self.draw_polygon_trans(&[vec, vec + Vector::x(), vec + Vector::one(), vec + Vector::y()], 
                                col, Transform::identity());
    }

    pub fn draw_shape(&mut self, shape: Shape, col: Color) {
        self.draw_shape_trans(shape, col, Transform::identity());
    }

    pub fn draw_shape_trans(&mut self, shape: Shape, col: Color, trans: Transform) {
        match shape {
            Shape::Rect(r) => self.draw_rect_trans(r, col, trans),
            Shape::Circ(c) => self.draw_circle_trans(c, col, trans),
            Shape::Line(l) => self.draw_line_trans(l, col, trans),
            Shape::Vect(v) => self.draw_point(trans * v, col)
        }
    }
}
