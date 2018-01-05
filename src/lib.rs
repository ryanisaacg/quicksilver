#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(not(target_arch="wasm32"))]
extern crate rand;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;
#[cfg(not(target_arch="wasm32"))]
extern crate tiled;

mod gl;
pub mod asset;
pub mod geom;
pub mod graphics;
pub mod input;
#[cfg(not(target_arch="wasm32"))]
pub mod sound;
mod timer;

pub use self::timer::Timer;

#[macro_export]
macro_rules! game_loop {
    ($state: tt) => (
        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn init() -> *mut $state {
            Box::into_raw(Box::new($state::new()))
        }

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn update(state: *mut $state) -> u32 {
            let mut state = unsafe { Box::from_raw(state) };
            state.events();
            let duration = state.update();
            Box::into_raw(state);
            duration.subsec_nanos() / 1000000
        }

        #[no_mangle]
        #[cfg(target_arch="wasm32")]
        pub extern "C" fn draw(state: *mut $state) {
            let mut state = unsafe { Box::from_raw(state) };
            state.draw();
            Box::into_raw(state);
        }
        
        #[cfg(target_arch="wasm32")]
        fn main() {}

        #[cfg(not(target_arch="wasm32"))]
        fn main() {
            let mut timer = quicksilver::Timer::new();
            let mut state = $state::new();
            while state.events() {
                timer.tick(|| state.update());
                state.draw();
            }
        }
    )
}
