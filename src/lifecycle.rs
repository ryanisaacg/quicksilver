//! Manage events, input, and the window via the [`blinds`] library
//!
//! The [`run`] function is the entry point for all applications
use crate::geom::{Rectangle, Transform};
use crate::graphics::Graphics;
use crate::Result;
use std::future::Future;

pub use blinds::event;
#[cfg(feature = "event-cache")]
pub use blinds::{CachedEventStream, EventCache};
pub use blinds::{
    CursorIcon, Event, EventStream, GamepadAxis, GamepadButton, GamepadId, Key, MouseButton,
    PointerId, Settings, Window,
};

/// The entry point of a Quicksilver application
///
/// It provides your application (represented by an async closure or function) with a [`Window`],
/// [`Graphics`] context, and [`EventStream`].
pub fn run<F, T>(settings: Settings, app: F) -> !
where
    T: 'static + Future<Output = Result<()>>,
    F: 'static + FnOnce(Window, Graphics, EventStream) -> T,
{
    #[cfg(feature = "easy-log")]
    set_logger();

    let size = settings.size;
    let screen_region = Rectangle::new_sized(size);

    blinds::run_gl(settings, move |window, ctx, events| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if std::env::set_current_dir("static").is_err() {
                log::warn!("Warning: no asset directory found. Please place all your assets inside a directory called 'static' so they can be loaded");
                log::warn!("Execution continuing, but any asset-not-found errors are likely due to the lack of a 'static' directory.")
            }
        }

        let ctx = golem::Context::from_glow(ctx).unwrap();
        let mut graphics = Graphics::new(ctx).unwrap();
        graphics.set_projection(Transform::orthographic(screen_region));

        async {
            match app(window, graphics, events).await {
                Ok(()) => log::info!("Exited successfully"),
                Err(err) => {
                    log::error!("Error: {:?}", err);
                    panic!("{:?}", err);
                }
            }
        }
    });
}

#[cfg(feature = "easy-log")]
fn set_logger() {
    #[cfg(target_arch = "wasm32")]
    web_logger::custom_init(web_logger::Config {
        level: log::Level::Debug,
    });
    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_level(log::Level::Debug).expect("A logger was already initialized");
}
