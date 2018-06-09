use geom::{ Rectangle, Transform, Vector};
#[cfg(not(target_arch="wasm32"))]
use gl;
use graphics::{Backend, BackendImpl, BlendMode, Color, Drawable, GpuTriangle, ImageScaleStrategy, ResizeStrategy, Vertex, View};
use input::{ButtonState, Event, Gamepad, GamepadProvider, Keyboard, Mouse};
#[cfg(not(target_arch="wasm32"))]
use {
    glutin,
    glutin::{EventsLoop, GlContext}
};
#[cfg(target_arch="wasm32")]
use stdweb::{
    Value,
    unstable::{TryFrom, TryInto},
    web::{
        html_element::CanvasElement, 
        IParentNode, document, window
    }
};


///A builder that constructs a Window
#[derive(Debug)]
pub struct WindowBuilder {
    title: &'static str, 
    width: u32, 
    height: u32,
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
    pub fn new(title: &'static str, width: u32, height: u32) -> WindowBuilder {
        WindowBuilder {
            title,
            width,
            height,
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

    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn build(self) -> (Window, EventsLoop) {
        let mut actual_width = self.width;
        let mut actual_height = self.height;
        let events = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_decorations(!self.fullscreen)
            .with_title(self.title);
        let window = match self.min_size { 
            Some(v) => window.with_min_dimensions(v.x as u32, v.y as u32),
            None => window
        };
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
        let scale_factor = gl_window.hidpi_factor(); // Need to be calculated before moving gl_window
        let screen_region = self.resize.resize(Vector::new(self.width, self.height), Vector::new(actual_width, actual_height)); 
        let view = View::new(Rectangle::newv_sized(screen_region.size()));
        (Window {
            gl_window,
            gamepads: Vec::new(),
            gamepad_buffer: Vec::new(),
            provider: GamepadProvider::new(),
            resize: self.resize,
            screen_region,
            scale_factor,
            keyboard: Keyboard { keys: [ButtonState::NotPressed; 256] },
            mouse: Mouse { pos: Vector::zero(), buttons: [ButtonState::NotPressed; 3], wheel: Vector::zero() },
            view,
            backend: BackendImpl::new(self.scale),
            vertices: Vec::new(),
            triangles: Vec::new()
        }, events)
    }

    #[cfg(target_arch="wasm32")]
    pub(crate) fn build(self) -> Window {
        let mut actual_width = self.width;
        let mut actual_height = self.height;
        let document = document();
        let window = window();
        let canvas: CanvasElement = document.query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
        document.set_title(self.title);
        if self.fullscreen {
            actual_width = window.inner_width() as u32;
            actual_height = window.inner_height() as u32;
        }
        //TODO: GL VIEWPORT SIZE
        canvas.set_width(actual_width);
        canvas.set_height(actual_height);
        js! ( @{canvas}.style.cursor = @{self.show_cursor} ? "auto" : "none"; );
        let screen_region = self.resize.resize(Vector::new(self.width, self.height), Vector::new(actual_width, actual_height));
        let view = View::new(Rectangle::newv_sized(screen_region.size()));
        Window {
            gamepads: Vec::new(),
            gamepad_buffer: Vec::new(),
            provider: GamepadProvider::new(),
            resize: self.resize,
            screen_region,
            scale_factor: 1.0,
            keyboard: Keyboard { keys: [ButtonState::NotPressed; 256] },
            mouse: Mouse { pos: Vector::zero(), buttons: [ButtonState::NotPressed; 3], wheel: Vector::zero() },
            view,
            backend: BackendImpl::new(self.scale),
            vertices: Vec::new(),
            triangles: Vec::new()
        }
    }
}

///The window currently in use
pub struct Window {
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) gl_window: glutin::GlWindow,
    provider: GamepadProvider,
    gamepads: Vec<Gamepad>,
    gamepad_buffer: Vec<Gamepad>, //used as a temporary buffer for storing new gamepads
    resize: ResizeStrategy,
    pub(crate) scale_factor: f32,
    screen_region: Rectangle,
    keyboard: Keyboard,
    mouse: Mouse,
    view: View,
    pub(crate) backend: BackendImpl,
    vertices: Vec<Vertex>,
    triangles: Vec<GpuTriangle>
}

impl Window {
    pub(crate) fn process_event(&mut self, event: &Event) {
        match event {
            &Event::Key(key, state) => self.keyboard.process_event(key as usize, state),
            &Event::MouseMoved(pos) => self.mouse = Mouse { 
                pos: self.unproject() * pos, 
                ..self.mouse
            },
            &Event::MouseWheel(wheel) => self.mouse = Mouse { wheel, ..self.mouse },
            &Event::MouseButton(button, state) => self.mouse.process_button(button, state),
            _ => ()
        }
    }

    pub(crate) fn update_gamepads(&mut self, events: &mut Vec<Event>) {
        self.provider.provide_gamepads(&mut self.gamepad_buffer);
        let (mut i, mut j) = (0, 0);
        while i < self.gamepads.len() && j < self.gamepad_buffer.len() {
            if self.gamepads[i].id() == self.gamepad_buffer[j].id() {
                self.gamepad_buffer[j].set_previous(&self.gamepads[i], events);
                i += 1;
                j += 1;
            } else if self.gamepads[i].id() > self.gamepad_buffer[j].id() {
                events.push(Event::GamepadDisconnected(self.gamepad_buffer[j].id()));
                j += 1;
            } else {
                events.push(Event::GamepadConnected(self.gamepads[i].id()));
                i += 1;
            }
        }
        self.gamepads.clear();
        self.gamepads.append(&mut self.gamepad_buffer);
    }
    
    ///Transition temporary input states (Pressed, Released) into sustained ones (Held, NotPressed)
    pub fn clear_temporary_states(&mut self) {
        self.keyboard.clear_temporary_states();
        self.mouse.clear_temporary_states();
        for gamepad in self.gamepads.iter_mut() {
            gamepad.clear_temporary_states();
        }
    }

    ///Handle the available size for the window changing
    pub(crate) fn adjust_size(&mut self, available: Vector) {
        self.screen_region = self.resize.resize(self.screen_region.size(), available);
        BackendImpl::viewport(self.screen_region.x as i32, self.screen_region.y as i32, 
                              self.screen_region.width as i32, self.screen_region.height as i32);
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

    // Get the screen offset
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn screen_offset(&self) -> Vector {
        self.screen_region.top_left()
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
        document().set_title(title);
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

    /// Flush changes and also present the changes to the window
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
    ///
    /// It will not appear until Window::flush is called
    pub fn draw<T: Drawable>(&mut self, item: &T) {
        item.draw(self);
    }

    /// Add vertices directly to the list without using a Drawable
    ///
    /// Each vertex has a position in terms of the current view. The indices
    /// of the given GPU triangles are specific to these vertices, so that
    /// the index must be at least 0 and at most the number of vertices.
    /// Other index values will have undefined behavior
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
        &self.gamepads
    }
}
