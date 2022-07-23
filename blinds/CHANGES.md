# Changelog

## v0.2.0
- Always enable gl, removing the need for the `gl` feature
- Remove `run_gl` because all runs are now GL
- Add `get_proc_address` for desktop GL, and `webgl_context` for web GL
- Remove `glow` dependency; use the new context methods to construct glow contexts

## v0.1.6
- Automatically focus the canvas on window creation on web

## v0.1.5
- Fix alpha-blending settings on the web

## v0.1.4
- Add Clone, PartialEq derivations to Settings
- Add Copy, Clone, PartialEq, Eq, Hash to CursorIcon

## v0.1.3
- Fix a bug in ResizedEvent's `logical_size` calculation
- Upgraded winit to version 0.22 (and glutin to 0.24)

## v0.1.2
- Fix logical vs physical coordinates in mouse move events

## v0.1.1
- docs.rs build fix

## v0.1.0
- [Breaking] Reworked the events API for forwards-compatibility
- Set the tab title on web
- Fix building when features are disabled
- Add an optional module to cache event states

## v0.1.0-alpha10
- Fixed a bug where the docs said "logical sizes" but was actually using physical sizes
