extern crate glutin;

use geom::{Circle, Line, Rectangle, Shape, Transform, Vector};
use graphics::{Backend, Camera, Color, Colors, TextureRegion, Vertex};

pub struct Graphics {
    backend: Box<Backend>,
    cam: Camera,
    ui_mode: bool,
    clear_color: Color,
}

const CIRCLE_POINTS: usize = 32; //the number of points in the poly to simulate the circle

impl Graphics {
    pub fn new(backend: Box<Backend>, cam: Camera) -> Graphics {
        Graphics {
            backend,
            cam,
            ui_mode: false,
            clear_color: Colors::BLACK,
        }
    }

    pub fn set_camera(&mut self, cam: Camera) {
        self.cam = cam;
    }

    pub fn get_ui_mode(&self) -> bool {
        self.ui_mode
    }

    pub fn set_ui_mode(&mut self, ui_mode: bool) {
        self.ui_mode = ui_mode;
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

    pub fn present(&mut self, ctx: &glutin::GlContext) {
        self.backend.display(self.clear_color);
        ctx.swap_buffers().unwrap();
    }

    pub fn draw_image(&mut self, image: TextureRegion, area: Rectangle) {
        self.draw_image_blend(image, area, Colors::WHITE);
    }

    pub fn draw_image_blend(&mut self, image: TextureRegion, area: Rectangle, col: Color) {
        self.draw_image_trans(image, area, col, Transform::identity());
    }

    pub fn draw_image_trans(&mut self, image: TextureRegion, area: Rectangle, col: Color, trans: Transform) {
        let trans = self.camera() * Transform::translate(area.top_left()) * trans *
            Transform::scale(area.size());
        let recip_size = image.source_size().recip();
        let normalized_pos = area.top_left().times(recip_size);
        let normalized_size = area.size().times(recip_size);
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
        self.draw_polygon_trans(&[rect.top_left(), rect.top_left() + rect.size().x_comp(), 
                rect.top_left() + rect.size(), rect.top_left() + rect.size().y_comp()], col, trans);
    }

    pub fn draw_circle(&mut self, circ: Circle, col: Color) {
        self.draw_circle_trans(circ, col, Transform::identity());
    }

    pub fn draw_circle_trans(&mut self, circ: Circle, col: Color, trans: Transform) {
        let mut points = [Vector::zero(); CIRCLE_POINTS];
        let rotation = Transform::rotate(360f32 / CIRCLE_POINTS as f32);
        let mut arrow = Vector::new(0f32, -circ.radius);
        for i in 0..CIRCLE_POINTS {
            points[i] = circ.center() + arrow;
            arrow = rotation * arrow;
        }
        self.draw_polygon_trans(&points, col, trans);
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
        self.draw_polygon_trans(&[line.start, line.start, line.end], col, trans);
    }

    pub fn draw_point(&mut self, vec: Vector, col: Color) {
        self.draw_polygon_trans(&[vec, vec, vec], col, Transform::identity());
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
