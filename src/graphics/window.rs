extern crate gl;
extern crate glutin;

use geom::{Circle, Line, Rectangle, Shape, Transform, Vector};
use glutin::{EventsLoop, GlContext};
use graphics::{Backend, GLBackend, Camera, Color, Colors, Image, Vertex};
use input::{Keyboard, Mouse, ViewportBuilder };

pub struct WindowBuilder {
    clear_color: Color,
    show_cursor: bool
}

impl WindowBuilder {
    pub(crate) fn new() -> WindowBuilder {
        WindowBuilder {
            clear_color: Colors::BLACK,
            show_cursor: true
        }
    }
    
    pub fn with_show_cursor(self, show_cursor: bool) -> WindowBuilder {
        WindowBuilder {
            show_cursor,
            ..self
        }
    }

    pub fn with_clear_color(self, clear_color: Color) -> WindowBuilder {
        WindowBuilder {
            clear_color,
            ..self
        }
    }

    pub fn build(self, title: &str, width: u32, height: u32) -> Window {
        Window::new(self, title, width, height)
    }
}

pub struct Window {
    backend: Box<Backend>,
    cam: Camera,
    clear_color: Color,
    show_cursor: bool,
    events: EventsLoop,
    gl_window: glutin::GlWindow,
    running: bool,
    scale_factor: f32,
    offset: Vector,
    screen_size: Vector
}

const CIRCLE_POINTS: usize = 32; //the number of points in the poly to simulate the circle

impl Window {
    pub(crate) fn new(builder: WindowBuilder, title: &str, width: u32, height: u32) -> Window {
        let events = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events).unwrap();
        unsafe {
            gl_window.make_current().unwrap();
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        }
        let scale_factor = gl_window.hidpi_factor();
        let screen_size = Vector::new(width as f32, height as f32);

        Window {
            backend: Box::new(GLBackend::new()),
            cam: Camera::new(Rectangle::newv_sized(screen_size)),
            clear_color: builder.clear_color,
            show_cursor: builder.show_cursor,
            events,
            gl_window,
            running: true,
            scale_factor,
            offset: Vector::zero(),
            screen_size
        }
    }

    pub fn poll_events(&mut self, keyboard: &mut Keyboard, mouse: &mut Mouse) {
        keyboard.clear_temporary_states();
        mouse.clear_temporary_states();
        self.scale_factor = self.gl_window.hidpi_factor();
        let scale_factor = self.scale_factor;
        let mut running = true;
        let mut screen_size = self.screen_size;
        let mut offset = self.offset;
        let target_ratio = self.screen_size.x / self.screen_size.y;
        self.events.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input: event,
                    } => {
                        keyboard.process_event(&event);
                    }
                    glutin::WindowEvent::MouseMoved { position, .. } => {
                        let (x, y) = position;
                        *mouse = mouse.with_position(
                            (Vector::new(x as f32, y as f32) - offset) / scale_factor);
                    }
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        mouse.process_button(state, button);
                    }
                    glutin::WindowEvent::Closed => {
                        running = false;
                    }
                    glutin::WindowEvent::Resized(new_width, new_height) => {
                        let window_ratio = new_width as f32 / new_height as f32;
                        let (w, h) = if window_ratio > target_ratio {
                            ((target_ratio * new_height as f32) as i32, new_height as i32)
                        } else if window_ratio < target_ratio {
                            (new_width as i32, (new_width as f32 / target_ratio) as i32)
                        } else {
                            (new_width as i32, new_height as i32)
                        };
                        let offset_x = (new_width as i32 - w) / 2;
                        let offset_y = (new_height as i32 - h) / 2;
                        screen_size = Vector::new(w as f32, h as f32);
                        offset = Vector::newi(offset_x, offset_y);
                        unsafe {
                            gl::Viewport(offset_x, offset_y, w, h);
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        });
        self.running = running;
        self.screen_size = screen_size;
        self.offset = offset;
    }

    pub fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::new(self.screen_size / self.scale_factor)
    }

    pub fn screen_size(&self) -> Vector {
        self.screen_size
    }

    pub fn running(&self) -> bool {
        self.running
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

    pub fn present(&mut self) {
        self.gl_window.set_cursor_state(if self.show_cursor { glutin::CursorState::Normal } else { glutin::CursorState::Hide }).unwrap();
        self.backend.display();
        self.gl_window.swap_buffers().unwrap();
        self.backend.clear(self.clear_color);
    }

    pub fn draw_image(&mut self, image: Image, area: Rectangle) {
        self.draw_image_blend(image, area, Colors::WHITE);
    }

    pub fn draw_image_blend(&mut self, image: Image, area: Rectangle, col: Color) {
        self.draw_image_trans(image, area, col, Transform::identity());
    }

    pub fn draw_image_trans(&mut self, image: Image, area: Rectangle, col: Color, trans: Transform) {
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
