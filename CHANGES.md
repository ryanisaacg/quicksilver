# Changelog

## In-development

## 0.3.15
- Fix an issue with the viewport in `Surface::render_to()`
- Fix a GL error in `Image::new_null()` for the WebGL backend
- Fix a JavaScript error for WebGL backend in Firefox
- Unbind newly created FBOs and textures to prevent GL errors and unwanted behaviours

## 0.3.14
- Documentation fixes in src/lib.rs

## 0.3.13
- Allow any type that is `Into<Background>` to be passed into `draw` and `draw_ex`
- Update docs for `save` and `load`

## 0.3.12
- Updated glutin to 0.21
- Expand the Event::Typed to include punctuation

## 0.3.11
- Add support for lyon's StrokeTessellator

## 0.3.10
- Updated lyon to 0.13
- Updated gilrs to 0.7, which brings macOS support
- Created a `prelude` module which contains commonly-used imports

## 0.3.9
- Fix web mouse positions to be consistent with native behaviour

## 0.3.8

- Fix web keycodes
- Add more characters to Event::Typed reporting on web
- Add `from_rgba` and `from_hex` functions to create colors
- Fix mouse clicks not registering in Immi

## 0.3.7

- Fix text rendering failing on immi (it's still not very good)
- Render string slices with newlines correctly
- Updated optional dependencies: nalgebra to ^0.17, ncollide2d to ^0.18

## 0.3.6

- Fix a crash in certain font & text combinations
- Add a blend background option for blending images and color

## 0.3.5

- Add a new event: `Typed(char)` that allows reading typed alphanumeric characters
- Add support for touch events on web
- Fix a bug with swapping textures in the WebGL backend
- Add support for cursor style change
- Fix a bug with save / load on WASM
- Add support for saving raw bytes
- Add the ability to use a custom initializer for `State` implementors using `run_with`

## 0.3.4

- Updated stdweb to allow use of stable Rust for web builds
    - This requires cargo web of >= 0.6.23, use `cargo install -f cargo-web` to update
- Add the ability to take screenshots of the window or surface with `Window::screenshot`
- Fix a bug in web key input where any key past F15 would not map correctly

## 0.3.3

- Add a new ResizeStrategy: IntegerScale
- Fix bug in `Rectangle::contains`
- Fix transformed lines not displaying properly
- Update to Rust 2018
- Fix immi draws and mouse pointer location being scaled incorrectly
- Added `ImmiRender::new_with_view`, `ImmiRender::new_with_window`, and `create_immi_ctx`
- Deprecated `ImmiRender::new` in favor of the above
- Added dynamic full screen to the web backend

## 0.3.2

- Add `Image::from_bytes` to load an Image from an encoded byte array
- Fix Transform's impl of PartialEq always returning the wrong result

## 0.3.1

- Add implementing custom drawables to the mesh tutorial
- Mitigate a glutin bug on macOS Mojave that causes content to not be rendered to the window
- Add the ability to close the window programmatically through `Window::close`
- Fix Asset loading bugs on Chromium
- Fix `MouseWheel` event being reported as `MouseMove` on non-wasm platforms.
- Fix alpha blending working incorrectly

## 0.3.0
- Add new methods of initializing fonts `Font::from_slice` and `Font::from_bytes` using byte sequences
- [Breaking] Add more fields to `Error`, and add a Result alias to make error bubbling more convenient
- [Breaking] Rename type `QuicksilverError` to `Error`
- [Breaking] Add `SaveError`, a new error type
- [Breaking] Made the letterbox a configurable color
- [Breaking] Replace the `Draw` struct with a `Drawable` trait that draws to a mesh object
- Added a Result type alias
- [Breaking] Add a Result return to all of the State methods
- [Breaking] Move the Font parameters into their own structure
- [Breaking] Remove the Loader types in favor of a new Asset type
- Implemented `Line` as drawable object
- Implemented `Triangle` as drawable object
- Added `distance` method to `Vector`
- Fixed bug with Windows not scaling the viewport by DPI
- Fixed bug with macOS not letterboxing correctly
- Added an implementation of the immi renderer
- Mark some functions #[must_use]
- Implement `Line` as drawable object
- Implement `Triangle` as drawable object
- Add `distance` method to `Vector`
- [Breaking] Use constants instead of functions for `Vector`s' "presets"
- Add an optional method to `State` to handle error logging
- Re-export the `futures::future` module as the `combinators` module
- [Breaking] Remove `Line::new_sized`
- [Breaking] Take `Vector`s in any function-argument where sensible (positions and sizes)
- [Breaking] Replace the `new` functions by the `newv` functions (`new` takes `Vector`s now)
- Add a conversion from tuples with two `Scalar`s two `Vector`s
- Create a Mesh structure that caches drawing
- Create a Background enum that can either be a color or an image
- Add optional `lyon` integration for vector graphics
- [Breaking] Replace the `Shape` enum and the `Positioned` trait with a `Shape` enum
- [Breaking] Remove the `present` function and automatically switch the buffers after a draw call
- [Breaking] Move `State` and `run` into a new `lifecycle` module
- Add functions to determine the current framerate
- Add the ability to customize the update and draw rate independently
- [Breaking] Use `static` as the directory to place assets for cargo-web compatibility
- [Breaking] Remove `WindowBuilder` and create a new `Settings` struct to replace it
- Add functions to the window to allow changing settings at run-time
- Add the ability to set window icons and favicons
- Added new `Stopwatch` example
- Added configurable `vsync` option to `Settings`
- Add multisampling anti-aliasing as an option
- Dependencies
    - Versions
        - alga: ``0.5 -> 0.6``
        - glutin: ``0.16 -> 0.17``
        - nalgebra: ``0.14 -> 0.15.1``
        - ncollide2d ``0.15 -> 0.16.0``
    - Highlights
        - Sebcrozet added official wasm32 support for nalgebra
        - Added deps.rs badge to readme for a visual indicator

## v0.2.1

- Revert the update to rodio v0.7, which caused compilation issues on some platforms

## v0.2.0 (yanked)

- Add the ability to save state cross-platform, through `save` and `load`
- Created a unified View system for input and camera
- Added the option to preserve pixelization with `ImageScaleStrategy`
- Added render-to-texture support through `Surface`
- Made geometry constructors able to receive i32 and u32 through `Scalar`
- Added support for various drawing options through `BlendMode`
- Added basic animation support through `Animation`
- Added support for Futures-powered Async asset loading
- Added support for texture atlases through `Atlas`
- Created a `QuicksilverError` type
- Added TTF font support through `Font`
- Integrated with the `rand` crate
- Added support for borderless fullscreen to WindowBuilder
- Added a `constrain` function to `Circle`
- Switched from a game loop macro to a `run` function
- Created JS bindings for Rust math operations
- Made the position of a shape a trait through `Positioned`
- Created feature flags to allow users to pick and choose subsystems
- Added gamepad support to `Gamepad`
- Added an input event system through `State::event`
- Integrated with `ncollide` and `nalgebra`
- Create a single unified drawing function
- Removed the `Line` struct
