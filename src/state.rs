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


#[macro_export]
/// A macro that defines the main functions required for native and web using a State typename
macro_rules! run {
    ($Start: tt) => (
        #[doc(hidden)]
        pub struct Application {
            state: $Start, 
            window: Window,
        }

        #[doc(hidden)]
        impl Application {
            pub fn events(&mut self) -> bool {
                self.window.poll_events()
            }

            pub fn update(&mut self) {
                self.state.update(&mut self.window);
            }

            pub fn draw(&mut self) {
                self.state.draw(&mut self.window);
            }
        }


        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn init() -> *mut Application {
            let (window, canvas) = $Start::configure();
            let state = $Start::new();
            Box::into_raw(Box::new(Application { state, window }))
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
            let window = $Start::configure();
            let state = $Start::new();
            let mut app = Application { state, window };
            while app.events() {
                timer.tick(||  { app.update(); Duration::from_millis(16) });
                app.draw();
            }
        }
    )
}

