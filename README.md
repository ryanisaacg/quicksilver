# quicksilver

[![Build Status](https://travis-ci.org/ryanisaacg/quicksilver.svg)](https://travis-ci.org/ryanisaacg/quicksilver)
[![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
[![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)
[![dependency status](https://deps.rs/repo/github/ryanisaacg/quicksilver/status.svg)](https://deps.rs/repo/github/ryanisaacg/quicksilver)

A 2D game framework written in pure Rust

## A quick example

Create a rust project and add this line to your `Cargo.toml` file under `[dependencies]`:

    quicksilver = "*"

Then replace `src/main.rs` with the following (the contents of quicksilver's examples/draw-geometry.rs):

```rust
// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    run, Result, State,
    geom::{Circle, Rectangle, Transform, Line, Triangle},
    graphics::{Background::Col, Color, Window, WindowBuilder}
};

struct DrawGeometry;

impl State for DrawGeometry {
    fn new() -> Result<DrawGeometry> {
        Ok(DrawGeometry)
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
        window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
        window.draw(&Circle::new((400, 300), 100), Col(Color::GREEN));
        window.draw_ex(
            &Line::new((50, 80),(600, 450)).with_thickness(2.0),
            Col(Color::RED),
            Transform::IDENTITY,
            5
        );
        window.draw_ex(
            &Triangle::new((500, 50), (450, 100), (650, 150)),
            Col(Color::RED),
            Transform::rotate(45) * Transform::scale((0.5, 0.5)),
            0
        );
        Ok(())
    }
}

fn main() {
    run::<DrawGeometry>(WindowBuilder::new("Draw Geometry", (800, 600)));
}
```

Run this with `cargo run` or, if you have the wasm32 toolchain installed, you can build for the web 
(instructions below).

## Deploying a Quicksilver application


### Deploying for desktop

If you're deploying for desktop platforms, build in release mode (`cargo build --release`) 
and copy the executable file produced (found at "target/release/") and any assets you used (image files 
etc) and create an archive (on Windows a zip file, on Unix a tar file). You should be able to distribute
this archive with no problems; if there are problems, please open an issue.

### Deploying for the web

If you're deploying for the web, first make sure you've 
[installed the wasm toolchain](https://www.hellorust.com/news/native-wasm-target.html) 
and the [cargo web tool](https://github.com/koute/cargo-web). Build the 
wasm file and its js bindings (`cargo +nightly web build --target wasm32-unknown-unknown`). Copy the .wasm and .js
files produced (found at "target/wasm32-unknown-unknown/release") and any assets you may have used. Create an HTML file and attach the script with a `script` tag.

If you want to test your application locally, use `cargo +nightly web start --target wasm32-unknown-unknown` and open your favorite browser to the port it provides. 

## Learning Quicksilver

Currently there isn't a book-like tutorial for Quicksilver, but there 

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

- 2D geometry: Vectors, Transformation matrices, Rectangles, Circles, and a generic Shape abstraction
- Keyboard and 3-button mouse support
- Viewport projection of the mouse to the world space automatically
- OpenGL hardware-accelerated graphics
- A variety of image formats
- Multi-play sound clips
- A looping music player
- Asynchronous asset loading
- Unified codebase across desktop and the web
- Collision support (via [ncollide2d](https://github.com/sebcrozet/ncollide)), 
- TTF font support (via [rusttype](https://github.com/redox-os/rusttype)), 
- Gamepad support (via [gilrs](https://gitlab.com/gilrs-project/gilrs)), 
- Saving on web and desktop (via [serde_json](https://github.com/serde-rs/json)),

## Comparison with [ggez](https://github.com/ggez/ggez)

| Quicksilver | GGEZ |
|-|:-:|
| 2D only game development framework | 2D focused game development framework |
| Targets native and web | Targets native, plans to target mobile and web |
| Built on OpenGL and WebGL | Built on gfx-rs |
| Automatic batched drawing | Opt-in batched drawing |
| Sound playback through rodio | Sound playback through rodio |
| Font rendering with rusttype | Font rendering with rusttype |
| Polling-based and event-based input handling | Event / callback based input handling |
| No custom shader support | Custom shader support |
| Pure rust | Dependency on SDL2, with plans to transition to glutin |
| Configurable feature flags | Most features have no flags |

## Compiler versions

The desktop targets should always compile and run on the latest stable rust. 
Currently the web target is limited to nightly rust, because the WASM target that does not require emscripten is limited to nightly.

