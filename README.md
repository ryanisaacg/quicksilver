# quicksilver

[![Code Coverage](https://codecov.io/gh/ryanisaacg/quicksilver/branch/master/graph/badge.svg)](https://codecov.io/gh/ryanisaacg/quicksilver)
[![Build Status](https://travis-ci.org/ryanisaacg/quicksilver.svg?branch=asset-rework)](https://travis-ci.org/ryanisaacg/quicksilver)
[![License](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/ryanisaacg/quicksilver/blob/master/LICENSE)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ryanisaacg/quicksilver/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/quicksilver.svg)](https://crates.io/crates/quicksilver)

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

It has not been tested extensively on desktop platforms other than x86, but there is no reason it should fail to work. If the dependencies libraries and the Rust compiler support a platform, quicksilver should as well.

There are no plans to support mobile / touch-primary platforms, as the paradigms are completely different. UI elements must be created differently, input is one or two points of contact rather than primarily through a keyboard, etc. 

## Compiler versions

The desktop targets should always compile and run on the latest stable rust. 
Currently the web target is limited to nightly rust, because the WASM target that does not require emscripten is limited to nightly.

