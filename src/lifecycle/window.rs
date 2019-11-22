use crate::{
    backend::{instance, set_instance, Backend, BackendImpl},
    geom::{Rectangle, Scalar, Transform, Vector},
    graphics::{Background, BlendMode, Color, Drawable, Mesh, PixelFormat, ResizeStrategy, View},
    input::{ButtonState, Gamepad, Keyboard, Mouse, MouseCursor},
    lifecycle::{Event, Settings},
    Result,
};
use image::{DynamicImage, RgbImage, RgbaImage};
#[cfg(target_arch = "wasm32")]
use {
    crate::error::QuicksilverError,
    stdweb::{
        unstable::TryInto,
        web::{document, html_element::CanvasElement, IElement, INode},
    },
};
#[cfg(feature = "gilrs")]
use {
    crate::input::{GAMEPAD_BUTTON_LIST, GILRS_GAMEPAD_LIST},
    gilrs::{ev::state::AxisData, Axis, Gilrs},
};
#[cfg(not(target_arch = "wasm32"))]
use {
    gl,
    glutin::{self, EventsLoop, Icon},
};

///The window currently in use
pub struct Window {
    #[cfg(feature = "gilrs")]
    gilrs: Gilrs,
    gamepads: Vec<Gamepad>,
    gamepad_buffer: Vec<Gamepad>, //used as a temporary buffer for storing new gamepads
    resize: ResizeStrategy,
    screen_region: Rectangle,
    keyboard: Keyboard,
    mouse: Mouse,
    view: View,
    update_rate: f64,
    max_updates: u32,
    draw_rate: f64,
    mesh: Mesh,
    frame_count: f64,
    fps: f64,
    last_framerate: f64,
    running: bool,
    fullscreen: bool,
}

