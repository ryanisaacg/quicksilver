//! Our first tutorial is simple: Just create a blank window.
//!
//! Here's the full source:
//! 
//! ```no_run
//! extern crate quicksilver;
//! 
//! use quicksilver::{
//!     Result,
//!     geom::Vector,
//!     lifecycle::{State, run}
//! };
//! 
//! struct Screen;
//! 
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!         Ok(Screen)
//!     }
//! }
//! 
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
//! Let's start with importing what we need from Quicksilver:
//! ```no_run
//! extern crate quicksilver;
//! 
//! use quicksilver::{
//!     Result,
//!     geom::Vector,
//!     lifecycle::{State, run}
//! };
//! ```
//! 
//! Quicksilver has its own `Result` type, which is just the same as `std::result::Result<T, quicksilver::Error>`.
//! We use `Vector` for anything 2-dimensional: position, speed, or size, for example.
//! `State` is the trait that defines how we handle the core loop of Quicksilver
//! `run` is the function that kicks off the core loop.
//! 
//! Next we declare our `State` handler:
//! ```no_run
//! struct Screen;
//! ```
//! It's a unit struct (a struct with no fields) because we don't need to store anything.
//! 
//! Now we implement `State` for our handler:
//! 
//! ```no_run
//! # use quicksilver::{
//! #    Result,
//! #    geom::Vector,
//! #    lifecycle::{State, run}
//! # };
//! # struct Screen;
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!          Ok(Screen)
//!     }
//! }
//! ```
//! All we have to do is implement the `new` function, and Quicksilver will take care of all the other functions.
//! The other functions we could override are `draw`, `update`, and `event`, which will be covered in later tutorials.
//! ```no_run
//! # use quicksilver::{
//! #    Result,
//! #    geom::Vector,
//! #    lifecycle::{State, run}
//! # };
//! # struct Screen;
//! # impl State for Screen {
//! #    fn new() -> Result<Screen> {
//! #         Ok(Screen)
//! #    }
//! # }
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
//! Lastly, we create a main that calls `run`, starting the event loop and showing our window
