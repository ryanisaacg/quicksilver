//! # quicksilver
//! ![Quicksilver Logo](https://raw.github.com/ryanisaacg/quicksilver/master/logo.svg?sanitize=true)
//!
//! [![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
//! [![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)
//! [![dependency status](https://deps.rs/repo/github/ryanisaacg/quicksilver/status.svg)](https://deps.rs/repo/github/ryanisaacg/quicksilver)
//!
//! A simple 2D game framework written in pure Rust, for both the Web and Desktop
//!
//! ## Alpha Notice
//!
//! This version of Quicksilver is currently in a very early alpha! There are still planned changes
//! to the API, some of them breaking. Additionally, major features (like audio support or text, for
//! example) are entirely missing. Use at your own risk! Feedback on alpha-related bugs or the API
//! changes from the 0.3.x API to the new API would be appreciated.
//!
//! ## A quick example
//!
//! Create a rust project and add this line to your `Cargo.toml` file under `[dependencies]`:
//! ```text
//!     quicksilver = "=0.4.0-alpha0"
//! ```
//! Then replace `src/main.rs` with the following (the contents of quicksilver's
//! `examples/01_square.rs`):
//!
//! ```no_run
//! // Example 1: The Square
//! // Open a window, and draw a colored square in it
//! use mint::Vector2;
//! use quicksilver::{
//!     geom::{Rectangle, Vector},
//!     graphics::{Color, Graphics},
//!     lifecycle::{run, EventStream, Settings, Window},
//!     Result,
//! };
//!
//! fn main() {
//!     run(
//!         Settings {
//!             size: Vector2 { x: 800.0, y: 600.0 },
//!             title: "Square Example",
//!             ..Settings::default()
//!         },
//!         app,
//!     );
//! }
//!
//! async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
//!     // Clear the screen to a blank, white color
//!     gfx.clear(Color::WHITE);
//!     // Paint a blue square with a red outline in the center of our screen
//!     // It should have a top-left of (350, 100) and a size of (150, 100)
//!     let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
//!     gfx.fill_rect(&rect, Color::BLUE);
//!     gfx.stroke_rect(&rect, Color::RED);
//!     // Send the data to be drawn
//!     gfx.present(&window)?;
//!     loop {
//!         while let Some(_) = events.next_event().await {}
//!     }
//! }
//! ```
//!
//! Run this with `cargo run` or, if you have the wasm32 toolchain installed, you can build for the
//! web (instructions below).
//!
//! ## Learning Quicksilver
//!
//! A good way to get started with Quicksilver is to
//! [read and run the examples](https://github.com/ryanisaacg/quicksilver/tree/master/examples)
//! which also serve as tutorials. IF you have any questions, feel free to open an issue or ask for
//! help in the [Rust Community Discord](https://discord.gg/aVESxV8) from other Quicksilver users
//! and developers.
//!
//! ## Building and Deploying a Quicksilver application
//!
//! Quicksilver should always compile and run on the latest stable version of Rust, for both web and
//! desktop.
//!
//! Make sure to put all your assets in a top-level folder of your crate called `static/`. *All*
//! Quicksilver file loading-APIs will expect paths that originate in the static folder, so
//! `static/image.png` should be referenced as `image.png`.
//!
//! ### Linux dependencies
//!
//! On Windows and Mac, all you'll need to build Quicksilver is a recent stable version of `rustc`
//! and `cargo`. A few of Quicksilver's dependencies require Linux packages to build, namely
//! `libudev`, `zlib`, and `alsa`. To install these on Ubuntu or Debian, run the command
//! `sudo apt install libudev-dev zlib1g-dev alsa libasound2-dev`.
//!
//! ### Deploying for desktop
//!
//! If you're deploying for desktop platforms, build in release mode (`cargo build --release`)
//! and copy the executable file produced (found at "target/release/") and any assets you used
//! (image files, etc.) and create an archive (on Windows a zip file, on Unix a tar file). You
//! should be able to distribute this archive with no problems; if there are any, please open an
//! issue.
//!
//! ### Deploying for the web
//!
//! If you're deploying for the web, first make sure you've
//! [installed the cargo web tool](https://github.com/koute/cargo-web). Then use `cargo web deploy`
//! to build your application for distribution (located at `target/deploy`).
//!
//! If you want to test your application locally, use `cargo web start --features stdweb` and open your
//! favorite browser to the port it provides.
//!
//! #### wasm-bindgen support
//!
//! Quicksilver has recently gained experimental support for `wasm-bindgen`, under the `web-sys`
//! feature. The workflow is not currently documented here, but it should be the same as any other
//! library.
//!
//! ## Optional Features
//!
//! Quicksilver by default tries to provide all features a 2D application may need, but not all
//! applications need these features.
//!
//! The optional features available are:
//! - easy logging (via [log](https://github.com/rust-lang/log),
//! [simple_logger](https://github.com/borntyping/rust-simple_logger), and
//! [web_logger](https://github.com/yewstack/web_logger))
//! - gamepad event generation (via [gilrs](https://gitlab.com/gilrs-project/gilrs))
//! - saving (via [gestalt](https://github.com/ryanisaacg/golem))
//!
//! Each are enabled by default, but you can
//! [specify which features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features)
//! you actually want to use.
//!
//! ## Supported Platforms
//!
//! The engine is supported on Windows, macOS, Linux, and the web via WebAssembly.
//!
//! Mobile support would be a future possibility, but likely only through external contributions.

#![deny(
    bare_trait_objects,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
#![allow(clippy::needless_doctest_main)]

// Re-export every library that appears in the public API
pub use blinds;
pub use golem;
pub use log;
pub use mint;

mod error;

pub mod geom;
pub mod graphics;
pub mod lifecycle;
#[cfg(feature = "saving")]
pub mod saving {
    //! A module to manage cross-platform save data via the [`gestalt`] library
    pub use gestalt::*;
}
pub use crate::error::QuicksilverError;

mod timer;
pub use timer::Timer;

/// Load a file as a [`Future`]
///
/// Within an `async` function (like the one passed to [`run`]), you can use `.await`:
///
/// ```no_run
/// # use platter::load_file;
/// # async fn test() {
/// load_file("my_file_path").await.expect("The file was not found!");
/// # }
/// ```
///
/// [`Future`]: std::future::Future
pub use platter::load_file;

/// A Result that returns either success or a [`QuicksilverError`]
pub type Result<T> = std::result::Result<T, QuicksilverError>;
