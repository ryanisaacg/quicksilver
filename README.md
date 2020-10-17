# quicksilver
![Quicksilver Logo](./logo.svg)

[![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)
[![Docs Status](https://docs.rs/quicksilver/badge.svg)](https://docs.rs/quicksilver)
[![dependency status](https://deps.rs/repo/github/ryanisaacg/quicksilver/status.svg)](https://deps.rs/repo/github/ryanisaacg/quicksilver)

A simple 2D game framework written in pure Rust, for both the Web and Desktop

## Maintenance Status

I've [posted an update on my website](https://ryanisaacg.com/posts/quicksilver-goodbye) about Quicksilver.
To keep a long story short: **Quicksilver is no longer actively developed.**
For now I will continue to triage bugs and pull requests and (maybe) fix small bugs.


## Alpha Notice

This version of Quicksilver is currently working its way through alpha! There is still work to do
on the API and on bugfixes, as well as waiting on an upstream library for audio support.
Please feel free to use this version and **provide feedback!** If you run into bugs or want to
give feedback on API decisions, please open an issue.

## A quick example

Create a rust project and add this line to your `Cargo.toml` file under `[dependencies]`:
```text
quicksilver = "0.4"
```
Then replace `src/main.rs` with the following (the contents of quicksilver's
`examples/01_square.rs`):

```rust
// Example 1: The Square
// Open a window, and draw a colored square in it
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);
    // Paint a blue square with a red outline in the center of our screen
    // It should have a top-left of (350, 100) and a size of (150, 100)
    let rect = Rectangle::new(Vector::new(350.0, 100.0), Vector::new(100.0, 100.0));
    gfx.fill_rect(&rect, Color::BLUE);
    gfx.stroke_rect(&rect, Color::RED);
    // Send the data to be drawn
    gfx.present(&window)?;
    loop {
        while let Some(_) = input.next_event().await {}
    }
}
```

## Learning Quicksilver

A good way to get started with Quicksilver is to
[read and run the examples](https://github.com/ryanisaacg/quicksilver/tree/master/examples)
which also serve as tutorials. If you have any questions, feel free to open an issue or ask for
help in the [Rust Community Discord](https://discord.gg/aVESxV8) from other
Quicksilver users and developers.

## Made with Quicksilver

### Version 0.4

- Libraries
    - [Lenscas](https://github.com/lenscas): [Silver_Animation](https://crates.io/crates/silver_animation) - An animation system
    - [Lenscas](https://github.com/lenscas): [Mergui](https://crates.io/crates/mergui) - A simple GUI system
    - [johnpmayer](https://github.com/johnpmayer): [quicksilver-utils-async](https://crates.io/crates/quicksilver-utils-async) - Tasks, timers, and net code
    - [johnpmayer](https://github.com/johnpmayer): [quicksilver-utils-ecs](https://crates.io/crates/quicksilver-utils-ecs) - Entity Component System integrations
- Games
    - [alec-deason](https://github.com/alec-deason): [Pixel Imperfect](https://ntoheuns.itch.io/pixel-imperfect)

### Version 0.3

- Documentation / Tutorials
    - [tomassedovic](https://github.com/tomassedovic): [quicksilver-roguelike](https://github.com/tomassedovic/quicksilver-roguelike)
- Games
    - [WushuWorks](https://github.com/WushuWorks): [I am the Elder God](https://wushuworks.github.io/I-am-the-Elder-God/)
    - [codec-abc](https://github.com/codec-abc): [RustyVolley](https://github.com/RustyVolley/RustyVolleySrc)
    - [rickyhan](https://github.com/rickyhan): [Kingston Crabfight Simulator](https://github.com/rickyhan/crabs)
    - [robotcaleb](https://github.com/robotcaleb): [Replay](https://robotcaleb.github.io/Replay/)
    - [rsribeiro](https://github.com/rsribeiro/): [Evil Alligator](https://rsribeiro.github.io/website/)
    - [nycex](https://gitlab.com/nycex): [Axosnake](https://gitlab.com/nycex/axosnake)
    - [Leinnan](https://github.com/Leinnan): [Slavic Castles](https://github.com/Leinnan/slavic_castles)
    - [Lenscas](https://github.com/lenscas): [Arena keeper](https://github.com/lenscas/arena_keeper_quick)
- Libraries
    - [Lenscas](https://github.com/lenscas): [Mergui](https://crates.io/crates/mergui) - A simple GUI system

Want to add your project? Feel free to open an issue or PR!

## Building and Deploying a Quicksilver application

Quicksilver should always compile and run on the latest stable version of Rust, for both web and
desktop.

Make sure to put all your assets in a top-level folder of your crate called `static/`. *All*
Quicksilver file loading-APIs will expect paths that originate in the static folder, so
`static/image.png` should be referenced as `image.png`.

### Linux dependencies

On Windows and Mac, all you'll need to build Quicksilver is a recent stable version of `rustc`
and `cargo`. A few of Quicksilver's dependencies require Linux packages to build, namely
`libudev`, `zlib`, and `alsa`. To install these on Ubuntu or Debian, run the command
`sudo apt install libudev-dev zlib1g-dev alsa libasound2-dev`.

### Deploying for desktop

If you're deploying for desktop platforms, build in release mode (`cargo build --release`)
and copy the executable file produced (found at "target/release/") and any assets you used
(image files, etc.) and create an archive (on Windows a zip file, on Unix a tar file). You
should be able to distribute this archive with no problems; if there are any, please open an
issue.

### Deploying for the web

If you're deploying for the web, first make sure you've
[installed the cargo web tool](https://github.com/koute/cargo-web). Then use `cargo web deploy`
to build your application for distribution (located at `target/deploy`).

If you want to test your application locally, use `cargo web start --features quicksilver/stdweb` and open your
favorite browser to the port it provides.

#### wasm-bindgen support

Quicksilver has recently gained experimental support for `wasm-bindgen`, under the `web-sys`
feature. The workflow is not currently documented here, but it should be the same as using any other
library with `wasm-bindgen`.

## Optional Features

Quicksilver by default tries to provide all features a 2D application may need, but not all
applications need these features.

The optional features available are:
- easy logging (via [log](https://github.com/rust-lang/log),
[simple_logger](https://github.com/borntyping/rust-simple_logger), and
[web_logger](https://github.com/yewstack/web_logger))
- gamepad event generation (via [gilrs](https://gitlab.com/gilrs-project/gilrs))
- saving (via [gestalt](https://github.com/ryanisaacg/gestalt))
- font rendering (via [elefont](https://github.com/ryanisaacg/elefont)) and TTF parsing (via [rusttype](https://gitlab.redox-os.org/redox-os/rusttype))

Each are enabled by default, but you can
[specify which features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features)
you actually want to use.

## Supported Platforms

The engine is supported on Windows, macOS, Linux, and the web via WebAssembly.

Mobile support would be a future possibility, but likely only through external contributions.

