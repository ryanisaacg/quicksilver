
use graphics::{Canvas, Window};

/// The first Screen in the application
///
/// It is responsible for creating and configuring the window
pub trait InitialScreen: Screen {
    /// Create a Window and a Canvas to be used in the application
    fn configure() -> (Window, Canvas);

    /// Create the screen
    fn new() -> Self;
}

/// A screen in an application meant to be used with screens_loop!
///
/// Screens are responsible for managing their own internal state
pub trait Screen {
    /// Tick the internal state of the Screen
    ///
    /// If a Screen is returned, control flow will switch to it and the current screen will be
    /// discarded
    fn update(&mut self, window: &mut Window, canvas: &mut Canvas) -> Option<Box<Screen>>;

    /// Draw the screen to the window
    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas);
}


#[doc(hidden)]
pub struct Application {
    pub screen: Box<Screen>,
    pub window: Window,
    pub canvas: Canvas
}

#[doc(hidden)]
impl Application {
    pub fn events(&mut self) -> bool {
        self.window.poll_events()
    }

    pub fn update(&mut self) {
        let result = self.screen.update(&mut self.window, &mut self.canvas);
        if let Some(screen) = result {
            self.screen = screen;
        }
    }

    pub fn draw(&mut self) {
        self.screen.draw(&mut self.window, &mut self.canvas);
    }
}

#[macro_export]
/// A macro that defines the main functions required for native and web using Screens
///
/// It takes a typename where the type implements the Screen trait and has a 'new' function and runs the event loop.
macro_rules! screens_loop {
    ($Start: tt) => (
        use quicksilver::Application;

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn init() -> *mut Application {
            let (window, canvas) = $Start::configure();
            let screen = Box::new($Start::new());
            Box::into_raw(Box::new(Application { screen, window, canvas }))
        }

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn update(app: *mut Application) -> u32 {
            let mut app = unsafe { Box::from_raw(app) };
            app.events();
            app.update();
            Box::into_raw(app);
            ::std::time::Duration::from_millis(16).subsec_nanos() / 1000000
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
            let screen = Box::new($Start::new());
            let mut app = Application { screen, window, canvas };
            while app.events() {
                timer.tick(||  { app.update(); Duration::from_millis(16) });
                app.draw();
            }
        }
    )
}

