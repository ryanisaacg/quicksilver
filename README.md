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

Then replace `src/main.rs` with the following (the contents of quicksilver's example/pulsing_circle):


    #[macro_use]
    extern crate quicksilver;

    use quicksilver::geom::{Circle, Transform, Vector};
    use quicksilver::graphics::{Canvas, Color, Window, WindowBuilder};
    use std::time::Duration;

    pub struct State {
        window: Window,
        canvas: Canvas,
        scale: Vector
    }

    impl State {
        pub fn new() -> State {
            let (window, canvas) = WindowBuilder::new()
                .with_show_cursor(false)
                .build("Circle", 800, 600);
            let scale = Vector::one();
            State { window, canvas, scale }
        }

        pub fn events(&mut self) -> bool {
            self.window.poll_events()
        }

        pub fn update(&mut self) -> Duration {
            self.scale = self.scale.normalize() * ((self.scale.len() + 0.05) % 1.0 + 1.0);
            Duration::from_millis(16)
        }

        pub fn draw(&mut self) {
            self.canvas.clear(Color::black());
            self.canvas.draw_circle_trans(Circle::newi(400, 300, 50), Color::white(), Transform::scale(self.scale));
            self.canvas.present(&self.window);
        }
    }

    game_loop!(State);

Run this with `cargo run` or, if you have the wasm32 toolchain installed, build it for the web with `cargo +nightly build --target wasm32-unknown-unknown`. 
You should see a black screen with a pulsing circle in the middle, and your cursor should not be visible within the window. Try tweaking parameters to see if you can speed up or slow down the growth of the circle.

## Compiler versions

The desktop targets should always compile and run on the latest stable rust. 
Currently the web target is limited to nightly rust, because the WASM target that does not require emscripten is limited to nightly.

