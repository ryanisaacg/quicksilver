use {
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
    pub scale: ImageScaleStrategy,
    /// If the application should be fullscreen
    /// 
    /// Does nothing on web currently
    pub fullscreen: bool,
    /// How many milliseconds should elapse between 2 ticks
    pub tick_rate: f64,
    /// The maximum number of ticks to run in a single frame
    /// 
    /// See https://gafferongames.com/post/fix_your_timestep/ for an explanation of fixed timesteps
    pub max_ticks: u32,
    /// The icon on the window or the favicon on the tab
    pub icon_path: Option<&'static str>, // TODO: statiC?
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
            tick_rate: 1000.0 / 60.0,
            max_ticks: 0,
            icon_path: None,
        }
    }
}
