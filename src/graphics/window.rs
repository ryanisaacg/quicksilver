#[cfg(target_arch = "wasm32")]
use {
    error::QuicksilverError,
    stdweb::{
        web::{
            INode, document, window,
            html_element::CanvasElement,
        },
        unstable::TryInto
    }
};
use {
    Result, 
    geom::{Rectangle, Scalar, Transform, Vector},
     graphics::{Backend, BackendImpl, Background, BlendMode, Color, 
        Drawable, ImageScaleStrategy, Mesh, ResizeStrategy, View},
    input::{ButtonState, Gamepad, Keyboard, Mouse},
    lifecycle::{Event, GamepadProvider},
};
#[cfg(not(target_arch = "wasm32"))]
use {
    gl,
    glutin::{self, EventsLoop, GlContext}
};

///A builder that constructs a Window
#[derive(Debug)]
pub struct WindowBuilder {
    title: &'static str,
    width: u32,
    height: u32,
    show_cursor: bool,
    #[cfg(not(target_arch = "wasm32"))]
    min_size: Option<Vector>,
    #[cfg(not(target_arch = "wasm32"))]
    max_size: Option<Vector>,
    resize: ResizeStrategy,
    scale: ImageScaleStrategy,
    fullscreen: bool,
    tick_rate: f64,
}

