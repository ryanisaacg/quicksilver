use gl;
#[cfg(not(target_arch="wasm32"))]
use glutin;
use geom::{ Rectangle, Transform, Vector};
#[cfg(not(target_arch="wasm32"))]
use glutin::{EventsLoop, GlContext};
use graphics::{Backend, Canvas, ResizeStrategy, View};
use input::{ButtonState, Keyboard, Mouse};

///A builder that constructs a Window and its Canvas
pub struct WindowBuilder {
    show_cursor: bool,
    min_size: Option<Vector>,
    max_size: Option<Vector>,
    resize: ResizeStrategy,
}

#[cfg(target_arch="wasm32")]
extern "C" {
    fn set_show_mouse(show: bool);
    fn create_context(title: *mut i8, width: u32, height: u32);
    fn set_title(title: *mut i8);
    fn get_mouse_x() -> f32;
    fn get_mouse_y() -> f32;
    fn pump_key_queue() -> i32;
    fn pump_mouse_queue() -> i32;
    fn mouse_scroll_clear();
    fn mouse_scroll_type() -> u32;
    fn mouse_scroll_x() -> f32;
    fn mouse_scroll_y() -> f32;
}


impl WindowBuilder {
    ///Create a default window builder
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            show_cursor: true,
            min_size: None,
            max_size: None,
            resize: ResizeStrategy::Fit,
        }
    }
   
    ///Set if the window should show its cursor
    pub fn with_show_cursor(self, show_cursor: bool) -> WindowBuilder {
        WindowBuilder {
            show_cursor,
            ..self
        }
    }

    ///Set how the window should handle resizing
    pub fn with_resize_strategy(self, resize: ResizeStrategy) -> WindowBuilder {
        WindowBuilder {
            resize,
            ..self
        }
    }

    ///Set the minimum size for the window
    ///
    ///On the web, this does nothing.
    pub fn with_minimum_size(self, min_size: Vector) -> WindowBuilder {
        WindowBuilder {
            min_size: Some(min_size),
            ..self
        }
    }
    
    ///Set the maximum size for the window
    ///
    ///On the web, this does nothing.
    pub fn with_maximum_size(self, max_size: Vector) -> WindowBuilder {
        WindowBuilder {
            max_size: Some(max_size),
            ..self
        }
    }

    ///Create a window and canvas with the given configuration
    pub fn build(self, title: &str, width: u32, height: u32) -> (Window, Canvas) {
        #[cfg(not(target_arch="wasm32"))]
        let (gl_window, events) = {
            let events = glutin::EventsLoop::new();
            let window = glutin::WindowBuilder::new()
                .with_title(title);
            let window = match self.min_size { 
                Some(v) => window.with_min_dimensions(v.x as u32, v.y as u32),
                None => window
            };
            let window = match self.max_size {
                Some(v) => window.with_max_dimensions(v.x as u32, v.y as u32),
                None => window
            };
            let window = window.with_dimensions(width, height);
            let context = glutin::ContextBuilder::new().with_vsync(true);
            let gl_window = glutin::GlWindow::new(window, context, &events).unwrap();
            unsafe {
                gl_window.make_current().unwrap();
                gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            }
            gl_window.set_cursor_state(if self.show_cursor { 
                glutin::CursorState::Normal } else { glutin::CursorState::Hide }).unwrap();
            (gl_window, events)
        };
        #[cfg(target_arch="wasm32")] {
            use std::ffi::CString;
            unsafe { 
                set_show_mouse(self.show_cursor);
                create_context(CString::new(title).unwrap().into_raw(), width, height);
            }
        }
        let screen_size = Vector::new(width as f32, height as f32);
        #[cfg(not(target_arch="wasm32"))]
        let scale_factor = gl_window.hidpi_factor();
        #[cfg(target_arch="wasm32")]
        let scale_factor = 1f32;
        let view = View::new(Rectangle::newv_sized(screen_size));
        (Window {
            #[cfg(not(target_arch="wasm32"))]
            gl_window,
            #[cfg(not(target_arch="wasm32"))]
            events,
            resize: self.resize,
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
                right: ButtonState::NotPressed,
                wheel: Vector::zero()
            },
            view
        }, Canvas {
            backend: Backend::new(),
            view
        })
    }
}

///The window currently in use
pub struct Window {
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) gl_window: glutin::GlWindow,
    #[cfg(not(target_arch="wasm32"))]
    events: EventsLoop,
    resize: ResizeStrategy,
    scale_factor: f32,
    offset: Vector,
    screen_size: Vector,
    keyboard: Keyboard,
    mouse: Mouse,
    view: View,
}

impl Window {
    ///Update the keyboard, mouse, and window state, and return if the window is still open
    pub fn poll_events(&mut self) -> bool {
        self.poll_events_impl()
    }

