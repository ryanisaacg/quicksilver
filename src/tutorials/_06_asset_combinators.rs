//! In the last tutorial, we briefly touched on asset combinators. Asset combinators are a concept
//! ripped directly from Futures in Rust. While sometimes you want to wait for a simple future to
//! elapse (like loading a single image) sometimes you want something more, like loading 2 images,
//! loading an image then taking several subimages for it, or loading a text file then using its
//! contents to load an image. The last example could be written:
//! ```no_run
//! use quicksilver::{
//!     Future, Result,
//!     combinators::result,
//!     graphics::Image,
//!     lifecycle::Asset,
//!     load_file,
//! };
//! use std::str::from_utf8;
//! fn load_image_from_file(filename: &str) -> Asset<Image> {
//!     Asset::new(load_file(filename)
//!         .and_then(|contents| result(from_utf8(contents)))
//!         .and_then(|image_path| Image::load(image_path)))
//! }
//! ```text
//! This example uses 2 combinators: `result` and `and_then`. `result` takes a Result type and
//! converts it into a Future that immediately resolves. `and_then` chains a Future onto the
//! previous one, if the previous one completes. If we were to re-write the Futures code here as a
//! description, it would look something like:
//! ```text
//! - First load the file at path filename
//! - When that completes, if it succeeded, convert it to UTF8
//! - When that completes, if it succeeded, start loading an image pointed to by the file
//! ```
//! The combinator module is re-exported from the `future` module of the `futures` crate, 
//!
//! **Note: Do not use the `wait` method on a `Future`; on WASM, it will panic. In the near future,
//! [when Rust has async / await](https://github.com/rust-lang/rust/issues/50547), await can be
//! used in place of `wait`.