impl Window {
    pub(crate) fn build_agnostic(
        user_size: Vector,
        actual_size: Vector,
        settings: Settings,
    ) -> Result<Window> {
        let screen_region = settings.resize.resize(user_size, actual_size);
        let view = View::new(Rectangle::new_sized(screen_region.size()));
        let mut window = Window {
            gamepads: Vec::new(),
            gamepad_buffer: Vec::new(),
            #[cfg(feature = "gilrs")]
            gilrs: Gilrs::new()?,
            resize: settings.resize,
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
            update_rate: settings.update_rate,
            max_updates: settings.max_updates,
            draw_rate: settings.draw_rate,
            mesh: Mesh::new(),
            frame_count: 0.0,
            fps: 0.0,
            last_framerate: 0.0,
            running: true,
            fullscreen: false,
        };
        window.set_cursor(if settings.show_cursor {
            MouseCursor::Default
        } else {
            MouseCursor::None
        });
        window.set_fullscreen(settings.fullscreen);
        Ok(window)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn build(
        title: &str,
        user_size: Vector,
        settings: Settings,
    ) -> Result<(Window, EventsLoop)> {
        let events = glutin::EventsLoop::new();
        let mut window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(user_size.into());
        if let Some(path) = settings.icon_path {
            window = window.with_window_icon(Some(Icon::from_path(path)?));
        }
        if let Some(v) = settings.min_size {
            window = window.with_min_dimensions(v.into());
        }
        if let Some(v) = settings.max_size {
            window = window.with_max_dimensions(v.into());
        };
        let context = glutin::ContextBuilder::new()
            .with_vsync(settings.vsync)
            .with_multisampling(settings.multisampling.unwrap_or(0));
        let gl_window = context.build_windowed(window, &events)?;
        let gl_window = unsafe {
            let gl_window = match gl_window.make_current() {
                Ok(window) => window,
                Err((_, err)) => Err(err)?,
            };
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

            gl_window
        };
        unsafe {
            set_instance(BackendImpl::new(
                gl_window,
                settings.scale,
                settings.multisampling != None,
            )?)
        };
        let window = Window::build_agnostic(user_size, user_size, settings)?;
        Ok((window, events))
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn build(
        title: &str,
        size: Vector,
        settings: Settings,
    ) -> Result<(Window, CanvasElement)> {
        let document = document();
        if let Some(path) = settings.icon_path {
            let head = document.head().ok_or(QuicksilverError::ContextError(
                "Failed to find head node".to_owned(),
            ))?;
            let element = document.create_element("link").map_err(|_| {
                QuicksilverError::ContextError("Failed to create link element".to_owned())
            })?;
            element.set_attribute("rel", "shortcut icon").map_err(|_| {
                QuicksilverError::ContextError("Failed to create favicon element".to_owned())
            })?;
            element.set_attribute("type", "image/png").map_err(|_| {
                QuicksilverError::ContextError("Failed to create favicon element".to_owned())
            })?;
            element.set_attribute("href", path).map_err(|_| {
                QuicksilverError::ContextError("Failed to create favicon element".to_owned())
            })?;
            head.append_child(&element);
        }
        let element = document.create_element("canvas").map_err(|_| {
            QuicksilverError::ContextError("Failed to create canvas element".to_owned())
        })?;
        let canvas: CanvasElement = element.try_into().map_err(|_| {
            QuicksilverError::ContextError("Failed to create canvas element".to_owned())
        })?;
        let body = document.body().ok_or(QuicksilverError::ContextError(
            "Failed to find body node".to_owned(),
        ))?;
        body.append_child(&canvas);
        canvas.set_width(size.x as u32);
        canvas.set_height(size.y as u32);
        unsafe {
            set_instance(BackendImpl::new(
                canvas.clone(),
                settings.scale,
                settings.multisampling != None,
            )?)
        };
        let mut window = Window::build_agnostic(size, size, settings)?;
        window.set_title(title);
        Ok((window, canvas))
    }

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
        #[cfg(feature = "gilrs")]
        self.update_gamepads_impl();

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

    #[cfg(feature = "gilrs")]
    pub(crate) fn update_gamepads_impl(&mut self) {
        while let Some(ev) = self.gilrs.next_event() {
            self.gilrs.update(&ev);
        }
        fn axis_value(data: Option<&AxisData>) -> f32 {
            match data {
                Some(ref data) => data.value(),
                None => 0.0,
            }
        }
        self.gamepad_buffer
            .extend(self.gilrs.gamepads().map(|(id, gamepad)| {
                let id: usize = id.into();
                let id = id as i32;

                let axes = [
                    axis_value(gamepad.axis_data(Axis::LeftStickX)),
                    axis_value(gamepad.axis_data(Axis::LeftStickY)),
                    axis_value(gamepad.axis_data(Axis::RightStickX)),
                    axis_value(gamepad.axis_data(Axis::RightStickY)),
                ];

                let mut buttons = [ButtonState::NotPressed; 17];
                for i in 0..GAMEPAD_BUTTON_LIST.len() {
                    let button = GAMEPAD_BUTTON_LIST[i];
                    let value = match gamepad.button_data(GILRS_GAMEPAD_LIST[i]) {
                        Some(ref data) => data.is_pressed(),
                        None => false,
                    };
                    let state = if value {
                        ButtonState::Pressed
                    } else {
                        ButtonState::Released
                    };
                    buttons[button as usize] = state;
                }

                Gamepad { id, axes, buttons }
            }));
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
        unsafe {
            self.backend().set_viewport(self.screen_region);
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
            self.backend().reset_blend_mode();
            self.backend().clear_color(color, letterbox)
        }
    }

    /// Flush the current buffered draw calls
    ///
    /// Attributes like z-ordering will be reset: all items drawn after a flush will *always* draw
    /// over all items drawn before a flush.
    ///
    /// Note that calling this can be an expensive operation
    pub fn flush(&mut self) -> Result<()> {
        self.mesh.triangles.sort();
        for vertex in self.mesh.vertices.iter_mut() {
            vertex.pos = self.view.opengl * vertex.pos;
        }
        unsafe {
            self.backend().draw(
                self.mesh.vertices.as_slice(),
                self.mesh.triangles.as_slice(),
            )?;
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
            self.backend().set_blend_mode(blend);
        }
        Ok(())
    }

    /// Reset the blend mode for the window to the default alpha blending
    ///
    /// This will flush all of the drawn items to the screen
    pub fn reset_blend_mode(&mut self) -> Result<()> {
        self.flush()?;
        unsafe {
            self.backend().reset_blend_mode();
        }
        Ok(())
    }

    /// Get a reference to the connected gamepads
    pub fn gamepads(&self) -> &Vec<Gamepad> {
        &self.gamepads
    }

    /// Draw a Drawable to the window, which will be finalized on the next flush
    pub fn draw<'a>(&'a mut self, draw: &impl Drawable, bkg: impl Into<Background<'a>>) {
        self.draw_ex(draw, bkg.into(), Transform::IDENTITY, 0);
    }

    /// Draw a Drawable to the window with more options provided (draw exhaustive)
    pub fn draw_ex<'a>(
        &'a mut self,
        draw: &impl Drawable,
        bkg: impl Into<Background<'a>>,
        trans: Transform,
        z: impl Scalar,
    ) {
        draw.draw(&mut self.mesh, bkg.into(), trans, z);
    }