impl WindowBuilder {
    ///Create a default window builder
    pub fn new(title: &'static str, size: impl Into<Vector>) -> WindowBuilder {
        let size = size.into();
        WindowBuilder {
            title,
            width: size.x as u32,
            height: size.y as u32,
            show_cursor: true,
            #[cfg(not(target_arch = "wasm32"))]
            min_size: None,
            #[cfg(not(target_arch = "wasm32"))]
            max_size: None,
            resize: ResizeStrategy::Fit,
            scale: ImageScaleStrategy::Pixelate,
            fullscreen: false,
            tick_rate: 1.0 / 60.0
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
        WindowBuilder { resize, ..self }
    }

    ///Set the minimum size for the window (no value by default)
    ///
    ///On the web, this does nothing.
    pub fn with_minimum_size(self, _min_size: impl Into<Vector>) -> WindowBuilder {
        WindowBuilder {
            #[cfg(not(target_arch = "wasm32"))]
            min_size: Some(_min_size.into()),
            ..self
        }
    }

    ///Set the maximum size for the window (no value by default)
    ///
    ///On the web, this does nothing.
    pub fn with_maximum_size(self, _max_size: impl Into<Vector>) -> WindowBuilder {
        WindowBuilder {
            #[cfg(not(target_arch = "wasm32"))]
            max_size: Some(_max_size.into()),
            ..self
        }
    }

    ///Set the strategy for scaling images
    pub fn with_scaling_strategy(self, scale: ImageScaleStrategy) -> WindowBuilder {
        WindowBuilder { scale, ..self }
    }

    ///Set if the window should be in fullscreen mode
    ///
    ///On desktop it's borderless fullscreen, and on the web it makes the canvas the size of the browser window
    pub fn with_fullscreen(self, fullscreen: bool) -> WindowBuilder {
        WindowBuilder { fullscreen, ..self }
    }

    /// Set the ideal delay between two calls to `update` in milliseconds
    ///
    /// By default it is 16
    pub fn with_tick_rate(self, tick_rate: f64) -> WindowBuilder {
        WindowBuilder { tick_rate, ..self }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn build(self) -> Result<(Window, EventsLoop)> {
        let events = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_decorations(!self.fullscreen)
            .with_title(self.title);
        let window = match self.min_size {
            Some(v) => window.with_min_dimensions(v.into()),
            None => window,
        };
        let window = match self.max_size {
            Some(v) => window.with_max_dimensions(v.into()),
            None => window,
        };
        let size = if self.fullscreen {
            events.get_primary_monitor().get_dimensions().into()
        } else {
            Vector::new(self.width, self.height)
        };
        let window = window.with_dimensions(size.into());
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events)?;
        unsafe {
            gl_window.make_current()?;
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        }
        if !self.show_cursor {
            gl_window.hide_cursor(true);
        }
        let screen_region = self.resize.resize(
            Vector::new(self.width, self.height),
            size,
        );
        let view = View::new(Rectangle::new_sized(screen_region.size()));
        let window = Window {
            gl_window,
            gamepads: Vec::new(),
            gamepad_buffer: Vec::new(),
            provider: GamepadProvider::new()?,
            resize: self.resize,
            screen_region,
            keyboard: Keyboard {
                keys: [ButtonState::NotPressed; 256],
            },
            mouse: Mouse {
                pos: Vector::ZERO,
                buttons: [ButtonState::NotPressed; 3],
                wheel: Vector::ZERO,
            },
            view,
            tick_rate: self.tick_rate,
            backend: unsafe { BackendImpl::new((), self.scale)? },
            mesh: Mesh::new(),
            frame_count: 0.0,
            fps: 0.0,
            last_framerate: 0.0,
        };
        Ok((window, events))
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn build(self) -> Result<(Window, CanvasElement)> {
        let mut actual_width = self.width;
        let mut actual_height = self.height;
        let document = document();
        let window = window();
        let element = match document.create_element("canvas") {
            Ok(elem) => elem,
            Err(_) => return Err(QuicksilverError::ContextError("Failed to create canvas element".to_owned()))
        };
        let canvas: CanvasElement = match element.try_into() {
            Ok(elem) => elem,
            Err(_) => return Err(QuicksilverError::ContextError("Failed to create canvas element".to_owned()))
        };
        let body = match document.body() {
            Some(body) => body,
            None => return Err(QuicksilverError::ContextError("Failed to find body node".to_owned()))
        };
        body.append_child(&canvas);
        document.set_title(self.title);
        if self.fullscreen {
            actual_width = window.inner_width() as u32;
            actual_height = window.inner_height() as u32;
        }
        canvas.set_width(actual_width);
        canvas.set_height(actual_height);
        js! ( @{&canvas}.style.cursor = @{self.show_cursor} ? "auto" : "none"; );
        let screen_region = self.resize.resize(
            Vector::new(self.width, self.height),
            Vector::new(actual_width, actual_height),
        );
        let view = View::new(Rectangle::new_sized(screen_region.size()));
        let window = Window {
            gamepads: Vec::new(),
            gamepad_buffer: Vec::new(),
            provider: GamepadProvider::new()?,
            resize: self.resize,
            screen_region,
            keyboard: Keyboard {
                keys: [ButtonState::NotPressed; 256],
            },
            mouse: Mouse {
                pos: Vector::ZERO,
                buttons: [ButtonState::NotPressed; 3],
                wheel: Vector::ZERO,
            },
            view,
            tick_rate: self.tick_rate,
            backend: unsafe { BackendImpl::new(canvas.clone(), self.scale)? },
            mesh: Mesh::new(),
            frame_count: 0.0,
            fps: 0.0,
            last_framerate: 0.0,
        };
        Ok((window, canvas))
    }
}

///The window currently in use
pub struct Window {
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) gl_window: glutin::GlWindow,
    provider: GamepadProvider,
    gamepads: Vec<Gamepad>,
    gamepad_buffer: Vec<Gamepad>, //used as a temporary buffer for storing new gamepads
    resize: ResizeStrategy,
    screen_region: Rectangle,
    keyboard: Keyboard,
    mouse: Mouse,
    view: View,
    tick_rate: f64,
    pub(crate) backend: BackendImpl,
    mesh: Mesh,
    frame_count: f64,
    fps: f64,
    last_framerate: f64,
}

impl Window {
    pub(crate) fn process_event(&mut self, event: &Event) {
        match event {
            &Event::Key(key, state) => self.keyboard.process_event(key as usize, state),
            &Event::MouseMoved(pos) => {
                self.mouse = Mouse {
                    pos: self.unproject() * pos,
                    ..self.mouse
                }
            }
            &Event::MouseWheel(wheel) => {
                self.mouse = Mouse {
                    wheel,
                    ..self.mouse
                }
            }
            &Event::MouseButton(button, state) => self.mouse.process_button(button, state),
            _ => (),
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
        let dpi;
        #[cfg(not(target_arch = "wasm32"))] {
            let size: glutin::dpi::LogicalSize = self.screen_region.size().into();
            self.gl_window.resize(size.to_physical(self.gl_window.get_hidpi_factor()));
            dpi = self.gl_window.get_hidpi_factor();
        }
        #[cfg(target_arch = "wasm32")] {
            dpi = 1.0;
        }
        let position = self.screen_region.top_left() * dpi as f32;
        let size = self.screen_region.size() * dpi as f32;
        unsafe {
            BackendImpl::viewport(
                position.x as i32,
                position.y as i32,
                size.x as i32,
                size.y as i32,
            );
        }
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
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn screen_offset(&self) -> Vector {
        self.screen_region.top_left()
    }

