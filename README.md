![Quicksilver Logo](./logo.svg)

[![Build Status](https://travis-ci.org/ryanisaacg/quicksilver.svg)](https://travis-ci.org/ryanisaacg/quicksilver)
[![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
[![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)
[![dependency status](https://deps.rs/repo/github/ryanisaacg/quicksilver/status.svg)](https://deps.rs/repo/github/ryanisaacg/quicksilver)
[![Gitter chat](https://badges.gitter.im/quicksilver-rs/Lobby.svg)](https://gitter.im/quicksilver-rs/Lobby?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

A Rust 2D game framework, for both the Web and Desktop

## A quick example

Create a rust project and add this line to your `Cargo.toml` file under `[dependencies]`:

    quicksilver = "*"

Then replace `src/main.rs` with the following (the contents of quicksilver's examples/draw-geometry.rs):

```rust
// Draw some multi-colored geometry to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::{Settings, State, Window, run},
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
    run::<DrawGeometry>("Draw Geometry", Vector::new(800, 600), Settings::default());
}
```

Run this with `cargo run` or, if you have the wasm32 toolchain installed, you can build for the web (instructions below).

## Learning Quicksilver

A good way to get started with Quicksilver is to [read and run the examples](https://github.com/ryanisaacg/quicksilver/tree/master/examples) and go through the tutorial modules [on docs.rs](https://docs.rs/quicksilver). If you have any question, feel free to hop onto Gitter or open an issue.

## Made with Quicksilver

- Documentation / Tutorials
    - [tomassedovic](https://github.com/tomassedovic): [quicksilver-roguelike](https://github.com/tomassedovic/quicksilver-roguelike)
- Games
    - [codec-abc](https://github.com/codec-abc): [RustyVolley](https://github.com/RustyVolley/RustyVolleySrc)
    - [rickyhan](https://github.com/rickyhan): [Kingston Crabfight Simulator](https://github.com/rickyhan/crabs)
    - [robotcaleb](https://github.com/robotcaleb): [Replay](https://robotcaleb.github.io/Replay/)
    - [rsribeiro](https://github.com/rsribeiro/): [Evil Alligator](https://rsribeiro.github.io/website/)

Want to add your project? Feel free to open an issue or PR!

## Building and Deploying a Quicksilver application

Quicksilver should always compile and run on the latest stable version of Rust, for both web and desktop.

Make sure to put all your assets in a top-level folder of your crate called `static/`. *All* Quicksilver file loading-APIs will expect paths that originate in the static folder, so `static/image.png` should be referenced as `image.png`.

### Linux dependencies

On Windows and Mac, all you'll need to build Quicksilver is a recent stable version of `rustc` and `cargo`. A few of Quicksilver's dependencies require Linux packages to build, namely `libudev`, `zlib`, and `alsa`. To install these on Ubuntu or Debian, run the command `sudo apt install libudev-dev zlib1g-dev alsa libasound2-dev`.

### Deploying for desktop

If you're deploying for desktop platforms, build in release mode (`cargo build --release`) 
and copy the executable file produced (found at "target/release/") and any assets you used (image files 
etc) and create an archive (on Windows a zip file, on Unix a tar file). You should be able to distribute
this archive with no problems; if there are any, please open an issue.

### Deploying for the web

If you're deploying for the web, first make sure you've [installed the cargo web tool](https://github.com/koute/cargo-web). Then use the `cargo web deploy` to build your application for distribution (located at `target/deploy`).

If you want to test your application locally, use `cargo web start` and open your favorite browser to the port it provides. 

## Optional Features

Quicksilver by default tries to provide all features a 2D application may need, but not all applications need these features. 
The optional features available are 
collision support (via [ncollide2d](https://github.com/sebcrozet/ncollide)), 
font support (via [rusttype](https://github.com/redox-os/rusttype)), 
gamepad support (via [gilrs](https://gitlab.com/gilrs-project/gilrs)), 
saving (via [serde_json](https://github.com/serde-rs/json)),
complex shape / svg rendering (via [lyon](https://github.com/nical/lyon)),
immediate-mode GUIs (via [immi](https://github.com/tomaka/immi)),
and sounds (via [rodio](https://github.com/tomaka/rodio)). 

Each are enabled by default, but you can [specify which features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features) you actually want to use. 

## Supported Platforms

The engine is supported on Windows, macOS, Linux, and the web via WebAssembly. 
The web is only supported via the `wasm32-unknown-unknown` Rust target, not through emscripten.
It might work with emscripten but this is not an ongoing guarantee.

On desktop it requires OpenGL 3.2; on the web it requires WebGL 2.0.

Mobile support would be a future possibility, but likely only through external contributions.