    ///Transition temporary input states (Pressed, Released) into sustained ones (Held, NotPressed)
    pub fn clear_temporary_states(&mut self) {
        self.keyboard.clear_temporary_states();
        self.mouse.clear_temporary_states();
    }

    #[cfg(not(target_arch="wasm32"))]
    fn poll_events_impl(&mut self) -> bool {
        let scale_factor = self.gl_window.hidpi_factor();
        let mut running = true;
        let mut keyboard = self.keyboard.clone();
        let mut mouse = self.mouse.clone();
        let mut resized = None;
        let offset = self.offset;
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
                    glutin::WindowEvent::MouseWheel { delta, .. } => {
                        match delta {
                            glutin::MouseScrollDelta::LineDelta(x, y) => mouse.process_wheel_lines(x, -y),
                            glutin::MouseScrollDelta::PixelDelta(x, y) => mouse.process_wheel_pixels(x, y)
                        }
                    }
                    glutin::WindowEvent::Closed => {
                        running = false;
                    }
                    glutin::WindowEvent::Resized(new_width, new_height) => {
                        resized = Some(Vector::new(new_width as f32, new_height as f32));
                    }
                    _ => (),
                }
            }
            _ => (),
        });
        if let Some(vector) = resized {
            self.adjust_size(vector);
        }
        self.keyboard = keyboard;
        self.mouse = mouse;
        running
    }
    
    #[cfg(target_arch="wasm32")]
    fn poll_events_impl(&mut self) -> bool {
        let mut key = unsafe { pump_key_queue() };
        while key != 0 {
            self.keyboard.process_event(key.abs() as usize - 1, key > 0);
            key = unsafe { pump_key_queue() };
        }
        self.mouse = Mouse {
            pos: unsafe { Vector::new(get_mouse_x(), get_mouse_y()) } - self.offset,
            ..self.mouse
        };
        let mut button = unsafe { pump_mouse_queue() };
        while button != 0 {
            self.mouse.process_button(button.abs() as u32 - 1, button > 0);
            button = unsafe { pump_mouse_queue() };
        }
        let scroll = unsafe { mouse_scroll_type() };
        let x = unsafe { mouse_scroll_x() };
        let y = unsafe { mouse_scroll_y() };
        if scroll == 0 {
            self.mouse.process_wheel_pixels(x, y);
        } else {
            self.mouse.process_wheel_lines(x, y);
        }
        unsafe { mouse_scroll_clear(); }
        true
    }

    ///Handle the available size for the window changing
    fn adjust_size(&mut self, available: Vector) {
        let view = self.resize.resize(self.screen_size, available);
        self.offset = view.top_left();
        self.screen_size = view.size();
        unsafe { gl::Viewport(self.offset.x as i32, self.offset.y as i32, 
                              self.screen_size.x as i32, self.screen_size.y as i32); }
        #[cfg(not(target_arch="wasm32"))]
        self.gl_window.resize(self.screen_size.x as u32, self.screen_size.y as u32);
    }


    ///Get the view from the window
    pub fn view(&self) -> View {
        self.view
    }

    ///Set the view the window uses
    pub fn set_view(&mut self, view: View) {
        self.view = view;
    }

    ///Get the resize strategy used by the window
    pub fn resize_strategy(&self) -> ResizeStrategy {
        self.resize
    }
    
    ///Switch the strategy the window uses to display content when the available area changes
    pub fn set_resize_strategy(&mut self, resize: ResizeStrategy) {
        //Find the previous window size and reconfigure to match the new strategy
        let available = self.resize.get_window_size(self.offset, self.screen_size);
        self.resize = resize;
        self.adjust_size(available);
    }

    ///Get the screen size
    pub fn screen_size(&self) -> Vector {
        self.screen_size
    }

    ///Get the unprojection matrix according to the View
    pub fn unproject(&self) -> Transform {
        Transform::scale(self.screen_size / self.scale_factor)
            * self.view.normalize
    }
    
    ///Get the projection matrix according to the View
    pub fn project(&self) -> Transform {
        self.unproject().inverse()
    }

    ///Get a reference to the keyboard
    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    ///Get an instance of a mouse, projected into the current View
    pub fn mouse(&self) -> Mouse {
        Mouse {
            pos: self.project() * self.mouse.pos,
            ..self.mouse.clone()
        }
    }

    ///Set the title of the Window
    pub fn set_title(&self, title: &str) {
        self.set_title_impl(title);
    }

    #[cfg(not(target_arch="wasm32"))]
    fn set_title_impl(&self, title: &str) {
        self.gl_window.set_title(title);
    }
    
    #[cfg(target_arch="wasm32")]
    fn set_title_impl(&self, title: &str) {
        use std::ffi::CString;
        unsafe { set_title(CString::new(title).unwrap().into_raw()) };
    }
}
