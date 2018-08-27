//! We've now seen the four main methods that form the Quicksilver application lifecycle: `new`,
//! `update`, `draw`, and `event`. Before we go on, it might help to have an understanding of these
//! methods and when exactly they get called. 
//!
//! ## new
//!
//! `new` is the only mandatory function of `State`, which every Quicksilver application must
//! implement. Start all asset loading here, as well as initializing physics worlds or other
//! persistent state.
//!
//! Do not attempt to use *any* Quicksilver features before `new` runs! For example, do not call
//! `Image::load` in your main before you invoke `run`. Platform-specific setup occurs
//! behind-the-scenes, so just use `new` for all your initialization.
//!
//! ## draw
//!
//! `draw` is not mandatory, but it may as well be. By default, it will run as fast as vsync will
//! allow. You can choose to run it less often, by providing higher values to `draw_rate` in
//! Settings. For example, to only draw once every 35 milliseconds (approximately 30 FPS), you
//! could use the following `Settings` declaration:
//! ```no_run
//! # use quicksilver::{lifecycle::{State, Settings, run}};
//! # fn func<SomeStateType: State>(some_title: &'static str, some_dimensions: Vector) {
//! run::<SomeStateType>(some_title, some_dimensions, Settings {
//!     draw_rate: 35.0,
//!     ..Settings::default()
//! });
//! # }
//! ```
//! If you want to run the draw function as often as possible, you may want to disable vsync. You
//! can again do it with `Settings`: 
//! ```no_run
//! # use quicksilver::{lifecycle::{State, Settings, run}};
//! # fn func<SomeStateType: State>(some_title: &'static str, some_dimensions: Vector) {
//! run::<SomeStateType>(some_title, some_dimensions, Settings {
//!     vsync: false,
//!     ..Settings::default()
//! });
//! # }
//! ```
//! After each call to `draw`, the buffers are flipped (meaning your changes become visible to the
//! user.)
//!
//! ## update
//!
//! `update` is useful for any fixed-rate calculations or ticks. By default, it is called 60 times
//! per second, and will attempt to make up for any lost time. See [this Gaffer on Games blog
//! post](https://gafferongames.com/post/fix_your_timestep/) for a description of the algorithm.
//! You can change the tick rate with the `update_rate` setting, which determines how many
//! milliseconds take place between ticks.
//!
//! ## event
//!
//! `event` is called when the events are triggered, either immediately or buffered before the next
//! update. Events can form their own custom lifecycle: for example, listening for an
//! `Event::Closed` means you can run code to save the game state before the application
//! terminates. However, events aren't guaranteed to fire. If the user pulls the battery out of
//! their computer or a power outage shuts down a desktop, no event handler can ensure your code
//! runs.
