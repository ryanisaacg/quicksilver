# Changelog

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
