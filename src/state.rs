use geom::Vector;
use graphics::{Window, WindowBuilder};
use input::{Event, BUTTON_STATE_LIST, GAMEPAD_AXIS_LIST, GAMEPAD_BUTTON_LIST, KEY_LIST, MOUSE_BUTTON_LIST};

/// The structure responsible for managing the game loop state
pub trait State {
    /// Create a Window to be used in the application
    fn configure() -> WindowBuilder where Self: Sized;
    /// Create the state given the window and canvas
    fn new() -> Self where Self: Sized;
    /// Tick the State forward one frame
    ///
    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS. 
    ///
    /// By default it does nothing
    fn update(&mut self, &mut Window) {}
    /// Process an incoming event
    ///
    /// By default it does nothing
    fn event(&mut self, &Event, &mut Window) {}
    /// Draw the state to the screen
    ///
    /// Will happen as often as possible, only limited by vysnc
    ///
    /// By default it draws a black screen
    fn draw(&mut self, window: &mut Window) {
        use graphics::Color;
        window.clear(Color::black());
        window.present();
    }
}

/// Run the application's game loop
///
/// On desktop platforms, this yields control to a simple game loop controlled by a Timer. On wasm,
/// this yields control to the browser functions setInterval and requestAnimationFrame
pub fn run<T: 'static + State>() {
    run_impl::<T>()
}

#[doc(hidden)]
pub struct Application {
    state: Box<State>, 
    window: Window,
    event_buffer: Vec<Event>
}

impl Application {
    fn update(&mut self) {
        self.state.update(&mut self.window);
    }

    fn draw(&mut self) {
        self.state.draw(&mut self.window);
    }

    fn event(&mut self, event: &Event) {
        self.window.process_event(event);
        self.state.event(event, &mut self.window);
    }

    fn process_events(&mut self) {
        self.window.update_gamepads(&mut self.event_buffer);
        for i in 0..self.event_buffer.len() {
            self.window.process_event(&self.event_buffer[i]);
            self.state.event(&self.event_buffer[i], &mut self.window);
        }
        self.event_buffer.clear();
    }
}

#[cfg(not(target_arch="wasm32"))]
fn run_impl<T: 'static + State>() {
    use input::EventProvider;
    let (window, events_loop) = T::configure().build();
    let mut events = EventProvider::new(events_loop);
    let mut event_buffer = Vec::new();
    let state = Box::new(T::new());
    let mut app = Application { window, state, event_buffer };
    use std::time::Duration;
    use sound::Sound;
    Sound::initialize();
    let mut timer = ::Timer::new();
    let mut running = true;
    while running {
        running = events.generate_events(&mut app.window, &mut app.event_buffer);
        app.process_events();
        timer.tick(||  { 
            app.update(); 
            Duration::from_millis(16) 
        });
        app.draw();
        app.window.clear_temporary_states();
    }
}

#[cfg(target_arch="wasm32")]
fn run_impl<T: 'static + State>() {
    use ffi::wasm;
    use std::os::raw::c_void;
    let app = Box::new(Application { 
        window: T::configure().build(), 
        state: Box::new(T::new()),
        event_buffer: Vec::new()
    });
    unsafe { wasm::set_app(Box::into_raw(app) as *mut c_void) };
}

#[doc(hidden)]
#[no_mangle]
#[cfg(target_arch="wasm32")]
pub extern "C" fn update(app: *mut Application) {
    let mut app = unsafe { Box::from_raw(app) };
    app.process_events();
    app.update();
    Box::into_raw(app);
}

#[doc(hidden)]
#[no_mangle]
#[cfg(target_arch="wasm32")]
pub extern "C" fn draw(app: *mut Application) {
    let mut app = unsafe { Box::from_raw(app) };
    app.draw();
    Box::into_raw(app);
}

#[doc(hidden)]
#[no_mangle]
#[cfg(target_arch="wasm32")]
pub unsafe extern "C" fn event(app: *mut Application, event_tag: u32) {
    use ffi::wasm;
    let mut app = Box::from_raw(app);
    // TODO: Convert u32 to the enums
    let event = match event_tag {
        0 => Event::Closed,
        1 => Event::Focused,
        2 => Event::Unfocused,
        3 => Event::Key(KEY_LIST[wasm::event_data_button() as usize], BUTTON_STATE_LIST[wasm::event_data_state() as usize]),
        4 => Event::MouseMoved(Vector::new(wasm::event_data_f1(), wasm::event_data_f2())),
        5 => Event::MouseEntered,
        6 => Event::MouseExited,
        7 => Event::MouseWheel(Vector::new(wasm::event_data_f1(), wasm::event_data_f2())),
        8 => Event::MouseButton(MOUSE_BUTTON_LIST[wasm::event_data_button() as usize], BUTTON_STATE_LIST[wasm::event_data_state() as usize]),
        9 => Event::GamepadAxis(wasm::event_data_id(), GAMEPAD_AXIS_LIST[wasm::event_data_button() as usize], wasm::event_data_f1()),
        10 => Event::GamepadButton(wasm::event_data_id(), GAMEPAD_BUTTON_LIST[wasm::event_data_button() as usize], BUTTON_STATE_LIST[wasm::event_data_state() as usize]),
        11 => Event::GamepadConnected(wasm::event_data_id()),
        12 => Event::GamepadDisconnected(wasm::event_data_id()),
        _ => {
            Box::into_raw(app);
            return;
        }
    };
    app.event(&event);
    Box::into_raw(app);
}
