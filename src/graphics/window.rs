use ffi::gl;
#[cfg(not(target_arch="wasm32"))] use glutin;
use geom::{ Rectangle, Transform, Vector};
#[cfg(not(target_arch="wasm32"))] use glutin::{EventsLoop, GlContext};
use graphics::{Backend, BlendMode, Color, Drawable, GpuTriangle, ResizeStrategy, Vertex, View};
#[cfg(feature="gamepads")] use input::{Gamepad, GamepadManager};
use input::{Button, ButtonState, Keyboard, Mouse};

/// The way the images should change when drawn at a scale
#[repr(u32)]
pub enum ImageScaleStrategy {
    /// The image should attempt to preserve each pixel as accurately as possible
    Pixelate = gl::NEAREST,
    /// The image should attempt to preserve the overall picture by blurring
    Blur = gl::LINEAR
}

///A builder that constructs a Window
pub struct WindowBuilder {
    show_cursor: bool,
    #[cfg(not(target_arch="wasm32"))]
    min_size: Option<Vector>,
    #[cfg(not(target_arch="wasm32"))]
    max_size: Option<Vector>,
    resize: ResizeStrategy,
    scale: ImageScaleStrategy,
    fullscreen: bool
}

impl WindowBuilder {
    ///Create a default window builder
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            show_cursor: true,
            #[cfg(not(target_arch="wasm32"))]
            min_size: None,
            #[cfg(not(target_arch="wasm32"))]
            max_size: None,
            resize: ResizeStrategy::Fit,
            scale: ImageScaleStrategy::Pixelate,
            fullscreen: false
        }
    }
   
    ///Set if the window should show its cursor (defaults to true)
    pub fn with_show_cursor(self, show_cursor: bool) -> WindowBuilder {
        WindowBuilder {
            show_cursor,
            ..self
        }
    }

    ///Set how the window should handle resizing (defaults to `ResizeStrategy::Fit`)
    pub fn with_resize_strategy(self, resize: ResizeStrategy) -> WindowBuilder {
        WindowBuilder {
            resize,
            ..self
        }
    }

    ///Set the minimum size for the window (no value by default)
    ///
    ///On the web, this does nothing.
    pub fn with_minimum_size(self, _min_size: Vector) -> WindowBuilder {
        WindowBuilder {
            #[cfg(not(target_arch="wasm32"))]
            min_size: Some(_min_size),
            ..self
        }
    }
    
    ///Set the maximum size for the window (no value by default)
    ///
    ///On the web, this does nothing.
    pub fn with_maximum_size(self, _max_size: Vector) -> WindowBuilder {
        WindowBuilder {
            #[cfg(not(target_arch="wasm32"))]
            max_size: Some(_max_size),
            ..self
        }
    }

    ///Set the strategy for scaling images
    pub fn with_scaling_strategy(self, scale: ImageScaleStrategy) -> WindowBuilder {
        WindowBuilder {
            scale,
            ..self
        }
    }

    ///Set if the window should be in fullscreen mode
    ///
    ///On desktop it's borderless fullscreen, and on the web it makes the canvas the size of the browser window
    pub fn with_fullscreen(self, fullscreen: bool) -> WindowBuilder {
        WindowBuilder {
            fullscreen,
            ..self
        }
    }

    ///Create a window and canvas with the given configuration
    pub fn build(self, title: &str, width: u32, height: u32) -> Window {
        let mut actual_width = width;
        let mut actual_height = height;
        #[cfg(not(target_arch="wasm32"))]
        let (gl_window, events) = {
            let events = glutin::EventsLoop::new();
            let window = glutin::WindowBuilder::new()
                .with_decorations(!self.fullscreen)
                .with_title(title);
            #[cfg(not(target_arch="wasm32"))]
            let window = match self.min_size { 
                Some(v) => window.with_min_dimensions(v.x as u32, v.y as u32),
                None => window
            };
            #[cfg(not(target_arch="wasm32"))]
            let window = match self.max_size {
                Some(v) => window.with_max_dimensions(v.x as u32, v.y as u32),
                None => window
            };
            if self.fullscreen {
                let (w, h) = events.get_primary_monitor().get_dimensions();
                actual_width = w;
                actual_height = h;
            }
            let window = window.with_dimensions(actual_width, actual_height);
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
            use ffi::wasm;
            use std::ffi::CString;
            unsafe { 
                wasm::set_show_mouse(self.show_cursor);
                if self.fullscreen {
                    actual_width = wasm::get_page_width();
                    actual_height = wasm::get_page_height();
                }
                wasm::create_context(CString::new(title).unwrap().into_raw(), actual_width, actual_height);
            }
        }
        let screen_region = self.resize.resize(Vector::new(width, height), Vector::new(actual_width, actual_height)); 
        #[cfg(not(target_arch="wasm32"))]
        let scale_factor = gl_window.hidpi_factor();
        #[cfg(target_arch="wasm32")]
        let scale_factor = 1f32;
        let view = View::new(Rectangle::newv_sized(screen_region.size()));
        Window {
            #[cfg(not(target_arch="wasm32"))]
            gl_window,
            #[cfg(not(target_arch="wasm32"))]
            events,
            #[cfg(feature="gamepads")]
            gamepads: GamepadManager::new(),
            resize: self.resize,
            screen_region,
            scale_factor,
            keyboard: Keyboard {
                keys: [ButtonState::NotPressed; 256]
            },
            mouse: Mouse {
                pos: Vector::zero(),
                buttons: [ButtonState::NotPressed; 3],
                wheel: Vector::zero()
            },
            view,
            previous_button: None,
            backend: Backend::new(self.scale as u32),
            vertices: Vec::new(),
            triangles: Vec::new()
        }
    }
}