    ///Get the screen size
    pub fn screen_size(&self) -> Vector {
        self.screen_region.size()
    }

    ///Get the unprojection matrix according to the View
    pub fn unproject(&self) -> Transform {
        Transform::scale(self.screen_size()) * self.view.normalize
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

    #[cfg(not(target_arch = "wasm32"))]
    fn set_title_impl(&self, title: &str) {
        self.gl_window.set_title(title);
    }

    #[cfg(target_arch = "wasm32")]
    fn set_title_impl(&self, title: &str) {
        document().set_title(title);
    }

    /// Clear the screen to a given color
    ///
    /// The blend mode is also automatically reset,
    /// and any un-flushed draw calls are dropped.
    pub fn clear(&mut self, color: Color) -> Result<()> {
        self.clear_letterbox_color(color, Color::BLACK)
    }

    /// Clear the screen to a given color, with a given letterbox color
    ///
    /// The blend mode is also automatically reset,
    /// and any un-flushed draw calls are dropped.
    pub fn clear_letterbox_color(&mut self, color: Color, letterbox: Color) -> Result<()> {
        self.mesh.clear();
        unsafe {
            self.backend.reset_blend_mode();
            self.backend.clear_color(color, letterbox)
        }
    }

    /// Flush changes and also present the changes to the window
    pub fn present(&mut self) -> Result<()> {
        self.flush()?;
        #[cfg(not(target_arch = "wasm32"))]
        self.gl_window.swap_buffers()?;
        Ok(())
    }

    /// Flush the current buffered draw calls
    ///
    /// Until Window::present is called they won't be visible,
    /// but the items will be behind all future items drawn.
    ///
    /// Generally it's a bad idea to call this manually; as a general rule,
    /// the fewer times your application needs to flush the faster it will run.
    pub fn flush(&mut self) -> Result<()> {
        self.mesh.triangles.sort();
        for vertex in self.mesh.vertices.iter_mut() {
            vertex.pos = self.view.opengl * vertex.pos;
        }
        unsafe {
            self.backend.draw(self.mesh.vertices.as_slice(), self.mesh.triangles.as_slice())?;
        }
        self.mesh.clear();
        Ok(())
    }

    /// Set the blend mode for the window
    ///
    /// This will flush all of the drawn items to the screen and
    /// switch to the new blend mode.
    pub fn set_blend_mode(&mut self, blend: BlendMode) -> Result<()> {
        self.flush()?;
        unsafe {
            self.backend.set_blend_mode(blend);
        }
        Ok(())
    }

    /// Reset the blend mode for the window to the default alpha blending
    ///
    /// This will flush all of the drawn items to the screen
    pub fn reset_blend_mode(&mut self) -> Result<()> {
        self.flush()?;
        unsafe {
            self.backend.reset_blend_mode();
        }
        Ok(())
    }

    /// Get a reference to the connected gamepads
    pub fn gamepads(&self) -> &Vec<Gamepad> {
        &self.gamepads
    }

    /// Draw a Drawable to the window, which will be finalized on the next flush
    pub fn draw(&mut self, draw: &impl Drawable, bkg: Background) {
        self.draw_ex(draw, bkg, Transform::IDENTITY, 0);
    }

    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex(&mut self, draw: &impl Drawable, bkg: Background, trans: Transform, z: impl Scalar) {
        draw.draw(&mut self.mesh, bkg, trans, z);
    }
    
    /// The mesh the window uses to draw
    pub fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    /// The ideal delay between two calls to update in milliseconds
    pub fn tick_rate(&self) -> f64 {
        self.tick_rate
    }

    /// Set the desired time between two calls to update in milliseconds
    pub fn set_tick_rate(&mut self, tick_rate: f64) {
        self.tick_rate = tick_rate;
    }

    pub(crate) fn log_framerate(&mut self, delay: f64) {
        if delay > 0.0 {
            let total = self.frame_count * self.fps;
            self.frame_count += 1.0;
            let framerate = 1000.0 / delay;
            self.last_framerate = framerate;
            self.fps = (total + framerate) / self.frame_count;
        }
    }

    /// Get the delay between the last two draw frames
    pub fn current_fps(&self) -> f64 {
        self.last_framerate
    }

    /// Get the average framerate over the history of the app
    pub fn average_fps(&self) -> f64 {
        self.fps
    }
}
