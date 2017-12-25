use gl;
use glutin;
use geom::{ Rectangle, Vector};
use glutin::{EventsLoop, GlContext};
use graphics::{Backend, Camera, Canvas, Color};
use input::{Keyboard, Mouse, MouseBuilder, ViewportBuilder };

///A builder that constructs a Window and its Canvas
pub struct WindowBuilder {
    clear_color: Color,
    show_cursor: bool
}

impl WindowBuilder {
    ///Create a default window builder
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            clear_color: Color::black(),
            show_cursor: true
        }
    }
   
    ///Set if the window should show its cursor
    pub fn with_show_cursor(self, show_cursor: bool) -> WindowBuilder {
        WindowBuilder {
            show_cursor,
            ..self
        }
    }

    ///Set the window's default clear color
    pub fn with_clear_color(self, clear_color: Color) -> WindowBuilder {
        WindowBuilder {
            clear_color,
            ..self
        }
    }

    ///Create a window and canvas with the given configuration
    pub fn build(self, title: &str, width: u32, height: u32) -> (Window, Canvas) {
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
        gl_window.set_cursor_state(if self.show_cursor { 
            glutin::CursorState::Normal } else { glutin::CursorState::Hide }).unwrap();
        let scale_factor = gl_window.hidpi_factor();
        let screen_size = Vector::new(width as f32, height as f32);
        let window = Window {
            gl_window,
            events,
            scale_factor,
            offset: Vector::zero(),
            screen_size,
            keyboard: Keyboard::new(),
            mouse: Mouse::new(),
        };
        let canvas = Canvas {
            backend: Backend::new(),
            clear_color: self.clear_color,
            cam: Camera::new(Rectangle::newv_sized(screen_size)),
        };
        (window, canvas)
    }
}

///The window currently in use
pub struct Window {
    pub(crate) gl_window: glutin::GlWindow,
    events: EventsLoop,
    scale_factor: f32,
    offset: Vector,
    screen_size: Vector,
    keyboard: Keyboard,
    mouse: Mouse,
}

impl Window {
    ///Update the keyboard, mouse, and window state, and return if the window is still open
    pub fn poll_events(&mut self) -> bool {
        self.keyboard.clear_temporary_states();
        self.mouse.clear_temporary_states();
        let scale_factor = self.gl_window.hidpi_factor();
        let mut running = true;
        let mut screen_size = self.screen_size;
        let mut offset = self.offset;
        let target_ratio = self.screen_size.x / self.screen_size.y;
        let mut keyboard = self.keyboard.clone();
        let mut mouse = self.mouse.clone();
        self.events.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input: event,
                    } => {
                        keyboard.process_event(&event);
                    }
                    glutin::WindowEvent::CursorMoved { position, .. } => {
                        let (x, y) = position;
                        mouse = Mouse {
                            pos: (Vector::new(x as f32, y as f32) - offset) / scale_factor,
                            ..mouse
                        };
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
        self.screen_size = screen_size;
        self.offset = offset;
        self.keyboard = keyboard;
        self.mouse = mouse;
        running
    }


    ///Create a viewport builder
    pub fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder::new(self.screen_size / self.scale_factor)
    }

    ///Get the screen size
    pub fn screen_size(&self) -> Vector {
        self.screen_size
    }


    ///Get a reference to the keyboard
    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    ///Create a mouse builder
    pub fn mouse(&self) -> MouseBuilder {
        MouseBuilder {
            mouse: self.mouse.clone(),
            viewport: self.viewport()
        }
    }

}
