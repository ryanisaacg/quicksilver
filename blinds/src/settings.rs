use mint::Vector2;

/// The various options to pass to the Window and/or GL context
#[derive(Clone, PartialEq)]
pub struct Settings {
    /// The size of the window
    pub size: Vector2<f32>,
    /// If the cursor should be visible over the application, or if the cursor should be hidden
    pub cursor_icon: Option<CursorIcon>,
    /// If the application should be fullscreen
    pub fullscreen: bool,
    /// The icon on the window or the favicon on the tab
    #[cfg(feature = "image")]
    pub icon_path: Option<&'static str>,
    /// How many samples to do for MSAA
    ///
    /// By default it is None; if it is Some, it should be a non-zero power of two
    ///
    /// Does nothing on web currently
    pub multisampling: Option<u16>,
    /// Enable or disable vertical sync
    ///
    /// Does nothing on web; defaults to true
    pub vsync: bool,
    /// If the window can be resized by the user
    ///
    /// Does nothing on web; defaults to false
    pub resizable: bool,
    /// The title of your application
    pub title: &'static str,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            size: Vector2 {
                x: 1024.0,
                y: 768.0,
            },
            cursor_icon: Some(CursorIcon::Default),
            fullscreen: false,
            #[cfg(feature = "image")]
            icon_path: None,
            multisampling: None,
            vsync: true,
            resizable: false,
            title: "",
        }
    }
}

/// The options for the cursor icon
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl Default for CursorIcon {
    fn default() -> CursorIcon {
        CursorIcon::Default
    }
}