///The window currently in use
pub struct Window {
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) gl_window: glutin::GlWindow,
    #[cfg(not(target_arch="wasm32"))]
    events: EventsLoop,
    #[cfg(feature="gamepads")]
    gamepads: GamepadManager,
    resize: ResizeStrategy,
    scale_factor: f32,
    screen_region: Rectangle,
    keyboard: Keyboard,
    mouse: Mouse,
    view: View,
    previous_button: Option<(Button, ButtonState)>,
    pub(crate) backend: Backend,
    vertices: Vec<Vertex>,
    triangles: Vec<GpuTriangle>
}

impl Window {
    ///Update the keyboard, mouse, and window state, and return if the window is still open
    pub fn poll_events(&mut self) -> bool {
        self.gamepads.update();
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
        let offset = self.screen_region.top_left();
        let mut button_change = None;
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
                            let change = keyboard.process_event(keycode as usize, state);
                            if let Some((button, state)) = change {
                                button_change = Some((Button::Keyboard(button), state));
                            }
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
                        let change = mouse.process_button(state, button);
                        if let Some((button, state)) = change {
                            button_change = Some((Button::Mouse(button), state));
                        }
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
        self.previous_button = button_change;
        running
    }
    
    #[cfg(target_arch="wasm32")]
    fn poll_events_impl(&mut self) -> bool {
        use ffi::wasm;
        let mut key = unsafe { wasm::pump_key_queue() };
        while key != 0 {
            let change = self.keyboard.process_event(key.abs() as usize - 1, key > 0);
            if let Some((button, state)) = change {
                self.previous_button = Some((Button::Keyboard(button), state));
            }
            key = unsafe { wasm::pump_key_queue() };
        }
        self.mouse = Mouse {
            pos: unsafe { Vector::new(wasm::get_mouse_x(), wasm::get_mouse_y()) } - self.screen_region.top_left(),
            ..self.mouse
        };
        let mut button = unsafe { wasm::pump_mouse_queue() };
        while button != 0 {
            let change = self.mouse.process_button(button.abs() as u32 - 1, button > 0);
            if let Some((button, state)) = change {
                self.previous_button = Some((Button::Mouse(button), state));
            }
            button = unsafe { wasm::pump_mouse_queue() };
        }
        let scroll = unsafe { wasm::mouse_scroll_type() };
        let x = unsafe { wasm::mouse_scroll_x() };
        let y = unsafe { wasm::mouse_scroll_y() };
        if scroll == 0 {
            self.mouse.process_wheel_pixels(x, y);
        } else {
            self.mouse.process_wheel_lines(x, y);
        }
        unsafe { wasm::mouse_scroll_clear(); }
        true
    }

