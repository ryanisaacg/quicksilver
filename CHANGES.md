# Changelog

## WIP
- Do not crash if logging is already initalized
- Add optional dependency on serde, when enabled it adds the Serialize and Deserialize traits to Circle, Vector and Rectangle
- Fix a bug where text extents were improperly reported
- [BREAKING] Remove `set_viewport`, `fit_to_surface`, and `fit_to_window`; the viewport is now set automatically
- [BREAKING] Remove `flush(Option<Surface>)` in favor of `flush_surface` and `flush_window`
- [BREAKING] Remove `Graphics::set_projection` in favor of `Graphics::set_view`
- [BREAKING] Remove `transform_for_size` and `projection` from `ResizeHandler`, in favor of `Graphics::set_resize_handler`

## v0.4.0-alpha0.5
- Fix `Timer::remaining` returning the time until next tick, instead of returning how late the tick is.
- Add methods to `Timer`: `late_by`, `period`, and `elapsed`
- [BREAKING] Remove `Scalar` and replace it with `f32`
- [BREAKNNG] Remove all uses of `impl Into<Vector>` in the public API
- Deprecate `Line`, `Triangle`, and a handful of `Shape` methods
- [BREAKING] Automatically re-set the viewport to the size of the window when `present` is called
- Actually add `ResizeHandler` to help with maintaing the same aspect ratio when the screen size changes

## v0.4.0-alpha0.4
- Fix compile issues with font-related features
- [BREAKING] Replace 'lifecycle' module with 'input' module:
    - [BREAKING] Rename `EventStream` to `Input`
    - Integrate the input state cache directly into `Input`
    - [BREAKING] The `blinds::Window` struct and the `Event` enums are now wrapped with methods that use `quicksilver::geom::Vector` instead of `mint::Vector2`
- Implement `From` instead of `Into` for some types

## v0.4.0-alpha0.3
- Update `golem` to `v0.1.1` to fix non-power-of-2 textures
- `impl std::iter::Sum for geom::Vector`
- Implement `std::ops::MulAssign`, `std::ops::AddAssign`, and `std::ops::SubAssign` for `Transform`s.
- Addition and subtraction of Tranforms supported to help with easing function calculations
- Added an example with loading progress bar
- Add `Timer` struct to help timing draw calls and a consistent update cycle
- Add `exhaust` and `reset` function to timer so they can be used for more than just an update cycle
- `lifecycle::run` can now accept any kind of Error.
- Add `into_raw_context` on Graphics, to allow lower-level graphics programming
- Add font support!
  - `VectorFont` allows you to load TTF files via rusttype
  - `FontRenderer` allows you to draw glyphs to the screen

## v0.4.0-alpha0.2
- Fix the "easy-log" feature
- [BREAKING] `fill_circle` now takes a `&Circle` instead of a separate center and radius

## 0.4.0-alpha0.1
- Fix the default blend mode: it should mix colors using their alphas
- [Breaking] Remove support for image types other than jpeg and png
- [Breaking] Add a parameter to `Graphics::flush` which determines what Surface to render to
- Add the Surface API, for rendering to textures
- Add functions to set texture parameters and to set texture data
- Add the ability to set the blend pipeline
- Fix over-estimation of GPU buffer sizes
- Add functions on Graphics to set the viewport (`set_viewport`, `fit_to_surfcae`, `fit_to_window`)
- Add `ResizeHandler` to help with maintaing the same aspect ratio when the screen size changes
- Re-export `blinds`, `golem`, `mint`, and `log`
- [Breaking] Update to blinds 0.1.0:
    - [Breaking] Reworked the events API for forwards-compatibility
    - Set the tab title on web
    - Add an optional module to cache event states

## 0.4.0-alpha0
The API change is *very breaking*. It can be considered nearly a full re-write of Quicksilver.

- Added the new async API, via `blinds`
- Added experimental support for `web-sys` behind the feature named `web-sys`
- [Breaking] Moved `stdweb` support behind the feature named `stdweb`
- [Breaking] Removed the following APIs/integrations, pending re-works (to be added before 0.4):
    - BlendMode
    - Font
    - ImageScaleStrategy
    - ResizeStrategy
    - Sound
    - Surface
    - `lyon` integration
- [Breaking] Removed the following APIs/integrations, (possibly to be added before 0.4);
    - Keyboard
    - Mouse
- [Breaking] Removed the following APIs/integrations permanently:
    - Asset
    - Animation
    - Atlas
    - State
    - combinators module
    - `immi` integration
    - `nalgebra` integration (replaced with `mint` integration)

## 0.3.21
- Fix gamepad buttons mistakenly marked as `Released` instead of `NotPressed`

## 0.3.20
- Fix gamepad Select/Start events misfiring when A is pressed

## 0.3.19
- Update dependencies
- Bump WebGL requirement from 2.0 down to 1.0

## 0.3.18
- Fix the circle fix (was mistakenly applied to triangles)

## 0.3.17
- Add `transform_bounding_box` to `Shape`
- Fix a bug with circles being textured incorrectly

## 0.3.16
- Fix an issue creating non-RGBA images on web
- Deprecate Animation and Immi integration, pending removal in a future release

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
