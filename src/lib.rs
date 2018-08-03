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
//!     Result,
//!     geom::{Circle, Rectangle, Transform, Line, Triangle},
//!     graphics::{Background::Col, Color, Window, WindowBuilder},
//!     lifecycle::{State, run},
//! };
//! 
//! struct DrawGeometry;
//! 
//! impl State for DrawGeometry {
//!     fn new() -> Result<DrawGeometry> {
//!         Ok(DrawGeometry)
//!     }
//! 
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
//!         window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
//!         window.draw(&Circle::new((400, 300), 100), Col(Color::GREEN));
//!         window.draw_ex(
//!             &Line::new((50, 80),(600, 450)).with_thickness(2.0),
//!             Col(Color::RED),
//!             Transform::IDENTITY,
//!             5
//!         );
//!         window.draw_ex(
//!             &Triangle::new((500, 50), (450, 100), (650, 150)),
//!             Col(Color::RED),
//!             Transform::rotate(45) * Transform::scale((0.5, 0.5)),
//!             0
//!         );
//!         window.present()
//!     }
//! }
//! 
//! fn main() {
//!     run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", (800, 600)));
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
#![deny(
    bare_trait_objects,
    missing_docs,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

extern crate futures;
extern crate image;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[cfg(not(target_arch = "wasm32"))]
extern crate gl;
#[cfg(not(target_arch = "wasm32"))]
extern crate glutin;

#[cfg(target_arch = "wasm32")]
#[macro_use]
extern crate stdweb;
#[cfg(target_arch = "wasm32")]
extern crate webgl_stdweb;

#[cfg(all(feature = "gilrs", not(target_arch = "wasm32")))]
extern crate gilrs;
#[cfg(feature = "lyon")]
pub extern crate lyon;
#[cfg(feature = "immi")]
extern crate immi;
#[cfg(feature = "nalgebra")]
extern crate nalgebra;
#[cfg(feature = "ncollide2d")]
extern crate ncollide2d;
#[cfg(all(feature = "rodio", not(target_arch = "wasm32")))]
extern crate rodio;
#[cfg(feature = "rusttype")]
extern crate rusttype;
#[cfg(feature = "serde_json")]
extern crate serde_json;

mod error;
mod file;
pub mod geom;
pub mod graphics;
pub mod input;
pub mod lifecycle;
#[cfg(feature = "saving")]
pub mod saving;
#[cfg(feature = "sounds")]
pub mod sound;
pub use error::QuicksilverError as Error;
pub use file::load_file;

/// A Result that returns either success or a Quicksilver Error
pub type Result<T> = ::std::result::Result<T, Error>;
/// Types that represents a "future" computation, used to load assets
pub use futures::{Async, Future};
/// Helpers that allow chaining computations together in a single Future
///
/// This allows one Asset object that contains all of the various resources
/// an application needs to load.
pub use futures::future as combinators;