    /// The mesh the window uses to draw
    pub fn mesh(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    /// The ideal delay between two calls to update in milliseconds
    pub fn update_rate(&self) -> f64 {
        self.update_rate
    }

    /// Set the desired time between two calls to update in milliseconds
    pub fn set_update_rate(&mut self, update_rate: f64) {
        self.update_rate = update_rate;
    }

    /// The ideal delay between two calls to draw in milliseconds
    pub fn draw_rate(&self) -> f64 {
        self.draw_rate
    }

    /// Set the desired time between two calls to draw in milliseconds
    pub fn set_draw_rate(&mut self, draw_rate: f64) {
        self.draw_rate = draw_rate;
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

    /// Get the maximum number of updates that are allowed to run in a frame
    ///
    /// 0 means no limitation
    pub fn max_updates(&self) -> u32 {
        self.max_updates
    }

    /// Set the maximum number of updates that are allowed to run in a frame
    ///
    /// 0 means no limitation
    pub fn set_max_updates(&mut self, max_updates: u32) {
        self.max_updates = max_updates;
    }

    /// Set if the cursor should be visible when over the application
    #[deprecated(since = "0.3.5", note = "please use `set_cursor` instead")]
    pub fn set_show_cursor(&mut self, show_cursor: bool) {
        if show_cursor {
            self.set_cursor(MouseCursor::Default);
        } else {
            self.set_cursor(MouseCursor::None);
        }
    }

    /// Set current cursor
    pub fn set_cursor(&mut self, cursor: MouseCursor) {
        self.backend().set_cursor(cursor);
    }

    /// Set the title of the window (or tab on mobile)
    pub fn set_title(&mut self, title: &str) {
        self.backend().set_title(title);
    }

    /// Get if the application is currently fullscreen
    pub fn get_fullscreen(&self) -> bool {
        self.fullscreen
    }

    /// Set if the application is currently fullscreen
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.fullscreen = fullscreen;
        let size = self.backend().set_fullscreen(fullscreen);
        if let Some(size) = size {
            self.adjust_size(size);
        }
    }

    /// Resize the window to the given size
    pub fn set_size(&mut self, size: impl Into<Vector>) {
        let size = size.into();
        self.backend().resize(size);
        self.adjust_size(size);
    }

    /// Close the application without triggering the onclose handler
    ///
    /// On desktop, this closes the window, and on the web it removes the canvas from the page
    pub fn close(&mut self) {
        self.running = false;
    }

    /// Create a screenshot as an image
    ///
    /// If a surface is active, an image of that surface is generated. If no surface is activated,
    /// an image of what has been drawn to the window is generated. Taking a screenshot in the
    /// midst of a `draw` call may have unexpected results.
    pub fn screenshot(&mut self, format: PixelFormat) -> DynamicImage {
        let (size, buffer) = unsafe { self.backend().screenshot(format) };
        let width = size.x as u32;
        let height = size.y as u32;
        let img = match format {
            PixelFormat::RGB => {
                DynamicImage::ImageRgb8(RgbImage::from_raw(width, height, buffer).expect("TODO"))
            }
            PixelFormat::RGBA => {
                DynamicImage::ImageRgba8(RgbaImage::from_raw(width, height, buffer).expect("TODO"))
            }
        };
        img.flipv()
    }

    pub(crate) fn is_running(&self) -> bool {
        self.running
    }

    pub(crate) fn backend(&mut self) -> &'static mut BackendImpl {
        unsafe { instance() }
    }
}
