//! Creating a blank window is all well and good, but drawing something to it is even better.
//! Rendering in Quicksilver usually takes the form:
//! ```no_run
//! # use quicksilver::{graphics::{Background, Drawable}, lifecycle::Window};
//! # fn func(window: &mut Window, some_drawable: impl Drawable, some_background: Background) {
//! window.draw(&some_drawable, some_background);
//! # }
//! ```
//! `Drawable` is a trait which allows an object to determine how to lay out some points to draw,
//! like a rectangle or a circle. A Background is what to fill those points with, like a solid 
//! color or an image. For example, drawing a red rectangle with a top-left coordinate of (50, 50),
//! a width of 100, and a height of 200 would look like:
//! ```no_run
//! # use quicksilver::{geom::{Rectangle}, graphics::{Background, Color, Drawable}, lifecycle::Window};
//! # fn func(window: &mut Window) {
//! let area = Rectangle::new((50, 50), (100, 200));
//! let background = Background::Col(Color::RED);
//! window.draw(&area, background);
//! # }
//! ```
//! If we wanted to switch out our rectangle for a Circle with a center at (100, 100) and a radius
//! of 50, we could do:
//! ```no_run
//! # use quicksilver::{geom::{Circle, Rectangle}, graphics::{Background, Color, Drawable}, lifecycle::Window};
//! # fn func(window: &mut Window) {
//! let area = Circle::new((100, 100), 50);
//! let background = Background::Col(Color::RED);
//! window.draw(&area, background);
//! # }
//! ```
//! The next step is actually integrating some drawing code into our blank window:
//! ```no_run
//! extern crate quicksilver;
//!
//! use quicksilver::{
//!     Result,
//!     geom::{Rectangle, Vector}, // We'll need to import Rectangle now
//!     graphics::{Background, Color}, // Also Background and Color
//!     lifecycle::{State, Window, run}
//! };
//!
//! struct Screen;
//!
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!         Ok(Screen)
//!     }
//!
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         // Clear the contents of the window to a white background
//!         window.clear(Color::WHITE)?;
//!         // Draw a red rectangle
//!         window.draw(&Rectangle::new((50, 50), (100, 200)), Background::Col(Color::RED));
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
//! We've made two changes from the previous example: imported `Rectangle`, `Background`, and
//! `Color`, as well as implementing the `draw` function. By default, `draw` will be called by the
//! host environment whenever the screen refreshes. First we clear out the window's previous
//! contents, then we draw a red rectangle.
//!
//! If we want the rectangle to be smaller, or bigger, or a different color, the code we wrote is
//! sufficient. Just tweak the input values and you could have a blue Rectangle that's twice as
//! big. But how could we do a rotation, or efficiently do a complex scaling or translation
//! operation? The answer is the `Transform` struct. If you're familiar with matrix math or linear
//! algebra, `Transform` is a 3x3 transformation matrix. If you don't know the underlying math,
//! worry not! There are 4 main ways to create a transform:
//!
//! - `Transform::IDENTITY`: Create a Transform that does nothing. When you apply this transform,
//! everything will look exactly the same
//! - `Transform::rotate(angle)`: Create a Transform that rotates counter-clockwise by a given
//! amount of degrees
//! - `Transform::translate(vector)`: Create a Transform that moves an object by a given vector
//! - `Transform::scale(vector)`: Create a Transform with a given x and y axis scale factor
//!
//! We combine Transform objects using the `*` operator, with the last transform in a chain being
//! applied first. This means that
//! ```no_run
//! # use quicksilver::geom::Transform;
//! Transform::rotate(30) * Transform::translate((0, -6));
//! ```
//! first translates an object up six pixels and then rotates it by 30 degrees.
//!
//! The last drawing concept for now is z-ordering. Sometimes you don't want to draw objects to the
//! screen in the order they're drawn, but with some other sorting method. Here you use z-ordering:
//! an object with a higher z value gets drawn on top of an object with a lower z value.
//!
//! If you want to use a transform or z-ordering, you need to use the more advanced draw function,
//! which takes the form:
//! ```no_run
//! # use quicksilver::{geom::{Transform}, graphics::{Background, Drawable}, lifecycle::Window};
//! # fn func(window: &mut Window, some_drawable: impl Drawable, some_background: Background,
//! # some_transform_value: Transform, some_z_value: f32) {
//! window.draw_ex(&some_drawable, some_background, some_transform_value, some_z_value);
//! # }
//! ```
//! Armed with Transform values, we can turn our little red rectangle into a little red diamond:
//! ```no_run
//! extern crate quicksilver;
//!
//! use quicksilver::{
//!     Result,
//!     geom::{Rectangle, Transform, Vector}, // Now we need Transform
//!     graphics::{Background, Color},
//!     lifecycle::{State, Window, run}
//! };
//!
//! struct Screen;
//!
//! impl State for Screen {
//!     fn new() -> Result<Screen> {
//!         Ok(Screen)
//!     }
//!
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         // Draw a red diamond
//!         window.draw_ex(
//!             &Rectangle::new((50, 50), (50, 50)),
//!             Background::Col(Color::RED),
//!             Transform::rotate(45), // Rotate by 45 degrees
//!             0 // we don't really care about the Z value
//!         );
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     run::<Screen>("Hello World", Vector::new(800, 600), Default::default());
//! }
//! ```
//! Quicksilver gives you a number of `Drawable` objects to work with by default: `Rectangle`,
//! `Vector`, `Circle`, `Line`, and `Triangle`. Most applications will only ever need these, or
//! even just a subset of these, but you can feel free to define your own `Drawable` objects. This
//! is covered later in the `mesh` tutorial.
