use crate::{
    geom::Vector,
    graphics::{ImageScaleStrategy, ResizeStrategy},
};

///A builder that constructs a Window
#[derive(Debug)]
pub struct Settings {
    /// If the cursor should be visible over the application
    pub show_cursor: bool,
    /// The smallest size the user can resize the window to
    ///
    /// Does nothing on web
    pub min_size: Option<Vector>,
    /// The largest size the user can resize the window to
    ///
    /// Does nothing on web
    pub max_size: Option<Vector>,
    /// How content should be presented when the window is resized
    pub resize: ResizeStrategy,
    /// How images should be scaled
    ///
    /// Does nothing on web
    pub scale: ImageScaleStrategy,
    /// If the application should be fullscreen
    pub fullscreen: bool,
    /// How many milliseconds should elapse between update calls
    pub update_rate: f64,
    /// The maximum number of updates to run in a single frame
    ///
    /// See https://gafferongames.com/post/fix_your_timestep/ for an explanation of fixed timesteps
    pub max_updates: u32,
    /// How many milliseconds should elapse between draw calls
    pub draw_rate: f64,
    /// The icon on the window or the favicon on the tab
    pub icon_path: Option<&'static str>, // TODO: statiC?
    /// If VSync should be enabled
    ///
    /// Does nothing on web currently
    pub vsync: bool,
    /// How many samples to do for MSAA
    ///
    /// By default it is None; if it is Some, it should be a non-zero power of two
    ///
    /// Does nothing on web currently
    pub multisampling: Option<u16>,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            show_cursor: true,
            min_size: None,
            max_size: None,
            resize: ResizeStrategy::default(),
            scale: ImageScaleStrategy::default(),
            fullscreen: false,
            update_rate: 1000. / 60.,
            max_updates: 0,
            draw_rate: 0.,
            icon_path: None,
            vsync: true,
            multisampling: None,
        }
    }
}
