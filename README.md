# quicksilver

[![Build Status](https://travis-ci.org/ryanisaacg/quicksilver.svg?branch=asset-rework)](https://travis-ci.org/ryanisaacg/quicksilver)
[![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
[![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)

A 2D game framework written in pure Rust

## A quick example

Create a rust project and add this line to your `Cargo.toml` file under `[dependencies]`:

    quicksilver = "*"

Then replace `src/main.rs` with the following (the contents of quicksilver's examples/draw-geometry.rs):

```rust
// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    State, run,
    geom::{Circle, Rectangle, Transform},
    graphics::{Color, Draw, Window, WindowBuilder}
};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> DrawGeometry { DrawGeometry }

   fn draw(&mut self, window: &mut Window) {
        window.clear(Color::black());
        window.draw(&Draw::rectangle(Rectangle::new(100, 100, 32, 32)).with_color(Color::red()));
        window.draw(&Draw::rectangle(Rectangle::new(400, 300, 32, 32)).with_color(Color::blue()).with_transform(Transform::rotate(45)).with_z(10));
        window.draw(&Draw::circle(Circle::new(400, 300, 100)).with_color(Color::green()));
        window.present();
   }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", 800, 600));
}
```

Run this with `cargo run` or, if you have the wasm32 toolchain installed, build it for the web with `cargo +nightly build --target wasm32-unknown-unknown`. 
You should see a black screen with a pulsing circle in the middle, and your cursor should not be visible within the window. Try tweaking parameters to see if you can speed up or slow down the growth of the circle.

## Optional Features

Quicksilver by default tries to provide all features a 2D application may need, but not all applications need these features. 
The optional features available are 
collision support (via [ncollide2d](https://github.com/sebcrozet/ncollide)), 
font support (via [rusttype](https://github.com/redox-os/rusttype)), 
gamepad support (via [gilrs](https://gitlab.com/gilrs-project/gilrs)), 
saving (via [serde_json](https://github.com/serde-rs/json)),
and sounds (via [rodio](https://github.com/tomaka/rodio)). 

Each are enabled by default, but you can [specify which features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features) you actually want to use. 

## Supported Platforms

The engine is supported on Windows, macOS, (somewhat) Linux, and the web via WebAssembly. 
Linux is supported inasmuch as the libraries used for graphics (glutin, gl) and sound (rodio) work correctly, 
but no extra attempts to support exotic setups will be made. 
The web is only supported via the `wasm32-unknown-unknown` Rust target, not through emscripten.
It might work with emscripten but this is not an ongoing guarantee.

It has not been tested extensively on desktop platforms other than x86, but there is no reason it should fail to work. If the dependencies and the Rust compiler support a platform, quicksilver should as well.

There are no plans to support mobile / touch-primary platforms, as the paradigms are completely different. UI elements must be created differently, input is one or two points of contact rather than primarily through a keyboard, etc. 

There is one exception: macOS does not currently support gamepads, see [gilrs-core issue #1](https://gitlab.com/gilrs-project/gilrs-core/issues/1)

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

## Comparison with [ggez](https://github.com/ggez/ggez)

| Quicksilver | GGEZ |
|-|:-:|
| 2D only game development framework | 2D focused game development framework |
| Targets native and web | Targets native, plans to target mobile and web |
| Built on OpenGL and WebGL | Built on gfx-rs |
| Sound playback through rodio | Sound playback through rodio |
| Font rendering with rusttype | Font rendering with rusttype |
| Polling-based input handling | Event / callback based input handling |
| No custom shader support | Custom shader support |
| Pure rust | Dependency on SDL2, with plans to transition to glutin |
| Configurable feature flags | Most features have no flags |

## Compiler versions

The desktop targets should always compile and run on the latest stable rust. 
Currently the web target is limited to nightly rust, because the WASM target that does not require emscripten is limited to nightly.

