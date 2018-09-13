//! When image loading and asset combinators are combined, we can go from loading a simple image
//! and drawing it to the screen to more complex uses of image files, like spritesheets or texture
//! atlases.
//!
//! An `Animation` is a linear series of `Image`s that loops
//! when it completes. You can provide either an iterator of images to `Animation::new` (as well as
//! how many frames Animation ticks) or an image and an iterator of regions to
//! `Animation::from_spritesheet`. Either way, the Animation will have a linear collection of
//! frames. Each time you draw, you should call the `Animation::tick` function so that the animation
//! advances; you can access the current frame of the animation with the
//! `Animation::current_frame` at any time.
//!
//! An `Atlas` is a structure that stores animations and images in a single actual image file. This
//! greatly improves GPU performance by reducing the number of texture unit switches. Quicksilver
//! uses the LibGDX file format to load an `Atlas`, described [on Spine's
//! website.](http://esotericsoftware.com/spine-atlas-format) You can query from an `Atlas` with
//! the `Atlas::get` function which returns an AtlasItem. An AtlasItem is just an enum of Image and
//! Animation.
