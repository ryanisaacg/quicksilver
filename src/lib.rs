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

#[cfg(feature="alga")]
extern crate alga;
#[cfg(feature="futures")] 
extern crate futures;
#[cfg(all(feature="gilrs", not(target_arch="wasm32")))] 
extern crate gilrs;
#[cfg(all(feature="glutin", not(target_arch="wasm32")))] 
extern crate glutin;
#[cfg(all(feature="image", not(target_arch="wasm32")))] 
extern crate image;
#[cfg(feature="nalgebra")]
extern crate nalgebra;
#[cfg(feature="ncollide")]
extern crate ncollide;
#[cfg(feature="rand")] 
extern crate rand;
#[cfg(all(feature="rodio", not(target_arch="wasm32")))] 
extern crate rodio;
#[cfg(feature="rusttype")] 
extern crate rusttype;
#[cfg(feature="serde")] 
extern crate serde;
#[cfg(feature="serde_json")] 
extern crate serde_json;

#[cfg(feature="serde_derive")]
#[macro_use]
extern crate serde_derive;

mod error;
#[cfg(feature="futures")] mod file;
mod ffi;
#[cfg(feature="window")] mod state;
mod timer;
#[cfg(feature="geometry")] pub mod geom;
#[cfg(feature="window")]   pub mod graphics;
#[cfg(feature="window")]   pub mod input;
#[cfg(feature="saving")]   pub mod saving;
#[cfg(feature="sounds")]   pub mod sound;
#[cfg(feature="futures")] pub use file::FileLoader;
pub use error::QuicksilverError;
pub use timer::Timer;
#[cfg(feature="window")]   pub use state::{State, run};
#[cfg(all(feature="window", target_arch="wasm32"))] pub use state::{update, draw, event};

/// Necessary types from futures-rs
#[cfg(feature="futures")] 
pub use futures::{Future, Async};

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
    use std::mem;
    let mut vec = Vec::with_capacity(length);
    let pointer = vec.as_mut_ptr();
    mem::forget(vec);
    pointer
}

