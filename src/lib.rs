//! # quicksilver
//!
//! [![Code
//! Coverage](https://codecov.io/gh/ryanisaacg/quicksilver/branch/master/graph/badge.svg)](https://codecov.io/gh/ryanisaacg/quicksilver)
//! [![Build
//! Status](https://travis-ci.org/ryanisaacg/quicksilver.svg?branch=asset-rework)](https://travis-ci.org/ryanisaacg/quicksilver)
//! [![License](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/ryanisaacg/quicksilver/blob/master/LICENSE)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ryanisaacg/quicksilver/blob/master/LICENSE)
//! [![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
//!
//! A 2D game framework written in pure Rust
//!
//! ## What's included?
//!
//! - 2D geometry: Vectors, Transformation matrices, Rectangles, Circles, Line segments, and a
//! generic Shape abstraction
//! - Keyboard and 3-button mouse support
//! - Viewport projection of the mouse to the world space automatically
//! - Zero-cost camera transformations
//! - OpenGL hardware-accelerated graphics
//! - A variety of image formats
//! - Multi-play sound clips
//! - A looping music player
//! - Asynchronous asset loading
//! - Unified codebase across desktop and the web
//!
//! ## Supported Platforms
//!
//! The engine is supported on Windows, macOS, (somewhat) Linux, and the web via WebAssembly. 
//! Linux is supported inasmuch as the libraries used for graphics (glutin, gl) and sound (rodio)
//! work correctly, 
//! but no extra attempts to support exotic setups will be made. 
//! The web is only supported via the `wasm32-unknown-unknown` Rust target, not through emscripten.
//! It might work with emscripten but this is not an ongoing guarantee.
//!
//! It has not been tested extensively on desktop platforms other than x86, but there is no reason
//! it should fail to work. If the dependencies libraries and the Rust compiler support a platform,
//! quicksilver should as well.
//!
//! There are no plans to support mobile / touch-primary platforms, as the paradigms are completely
//! different. UI elements must be created differently, input is one or two points of contact
//! rather than primarily through a keyboard, etc. 
//!
//! ## Compiler versions
//!
//! The desktop targets should always compile and run on the latest stable rust. 
//! Currently the web target is limited to nightly rust, because the WASM target that does not
//! require emscripten is limited to nightly.

#![deny(missing_docs)]

#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(not(target_arch="wasm32"))]
extern crate rand;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod gl;
pub mod asset;
pub mod geom;
pub mod graphics;
pub mod input;
pub mod saving;
pub mod sound;
mod timer;
pub use self::timer::Timer;

#[no_mangle]
#[cfg(target_arch="wasm32")]
#[doc(hidden)]
pub unsafe extern "C" fn deallocate_cstring(string: *mut i8) {
    use std::ffi::CString;
    CString::from_raw(string);
}

#[no_mangle]
#[cfg(target_arch="wasm32")]
#[doc(hidden)]
pub unsafe extern "C" fn allocate_memory(length: usize) -> *mut i8 {
    Vec::with_capacity(length).as_mut_slice().as_mut_ptr()
}


#[macro_export]
/// A macro that defines the main functions required for native and web
///
/// It takes a typename where the type has `new`, `update`, `draw`, and `events` functions. The
/// `new` function returns an instance of the type. The `events` function should return a `bool`
/// that indicates if the game loop should continue (`true`) or stop (`false`).
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
