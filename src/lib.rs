#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;
#[cfg(not(target_arch="wasm32"))]
extern crate tiled;

mod geom;
mod gl;
mod graphics;
#[cfg(not(target_arch="wasm32"))]
mod input;
#[cfg(not(target_arch="wasm32"))]
mod sound;
mod timer;

pub use geom::*;
pub use graphics::*;
#[cfg(not(target_arch="wasm32"))]
pub use input::*;
#[cfg(not(target_arch="wasm32"))]
pub use sound::{Sound, MusicPlayer}; 
pub use timer::Timer;
pub use std::time::Duration;
