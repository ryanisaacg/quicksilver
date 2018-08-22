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
    /// How many milliseconds should elapse between update calls
    pub update_rate: f64,
    /// How many milliseconds should elapse between draw calls
    pub draw_rate: f64,
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
            update_rate: 1000. / 60.,
            draw_rate: 0.,
            icon_path: None,
        }
    }
}
