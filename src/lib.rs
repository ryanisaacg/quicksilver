#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;
#[cfg(not(target_arch="wasm32"))]
extern crate tiled;

#[cfg(target_arch="wasm32")]
mod bridge;
mod geom;
mod gl;
mod graphics;
mod input;
#[cfg(not(target_arch="wasm32"))]
mod sound;
mod timer;

pub use geom::*;
pub use graphics::*;
pub use input::*;
#[cfg(not(target_arch="wasm32"))]
pub use sound::{Sound, MusicPlayer}; 
pub use timer::Timer;
pub use std::time::Duration;
