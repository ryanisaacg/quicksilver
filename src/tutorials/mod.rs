//! The quicksilver tutorials, generated through Rustdoc
//!
//! While this isn't a traditional way of hosting a tutorial, Rustdoc ensures that all the code in
//! the tutorials is checked when CI runs, keeping it nice and up-to-date.
//!
//! Before you jump into the tutorials below, make sure your development environment is ready. If
//! you're just targeting desktop, all you need is the latest stable Rust. If you're targeting the
//! web, first make sure you have a nightly toolchain installed (`rustup update nightly`), and the 
//! wasm target installed on nightly (`rustup target add wasm32-unknown-unknown --toolchain
//! nightly`.) Once that's done, install cargo-web (`cargo +nightly install -f cargo-web`) and you
//! should be good.

pub mod basic;
pub mod drawing;
pub mod input;
pub mod lifecycle;
pub mod images;
pub mod font;
pub mod sound;
pub mod asset_combinators;
pub mod multi_screen;
pub mod mesh;
pub mod ncollide_integration;
pub mod lyon_integration;
pub mod futures_integration;
