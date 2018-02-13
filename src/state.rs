use graphics::{Canvas, Window};

/// The structure responsible for managing the game loop state
pub trait State {
    /// Create a Window and a Canvas to be used in the application
    fn configure() -> (Window, Canvas) where Self: Sized;
    /// Create the state given the window and canvas
    fn new() -> Self where Self: Sized;
    /// Tick the State forward one frame
    ///
    /// Will happen at a fixed rate of 60 ticks per second under ideal conditions. Under non-ideal conditions,
    /// the game loop will do its best to still call the update at about 60 TPS. 
    fn update(&mut self, window: &mut Window, canvas: &mut Canvas);
    /// Draw the state to the screen
    ///
    /// Will happen as often as possible, only limited by vysnc
    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas);
}

#[doc(hidden)]
pub struct Application {
    pub state: Box<State>,
    pub window: Window,
    pub canvas: Canvas
}

#[doc(hidden)]
impl Application {
    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) {
        self.state.update(&mut self.window, &mut self.canvas);
    }

    pub fn draw(&mut self) {
        self.state.draw(&mut self.window, &mut self.canvas);
    }
}


#[macro_export]
/// A macro that defines the main functions required for native and web using a State typename
macro_rules! run {
    ($Start: tt) => (
        use quicksilver::Application;

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn init() -> *mut Application {
            let (window, canvas) = $Start::configure();
            let state = Box::new($Start::new());
            Box::into_raw(Box::new(Application { state, window, canvas }))
        }

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn update(app: *mut Application) {
            let mut app = unsafe { Box::from_raw(app) };
            app.events();
            app.update();
            Box::into_raw(app);
        }

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn draw(app: *mut Application) {
            let mut app = unsafe { Box::from_raw(app) };
            app.draw();
            Box::into_raw(app);
        }
        
        #[cfg(target_arch="wasm32")]
        fn main() {}

        #[cfg(not(target_arch="wasm32"))]
        fn main() {
            use std::time::Duration;
            quicksilver::initialize_sound();
            let mut timer = quicksilver::Timer::new();
            let (window, canvas) = $Start::configure();
            let state = Box::new($Start::new());
            let mut app = Application { state, window, canvas };
            while app.events() {
                timer.tick(||  { app.update(); Duration::from_millis(16) });
                app.draw();
            }
        }
    )
}