    ///Handle the available size for the window changing
    fn adjust_size(&mut self, available: Vector) {
        self.screen_region = self.resize.resize(self.screen_region.size(), available);
        unsafe { gl::Viewport(self.screen_region.x as i32, self.screen_region.y as i32, 
                              self.screen_region.width as i32, self.screen_region.height as i32); }
        #[cfg(not(target_arch="wasm32"))]
        self.gl_window.resize(self.screen_region.width as u32, self.screen_region.height as u32);
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
        let available = self.resize.get_window_size(self.screen_region);
        self.resize = resize;
        self.adjust_size(available);
    }

    ///Get the screen size
    pub fn screen_size(&self) -> Vector {
        self.screen_region.size()
    }

    ///Get the unprojection matrix according to the View
    pub fn unproject(&self) -> Transform {
        Transform::scale(self.screen_size() / self.scale_factor)
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

    ///Get the button that changed its state and its current state
    pub fn last_button(&self) -> Option<(Button, ButtonState)> {
        self.previous_button
    }

    ///Clear the last button state
    pub fn clear_last_button(&mut self) {
        self.previous_button = None;
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
        use ffi::wasm;
        use std::ffi::CString;
        unsafe { wasm::set_title(CString::new(title).unwrap().into_raw()) };
    }
    
    /// Clear the screen to a given color
    ///
    /// The blend mode is also automatically reset,
    /// and any un-flushed draw calls are dropped.
    pub fn clear(&mut self, color: Color) {
        self.vertices.clear();
        self.triangles.clear();
        self.backend.clear(color);
        self.backend.reset_blend_mode();
    }

    /// Draw the changes made to the screen
    pub fn present(&mut self) {
        self.flush();
        #[cfg(not(target_arch="wasm32"))]
        self.gl_window.swap_buffers().unwrap();
    }

    /// Flush the current buffered draw calls
    ///
    /// Until Window::present is called they won't be visible,
    /// but the items will be behind all future items drawn.
    ///
    /// Generally it's a bad idea to call this manually; as a general rule,
    /// the fewer times your application needs to flush the faster it will run.
    pub fn flush(&mut self) {
        self.triangles.sort();
        self.backend.draw(self.vertices.as_slice(), self.triangles.as_slice());
        self.vertices.clear();
        self.triangles.clear();
    }

    /// Set the blend mode for the window
    ///
    /// This will flush all of the drawn items to the screen and 
    /// switch to the new blend mode.
    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        self.flush();
        self.backend.set_blend_mode(blend);
    }

    /// Reset the blend mode for the window to the default alpha blending
    ///
    /// This will flush all of the drawn items to the screen
    pub fn reset_blend_mode(&mut self) {
        self.flush();
        self.backend.reset_blend_mode();
    }

    /// Draw a single object to the screen
    pub fn draw<T: Drawable>(&mut self, item: &T) {
        item.draw(self);
    }

    /// Add vertices directly to the list without using a Drawable
    pub fn add_vertices<V, T>(&mut self, vertices: V, triangles: T) where V: Iterator<Item = Vertex>, T: Iterator<Item = GpuTriangle> {
        let offset = self.vertices.len() as u32;
        self.triangles.extend(triangles.map(|t| GpuTriangle {
            indices: [t.indices[0] + offset, t.indices[1] + offset, t.indices[2] + offset],
            ..t
        }));
        let opengl = self.view.opengl;
        self.vertices.extend(vertices.map(|v| Vertex {
            pos: opengl * v.pos,
            ..v
        }));
    }

    /// Get a reference to the connected gamepads
    pub fn gamepads(&self) -> &Vec<Gamepad> {
        self.gamepads.list()
    }
}
