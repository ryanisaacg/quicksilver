use graphics::Window;

/// The structure responsible for managing the game loop state
pub trait State {
    /// Create a Window to be used in the application
    fn configure() -> Window where Self: Sized;
    /// Create the state given the window and canvas
    fn new() -> Self where Self: Sized;
    /// Tick the State forward one frame
    ///
    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS. 
    ///
    /// By default it does nothing
    fn update(&mut self, &mut Window) {}
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
}

impl Application {
    fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    fn update(&mut self) {
        self.state.update(&mut self.window);
    }

    fn draw(&mut self) {
        self.state.draw(&mut self.window);
    }
}

#[cfg(not(target_arch="wasm32"))]
fn run_impl<T: 'static + State>() {
    let window = T::configure();
    let state = Box::new(T::new());
    let mut app = Application { window, state };
    use std::time::Duration;
    use sound::Sound;
    Sound::initialize();
    let mut timer = ::Timer::new();
    while app.events() {
        timer.tick(||  { 
            app.update(); 
            Duration::from_millis(16) 
        });
        app.draw();
    }
}

#[cfg(target_arch="wasm32")]
fn run_impl<T: 'static + State>() {
    use ffi::wasm;
    let window_init = Box::new(T::configure);
    let state_init = Box::new(|| Box::new(T::new()) as Box<State>);
    unsafe { wasm::set_init(Box::into_raw(window_init), Box::into_raw(state_init)) };
}

#[doc(hidden)]
#[no_mangle]
#[cfg(target_arch="wasm32")]
pub extern "C" fn init(window_init: *mut FnMut() -> Window, state_init: *mut FnMut() -> Box<State>) -> *mut Application {
    let mut window_init = unsafe { Box::from_raw(window_init) };
    let mut state_init = unsafe { Box::from_raw(state_init) };
    let app = Box::new(Application { window: window_init(), state: state_init() });
    Box::into_raw(app)
}

#[doc(hidden)]
#[no_mangle]
#[cfg(target_arch="wasm32")]
pub extern "C" fn update(app: *mut Application) {
    let mut app = unsafe { Box::from_raw(app) };
    app.events();
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

