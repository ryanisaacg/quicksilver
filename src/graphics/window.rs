#[cfg(not(target_arch="wasm32"))]
use gl;
#[cfg(not(target_arch="wasm32"))]
use glutin;
use geom::{ Rectangle, Transform, Vector};
#[cfg(not(target_arch="wasm32"))]
use glutin::{EventsLoop, GlContext};
use graphics::{Backend, Camera, Canvas};
use input::{ButtonState, Keyboard, Mouse, Viewport, ViewportBuilder };


///A builder that constructs a Window and its Canvas
pub struct WindowBuilder {
    show_cursor: bool
}

#[cfg(target_arch="wasm32")]
extern "C" {
    pub fn set_show_mouse(show: bool);
    pub fn create_context(title: *mut i8, width: u32, height: u32);
    pub fn get_mouse_x() -> f32;
    pub fn get_mouse_y() -> f32;
    pub fn pump_key_queue() -> i32;
    pub fn pump_mouse_queue() -> i32;
}


impl WindowBuilder {
    ///Create a default window builder
    pub fn new() -> WindowBuilder {
        WindowBuilder {
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

    ///Create a window and canvas with the given configuration
    #[cfg(not(target_arch="wasm32"))]
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
            keyboard: Keyboard {
                keys: [ButtonState::NotPressed; 256]
            },
            mouse: Mouse {
                pos: Vector::zero(),
                left: ButtonState::NotPressed,
                middle: ButtonState::NotPressed,
                right: ButtonState::NotPressed
            }
        };
        let canvas = Canvas {
            backend: Backend::new(),
            cam: Camera::new(Rectangle::newv_sized(screen_size)),
        };
        (window, canvas)
    }
    
    #[cfg(target_arch="wasm32")]
    pub fn build(self, title: &str, width: u32, height: u32) -> (Window, Canvas) {
        use std::ffi::CString;
        unsafe { set_show_mouse(self.show_cursor) };
        unsafe { create_context(CString::new(title).unwrap().into_raw(), width, height) };
        let screen_size = Vector::new(width as f32, height as f32);
        let window = Window {
            screen_size,
            keyboard: Keyboard {
                keys: [ButtonState::NotPressed; 256]
            },
            mouse: Mouse {
                pos: Vector::zero(),
                left: ButtonState::NotPressed,
                middle: ButtonState::NotPressed,
                right: ButtonState::NotPressed
            }
        };
        let canvas = Canvas {
            backend: Backend::new(),
            cam: Camera::new(Rectangle::newv_sized(screen_size)),
        };
        (window, canvas)
    }
}

///The window currently in use
#[cfg(not(target_arch="wasm32"))]
pub struct Window {
    pub(crate) gl_window: glutin::GlWindow,
    events: EventsLoop,
    scale_factor: f32,
    offset: Vector,
    screen_size: Vector,
    keyboard: Keyboard,
    mouse: Mouse,
}

#[cfg(target_arch="wasm32")]
pub struct Window {
    screen_size: Vector,
    keyboard: Keyboard,
    mouse: Mouse
}

impl Window {
    ///Update the keyboard, mouse, and window state, and return if the window is still open
    #[cfg(not(target_arch="wasm32"))]
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
                        if let Some(keycode) = event.virtual_keycode {
                            let state = match event.state {
                                glutin::ElementState::Pressed => true,
                                glutin::ElementState::Released => false
                            };
                            keyboard.process_event(keycode as usize, state);
                        }
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
    
    #[cfg(target_arch="wasm32")]
    pub fn poll_events(&mut self) -> bool {
        self.keyboard.clear_temporary_states();
        let mut key = unsafe { pump_key_queue() };
        while key != 0 {
            self.keyboard.process_event(key.abs() as usize - 1, key > 0);
            key = unsafe { pump_key_queue() };
        }
        self.mouse = Mouse {
            pos: unsafe { Vector::new(get_mouse_x(), get_mouse_y()) },
            ..self.mouse
        };
        let mut button = unsafe { pump_mouse_queue() };
        while button != 0 {
            self.mouse.process_button(button.abs() as u32 - 1, button > 0);
            button = unsafe { pump_mouse_queue() };
        }
        true
    }

    ///Create a viewport builder
    #[cfg(not(target_arch="wasm32"))]
    pub fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder {
            screen_size: self.screen_size / self.scale_factor,
            transform: Transform::identity()
        }
    }
    
    #[cfg(target_arch="wasm32")]
    pub fn viewport(&self) -> ViewportBuilder {
        ViewportBuilder {
            screen_size: self.screen_size,
            transform: Transform::identity()
        }
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
    pub fn mouse(&self, viewport: &Viewport) -> Mouse {
        Mouse {
            pos: viewport.project() * self.mouse.pos,
            ..self.mouse.clone()
        }
    }

}
