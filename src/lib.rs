#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(not(target_arch="wasm32"))]
extern crate rand;
#[cfg(not(target_arch="wasm32"))]
extern crate rodio;
#[cfg(not(target_arch="wasm32"))]
extern crate tiled;

mod gl;
pub mod geom;
pub mod graphics;
pub mod input;
#[cfg(not(target_arch="wasm32"))]
pub mod level;
#[cfg(not(target_arch="wasm32"))]
pub mod sound;
mod timer;

pub use self::timer::Timer;
