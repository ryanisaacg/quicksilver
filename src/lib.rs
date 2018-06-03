//! # quicksilver
//! 
//! [![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
//! [![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)
//! 
//! A 2D game framework written in pure Rust
//! 
//! ## A quick example
//! 
//! ```no_run
//! // Draw some multi-colored geometry to the screen
//! extern crate quicksilver;
//! 
//! use quicksilver::{
//!     State, run,
//!     geom::{Circle, Rectangle, Transform},
//!     graphics::{Color, Draw, Window, WindowBuilder}
//! };
//! 
//! struct DrawGeometry;
//! 
//! impl State for DrawGeometry {
//!     fn new() -> DrawGeometry { DrawGeometry }
//! 
//!    fn draw(&mut self, window: &mut Window) {
//!         window.clear(Color::black());
//!         window.draw(&Draw::rectangle(Rectangle::new(100, 100, 32, 32)).with_color(Color::red()));
//!         window.draw(&Draw::rectangle(Rectangle::new(400, 300, 32, 32)).with_color(Color::blue()).with_transform(Transform::rotate(45)).with_z(10));
//!         window.draw(&Draw::circle(Circle::new(400, 300, 100)).with_color(Color::green()));
//!         window.present();
//!    }
//! }
//! 
//! fn main() {
//!     run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", 800, 600));
//! }
//! ```
//! Run this with `cargo run` or, if you have the wasm32 toolchain installed, you can build for the web 
//! (instructions in the [quicksilver README](https://github.com/ryanisaacg/quicksilver)
//!
//! You should see a red square in the top-left, and a green circle with a blue rectangle inside it 
//! on the bottom-right.
//! 
//! ## Optional Features
//! 
//! Quicksilver by default tries to provide all features a 2D application may need, but not all applications need these features. 
//! The optional features available are 
//! collision support (via [ncollide2d](https://github.com/sebcrozet/ncollide)), 
//! font support (via [rusttype](https://github.com/redox-os/rusttype)), 
//! gamepad support (via [gilrs](https://gitlab.com/gilrs-project/gilrs)), 
//! saving (via [serde_json](https://github.com/serde-rs/json)),
//! and sounds (via [rodio](https://github.com/tomaka/rodio)). 
//! 
//! Each are enabled by default, but you can [specify which features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features) you actually want to use. 

#![doc(html_root_url = "https://docs.rs/quicksilver/0.2.0")]
#![deny(missing_docs)]

extern crate futures;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(not(target_arch="wasm32"))] 
extern crate glutin;
#[cfg(not(target_arch="wasm32"))] 
extern crate image;

#[cfg(target_arch="wasm32")]
#[macro_use]
extern crate stdweb;

#[cfg(feature="alga")]
extern crate alga;
#[cfg(all(feature="gilrs", not(target_arch="wasm32")))] 
extern crate gilrs;
#[cfg(feature="nalgebra")]
extern crate nalgebra;
#[cfg(feature="ncollide2d")]
extern crate ncollide2d;
#[cfg(all(feature="rodio", not(target_arch="wasm32")))] 
extern crate rodio;
#[cfg(feature="rusttype")] 
extern crate rusttype;
#[cfg(feature="serde_json")]
extern crate serde_json;


mod error;
mod file;
mod ffi;
mod state;
mod timer;
pub mod geom;
pub mod graphics;
pub mod input;
#[cfg(feature="saving")]
pub mod saving;
#[cfg(feature="sounds")]
pub mod sound;
pub use file::FileLoader;
pub use error::QuicksilverError;
pub use timer::Timer;
pub use state::{State, run};

/// Necessary types from futures-rs
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

