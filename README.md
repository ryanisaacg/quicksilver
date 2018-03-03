# quicksilver

[![Build Status](https://travis-ci.org/ryanisaacg/quicksilver.svg?branch=asset-rework)](https://travis-ci.org/ryanisaacg/quicksilver)
[![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
[![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)

A 2D game framework written in pure Rust

## What's included?

- 2D geometry: Vectors, Transformation matrices, Rectangles, Circles, Line segments, and a generic Shape abstraction
- Keyboard and 3-button mouse support
- Viewport projection of the mouse to the world space automatically
- Zero-cost camera transformations
- OpenGL hardware-accelerated graphics
- A variety of image formats
- Multi-play sound clips
- A looping music player
- Asynchronous asset loading
- Unified codebase across desktop and the web

## Supported Platforms

The engine is supported on Windows, macOS, (somewhat) Linux, and the web via WebAssembly. 
Linux is supported inasmuch as the libraries used for graphics (glutin, gl) and sound (rodio) work correctly, 
but no extra attempts to support exotic setups will be made. 
The web is only supported via the `wasm32-unknown-unknown` Rust target, not through emscripten.
It might work with emscripten but this is not an ongoing guarantee.

It has not been tested extensively on desktop platforms other than x86, but there is no reason it should fail to work. If the dependencies and the Rust compiler support a platform, quicksilver should as well.

There are no plans to support mobile / touch-primary platforms, as the paradigms are completely different. UI elements must be created differently, input is one or two points of contact rather than primarily through a keyboard, etc. 

## A quick example

Create a rust project and add this line to your `Cargo.toml` file under `[dependencies]`:

    quicksilver = "*"

Then replace `src/main.rs` with the following (the contents of quicksilver's examples/pulsing_circle):

```rust
// Draw a pulsing circle in the middle of the window
extern crate quicksilver;

use quicksilver::{State, run};
use quicksilver::geom::{Circle, Vector, Transform};
use quicksilver::graphics::{Color, DrawCall, Window, WindowBuilder};

struct PulsingCircle {
    step: f32
}

impl State for PulsingCircle {
    fn configure() -> Window {
        WindowBuilder::new().build("Pulsing Circle", 800, 600)
    }

   fn new() -> PulsingCircle { 
       PulsingCircle { step: 0.0 }
   }

   fn update(&mut self, _window: &mut Window) {
       self.step = (self.step + 1.0) % 360.0;
   }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        let scale = Transform::scale(Vector::one() * (1.0 + (self.step.to_radians().sin() / 2.0)));
        window.draw(&[DrawCall::circle(Circle::new(400, 300, 50)).with_color(Color::green()).with_transform(scale)]);
        window.present();
   }
}

fn main() {
    run::<PulsingCircle>();
}
```

Run this with `cargo run` or, if you have the wasm32 toolchain installed, build it for the web with `cargo +nightly build --target wasm32-unknown-unknown`. 
You should see a black screen with a pulsing circle in the middle, and your cursor should not be visible within the window. Try tweaking parameters to see if you can speed up or slow down the growth of the circle.

## Compiler versions

The desktop targets should always compile and run on the latest stable rust. 
Currently the web target is limited to nightly rust, because the WASM target that does not require emscripten is limited to nightly.

