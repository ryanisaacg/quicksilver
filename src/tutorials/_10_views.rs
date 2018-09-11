//! Quicksilver uses the [`View`](quicksilver::graphics::View) structure as an abstraction for both
//! graphical and input projection. This means that a view can be thought of like a camera: it
//! determines what coordinates in draw calls appear where on screen, as well as the relationship
//! between the mouse location on the screen and the reported coordinates.
//!
//! Important to understanding `View` is understanding *world* versus *screen* coordinates.
//! *Screen* coordinates map the the window on the user's device. (0, 0) on the screen is the
//! top-left, and screen coordinates span to the pixel width and height of the window. *World*
//! coordinates are defined by the active view. By default, the world is a rectangle with the size
//! of the initial window.
//!
//! Here is a View in action (the camera example):
//! ```no_run
//! // Demonstrate adding a View to the draw-geometry example
//! // The camera can be controlled with the arrow keys
//! extern crate quicksilver;
//!
//! use quicksilver::{
//!     Result,
//!     geom::{Circle, Line, Rectangle, Shape, Transform, Triangle, Vector},
//!     graphics::{Background::Col, Color, View},
//!     input::{Key},
//!     lifecycle::{Settings, State, Window, run},
//! };
//!
//! struct Camera {
//!     view: Rectangle
//! }
//!
//! impl State for Camera {
//!     // Initialize the struct
//!     fn new() -> Result<Camera> {
//!         Ok(Camera {
//!             view: Rectangle::new_sized((800, 600))
//!         })
//!     }
//!
//!     fn update(&mut self, window: &mut Window) -> Result<()> {
//!         if window.keyboard()[Key::Left].is_down() {
//!             self.view = self.view.translate((-4, 0));
//!         }
//!         if window.keyboard()[Key::Right].is_down() {
//!             self.view = self.view.translate((4, 0));
//!         }
//!         if window.keyboard()[Key::Down].is_down() {
//!             self.view = self.view.translate((0, 4));
//!         }
//!         if window.keyboard()[Key::Up].is_down() {
//!             self.view = self.view.translate((0, -4));
//!         }
//!         window.set_view(View::new(self.view));
//!         Ok(())
//!     }
//!
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         window.draw(&Rectangle::new((100, 100), (32, 32)), Col(Color::BLUE));
//!         window.draw_ex(&Rectangle::new((400, 300), (32, 32)), Col(Color::BLUE), Transform::rotate(45), 10);
//!         window.draw(&Circle::new((400, 300), 100), Col(Color::GREEN));
//!         window.draw_ex(
//!             &Line::new((50, 80),(600, 450)).with_thickness(2.0),
//!             Col(Color::RED),
//!             Transform::IDENTITY,
//!             5
//!         );
//!         window.draw_ex(
//!             &Triangle::new((500, 50), (450, 100), (650, 150)),
//!             Col(Color::RED),
//!             Transform::rotate(45) * Transform::scale((0.5, 0.5)),
//!             0
//!         );
//!         Ok(())
//!     }
//! }
//!
//! fn main() {
//!     run::<Camera>("Camera", Vector::new(800, 600), Settings::default());
//! }
//! ```
