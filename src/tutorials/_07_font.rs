//! Armed with asset combinators and the ability to draw textures, we can now do some text
//! rendering. Fonts are loaded just like images: with a `load` function. A simple font example
//! could be:
//! ```no_run
//! // Draw some sample text to the screen
//! extern crate quicksilver;
//! 
//! use quicksilver::{
//!     Future, Result,
//!     combinators::result,
//!     geom::{Shape, Vector},
//!     graphics::{Background::Img, Color, Font, FontStyle, Image},
//!     lifecycle::{Asset, Settings, State, Window, run},
//! };
//! 
//! struct SampleText {
//!     asset: Asset<Image>,
//! }
//! 
//! impl State for SampleText {
//!     fn new() -> Result<SampleText> {
//!         let asset = Asset::new(Font::load("font.ttf")
//!             .and_then(|font| {
//!                 let style = FontStyle::new(72.0, Color::BLACK);
//!                 result(font.render("Sample Text", &style))
//!             }));
//!         Ok(SampleText { asset })
//!     }
//! 
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         self.asset.execute(|image| {
//!             window.draw(&image.area().with_center((400, 300)), Img(&image));
//!             Ok(())
//!         })
//!     }
//! }
//! 
//! fn main() {
//!     run::<SampleText>("Font Example", Vector::new(800, 600), Settings::default());
//! }
//! ```
//! [`font.render`](quicksilver::graphics::Font::render) a string into an image with a given font
//! style. It is not recommended to call this function often, as it is fairly expensive; future
//! updates to Quicksilver should make it cheaper.
//!
//! Each different font face (including bold and italic) requires a new `Font` object, and all
//! fonts must be loaded from local files (system fonts will not be found.)
