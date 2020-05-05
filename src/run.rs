use crate::geom::{Rectangle, Transform, Vector};
use crate::graphics::Graphics;
use crate::input::Input;
use std::error::Error;
use std::future::Future;

pub struct Settings {
    /// The size of the window
    pub size: Vector,
    /// If the cursor should be visible over the application, or if the cursor should be hidden
    pub cursor_icon: Option<blinds::CursorIcon>,
    /// If the application should be fullscreen
    pub fullscreen: bool,
    /// The icon on the window or the favicon on the tab
    pub icon_path: Option<&'static str>,
    /// How many samples to do for MSAA
    ///
    /// By default it is None; if it is Some, it should be a non-zero power of two
    ///
    /// Does nothing on web currently
    pub multisampling: Option<u16>,
    /// Enable or disable vertical sync
    ///
    /// Does nothing on web
    pub vsync: bool,
    /// If the window can be resized by the user
    ///
    /// Does nothing on web
    pub resizable: bool,
    /// The title of your application
    pub title: &'static str,
    /// The severity level of logs to show
    ///
    /// By default, it is set to Warn
    pub log_level: log::Level,
    /// On desktop, whether to assume the assets are in the 'static/' directory
    ///
    /// By default, this is on for comfortable parity between stdweb and desktop. If you know you
    /// don't need that, feel free to toggle this off
    pub use_static_dir: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: Vector::new(1024.0, 768.0),
            cursor_icon: Some(blinds::CursorIcon::Default),
            fullscreen: false,
            icon_path: None,
            multisampling: None,
            vsync: true,
            resizable: false,
            title: "",
            log_level: log::Level::Warn,
            use_static_dir: true,
        }
    }
}

/// The entry point of a Quicksilver application
///
/// It provides your application (represented by an async closure or function) with a [`Window`],
/// [`Graphics`] context, and [`EventStream`].
pub fn run<E, F, T>(settings: Settings, app: F) -> !
where
    E: Into<Box<dyn Error + Send + Sync>>,
    T: 'static + Future<Output = Result<(), E>>,
    F: 'static + FnOnce(crate::Window, Graphics, Input) -> T,
{
    #[cfg(feature = "easy-log")]
    set_logger(settings.log_level);

    let size = settings.size;
    let screen_region = Rectangle::new_sized(size);

    blinds::run_gl((&settings).into(), move |window, ctx, events| {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if settings.use_static_dir && std::env::set_current_dir("static").is_err() {
                log::warn!("Warning: no asset directory found. Please place all your assets inside a directory called 'static' so they can be loaded");
                log::warn!("Execution continuing, but any asset-not-found errors are likely due to the lack of a 'static' directory.")
            }
        }

        let ctx = golem::Context::from_glow(ctx).unwrap();
        let mut graphics = Graphics::new(ctx).unwrap();
        graphics.set_projection(Transform::orthographic(screen_region));

        async {
            match app(crate::Window(window), graphics, Input::new(events)).await {
                Ok(()) => log::info!("Exited successfully"),
                Err(err) => {
                    let err = err.into();
                    log::error!("Error: {:?}", err);
                    panic!("{:?}", err);
                }
            }
        }
    });
}

#[cfg(feature = "easy-log")]
fn set_logger(level: log::Level) {
    #[cfg(target_arch = "wasm32")]
    web_logger::custom_init(web_logger::Config { level });
    #[cfg(not(target_arch = "wasm32"))]
    simple_logger::init_with_level(level).expect("A logger was already initialized");
}

impl From<&Settings> for blinds::Settings {
    fn from(settings: &Settings) -> blinds::Settings {
        blinds::Settings {
            size: settings.size.into(),
            cursor_icon: settings.cursor_icon,
            fullscreen: settings.fullscreen,
            icon_path: settings.icon_path,
            multisampling: settings.multisampling,
            vsync: settings.vsync,
            resizable: settings.resizable,
            title: settings.title,
        }
    }
}

impl From<blinds::Settings> for Settings {
    fn from(settings: blinds::Settings) -> Settings {
        Settings {
            size: settings.size.into(),
            cursor_icon: settings.cursor_icon,
            fullscreen: settings.fullscreen,
            icon_path: settings.icon_path,
            multisampling: settings.multisampling,
            vsync: settings.vsync,
            resizable: settings.resizable,
            title: settings.title,
            ..Settings::default()
        }
    }
}
