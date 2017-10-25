extern crate glutin;

use geom::{Circle, Rectangle, Transform, Vector};
use graphics::{BLACK, Backend, Camera, Color, TextureRegion, Vertex};

pub struct Graphics {
    backend: Box<Backend>,
    cam: Camera,
    ui_mode: bool,
    clear_color: Color
}

const CIRCLE_POINTS: usize = 32; //the number of points in the poly to simulate the circle

impl Graphics {
    pub fn new(backend: Box<Backend>, cam: Camera) -> Graphics {
        Graphics {
            backend,
            cam,
            ui_mode: false,
            clear_color: BLACK
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
        self.cam.opengl
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

    pub fn draw_image(&mut self, image: TextureRegion, area: Rectangle, trans: Transform, col: Color) {
        let trans = self.camera()
            * Transform::translate(area.top_left()) 
            * trans 
            * Transform::scale(area.size());
        let recip_size = image.source_size().recip();
        let normalized_pos = area.top_left().times(recip_size);
        let normalized_size = area.size().times(recip_size);
        let get_vertex = |v: Vector| {
            Vertex {
                pos: trans * v,
                tex_pos: normalized_pos + v.times(normalized_size),
                col: col,
                use_texture: true
            }
        };
        self.backend.add(image.get_id(), &[get_vertex(Vector::zero()),
                    get_vertex(Vector::zero() + Vector::x()),
                    get_vertex(Vector::zero() + Vector::one()),
                    get_vertex(Vector::zero() + Vector::y())],
                    &[0, 1, 2, 2, 3, 0]);
    }

    pub fn draw_rect(&mut self, rect: Rectangle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans;
        self.draw_polygon(&[rect.top_left(), 
                          rect.top_left() + rect.size().x_comp(),
                          rect.top_left() + rect.size(),
                          rect.top_left() + rect.size().y_comp()], trans, col);
    }

    pub fn draw_circle(&mut self, circ: Circle, trans: Transform, col: Color) {
        let trans = self.camera()
            * trans;
        let mut points = [Vector::zero(); CIRCLE_POINTS];
        let rotation = Transform::rotate(360f32 / CIRCLE_POINTS as f32);
        let mut arrow = Vector::new(0f32, -circ.radius);
        for i in 0..CIRCLE_POINTS {
            points[i] = circ.center() + arrow;
            arrow = rotation * arrow;
        }
        self.draw_polygon(&points, trans, col);
    }
    
    pub fn draw_polygon(&mut self, vertices: &[Vector], trans: Transform, col: Color) {
        let first_index = self.backend.num_vertices() as u32;
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
}
