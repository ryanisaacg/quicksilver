# Changelog

## In-development
- Add new methods of initializing fonts `Font::from_slice` and `Font::from_bytes` using byte sequences
- [Breaking] Add more fields to `Error`, and add a Result alias to make error bubbling more convenient
- [Breaking] Rename type `Draw` to `Sprite` (and updated the readme accordingly)
- [Breaking] Rename type `QuicksilverError` to `Error`
- [Breaking] Add `SaveError`, a new error type
- [Breaking] Made the letterbox a configurable color
- Added a Result type alias
- [Breaking] Add a Result return to all of the State methods
- [Breaking] Move the Font parameters into their own structure
- [Breaking] Split the Sprite into Drawable and DrawAttributes objects, with convenient function overloads
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
