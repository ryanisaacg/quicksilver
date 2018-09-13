//! Interactability might be important, but just sticking to regular shapes can get a bit boring.
//!
//! It's time for the wonderful world of images! Drawing images is almost the same as drawing
//! colors, but instead of using `Background::Col` we use `Background::Img`. The big change to
//! learn with images is asset loading.
//! On desktop, you can make a blocking file load read, but that's not an option on web. This means
//! that *all* Quicksilver asset loading is asynchronous, through the `Asset` system.
//! ```no_run
//! // Draw an image to the screen
//! extern crate quicksilver;
//! 
//! use quicksilver::{
//!     Result,
//!     geom::{Shape, Vector},
//!     graphics::{Background::Img, Color, Image}, // We need Image and image backgrounds
//!     lifecycle::{Asset, Settings, State, Window, run}, // To load anything, we need Asset
//! };
//! 
//! struct ImageViewer {
//!     asset: Asset<Image>, // an image asset isn't state, but it does need to persist
//! }
//! 
//! impl State for ImageViewer {
//!     fn new() -> Result<ImageViewer> {
//!         let asset = Asset::new(Image::load("image.png")); // Start loading the asset
//!         Ok(ImageViewer { asset })
//!     }
//! 
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         self.asset.execute(|image| {
//!             // If we've loaded the image, draw it
//!             window.draw(&image.area().with_center((400, 300)), Img(&image));
//!             Ok(())
//!         })
//!     }
//! }
//! 
//! fn main() {
//!     run::<ImageViewer>("Image Example", Vector::new(800, 600), Settings {
//!         icon_path: Some("image.png"), // Set the window icon
//!         ..Settings::default()
//!     });
//! }
//! ```
//! You'll notice we provided a path to `Image::load`, which was "image.png." This asset is stored
//! in the `static` directory, not in the crate root. This is a current limitation of `cargo-web`
//! and may be changed in the future, but for now all assets must be stored in `static.` You can
//! use the [Quicksilver test
//! image](https://github.com/ryanisaacg/quicksilver/blob/development/static/image.png).
//!
//! The asset system uses Futures, which are an asychronous programming concept. Basically, a
//! Future is a computation that will complete at some point in, well, the future. For example,
//! loading an image over a network is a future: the web browser doesn't have the image downloaded
//! *yet* but it will (or it will produce an error.)
//!
//! This asset system is probably temporary, as async / await promise to be much more ergonomic
//! methods of dealing with futures. However, Rust's async / await story isn't stable yet, so the
//! Asset system is the most convenient way of loading things with Quicksilver. The execute
//! function on an Asset runs the provided closure if loading is complete, with the actual asset
//! data passed as a parameter. In the example above, the window is cleared every draw frame and
//! once the image is loaded, it is drawn to the screen.
//!
//! Additionally, we now set the application icon path. The icon is also sourced from `static`, and
//! determines the tab icon on the web and the window icon on desktop.
//!
//! Images can have subimages, which allows for spritesheets, texture atlases, and sprite batching:
//! ```no_run
//! // Draw an image to the screen
//! extern crate quicksilver;
//! 
//! use quicksilver::{
//!     Future, Result, // We need the Future trait to operate on a future
//!     geom::{Rectangle, Shape, Vector},
//!     graphics::{Background::Img, Color, Image},
//!     lifecycle::{Asset, Settings, State, Window, run},
//! };
//! 
//! struct ImageViewer {
//!     asset: Asset<Image>,
//! }
//! 
//! impl State for ImageViewer {
//!     fn new() -> Result<ImageViewer> {
//!         let asset = Asset::new(
//!             Image::load("image.png")
//!             // Between the image loading and the asset being "done", take a slice from it
//!                 .map(|image| image.subimage(Rectangle::new((0, 0), (32, 64))))
//!         );
//!         Ok(ImageViewer { asset })
//!     }
//! 
//!     fn draw(&mut self, window: &mut Window) -> Result<()> {
//!         window.clear(Color::WHITE)?;
//!         self.asset.execute(|image| {
//!             // If we've loaded the image, draw it
//!             window.draw(&image.area().with_center((400, 300)), Img(&image));
//!             Ok(())
//!         })
//!     }
//! }
//! 
//! fn main() {
//!     run::<ImageViewer>("Image Example", Vector::new(800, 600), Settings {
//!         icon_path: Some("image.png"), // Set the window icon
//!         ..Settings::default()
//!     });
//! }
//! ```
//! Here `map` applies a transformation to the image after it is loaded; the ability to chain
//! multiple assets together or apply complex operations to loading assets will be covered more in
//! depth in the asset combinator tutorial.
