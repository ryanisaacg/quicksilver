#[cfg(not(target_arch = "wasm32"))]
use glutin::MouseCursor as GlMouseCursor;

/// Mouse cursor styles
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MouseCursor {
    /// No cursor
    None,

    /// Default cursor
    Default,
    /// Crosshair cursor
    Crosshair,
    /// Hand cursor
    Hand,
    /// Arrow cursor
    Arrow,
    /// Move cursor
    Move,
    /// Text cursor
    Text,
    /// Wait cursor
    Wait,
    /// Help cursor
    Help,
    /// Progress cursor
    Progress,

    /// NotAllowed cursor
    NotAllowed,
    /// ContextMenu cursor
    ContextMenu,
    /// Cell cursor
    Cell,
    /// VerticalText cursor
    VerticalText,
    /// Alias cursor
    Alias,
    /// Copy cursor
    Copy,
    /// NoDrop cursor
    NoDrop,
    /// Grab cursor
    Grab,
    /// Grabbing cursor
    Grabbing,
    /// AllScroll cursor
    AllScroll,
    /// ZoomIn cursor
    ZoomIn,
    /// ZoomOut cursor
    ZoomOut,

    /// EResize cursor
    EResize,
    /// NResize cursor
    NResize,
    /// NeResize cursor
    NeResize,
    /// NwResize cursor
    NwResize,
    /// SResize cursor
    SResize,
    /// SeResize cursor
    SeResize,
    /// SwResize cursor
    SwResize,
    /// WResize cursor
    WResize,
    /// EwResize cursor
    EwResize,
    /// NsResize cursor
    NsResize,
    /// NeswResize cursor
    NeswResize,
    /// NwseResize cursor
    NwseResize,
    /// ColResize cursor
    ColResize,
    /// RowResize cursor
    RowResize,
}

impl MouseCursor {
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub(crate) fn into_css_style(self) -> &'static str {
        match self {
            MouseCursor::None => "none",
            MouseCursor::Default => "auto",
            MouseCursor::Crosshair => "crosshair",
            MouseCursor::Hand => "pointer",
            MouseCursor::Arrow => "default",
            MouseCursor::Move => "move",
            MouseCursor::Text => "text",
            MouseCursor::Wait => "wait",
            MouseCursor::Help => "help",
            MouseCursor::Progress => "progress",

            MouseCursor::NotAllowed => "not-allowed",
            MouseCursor::ContextMenu => "context-menu",
            MouseCursor::Cell => "cell",
            MouseCursor::VerticalText => "vertical-text",
            MouseCursor::Alias => "alias",
            MouseCursor::Copy => "copy",
            MouseCursor::NoDrop => "no-drop",
            MouseCursor::Grab => "grab",
            MouseCursor::Grabbing => "grabbing",
            MouseCursor::AllScroll => "all-scroll",
            MouseCursor::ZoomIn => "zoom-in",
            MouseCursor::ZoomOut => "zoom-out",

            MouseCursor::EResize => "e-resize",
            MouseCursor::NResize => "n-resize",
            MouseCursor::NeResize => "ne-resize",
            MouseCursor::NwResize => "nw-resize",
            MouseCursor::SResize => "s-resize",
            MouseCursor::SeResize => "se-resize",
            MouseCursor::SwResize => "sw-resize",
            MouseCursor::WResize => "w-resize",
            MouseCursor::EwResize => "ew-resize",
            MouseCursor::NsResize => "ns-resize",
            MouseCursor::NeswResize => "nesw-resize",
            MouseCursor::NwseResize => "nwse-resize",
            MouseCursor::ColResize => "col-resize",
            MouseCursor::RowResize => "row-resize",
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub(crate) fn into_gl_cursor(self) -> Option<GlMouseCursor> {
        match self {
            MouseCursor::None => None,
            MouseCursor::Default => Some(GlMouseCursor::Default),
            MouseCursor::Crosshair => Some(GlMouseCursor::Crosshair),
            MouseCursor::Hand => Some(GlMouseCursor::Hand),
            MouseCursor::Arrow => Some(GlMouseCursor::Arrow),
            MouseCursor::Move => Some(GlMouseCursor::Move),
            MouseCursor::Text => Some(GlMouseCursor::Text),
            MouseCursor::Wait => Some(GlMouseCursor::Wait),
            MouseCursor::Help => Some(GlMouseCursor::Help),
            MouseCursor::Progress => Some(GlMouseCursor::Progress),

            MouseCursor::NotAllowed => Some(GlMouseCursor::NotAllowed),
            MouseCursor::ContextMenu => Some(GlMouseCursor::ContextMenu),
            MouseCursor::Cell => Some(GlMouseCursor::Cell),
            MouseCursor::VerticalText => Some(GlMouseCursor::VerticalText),
            MouseCursor::Alias => Some(GlMouseCursor::Alias),
            MouseCursor::Copy => Some(GlMouseCursor::Copy),
            MouseCursor::NoDrop => Some(GlMouseCursor::NoDrop),
            MouseCursor::Grab => Some(GlMouseCursor::Grab),
            MouseCursor::Grabbing => Some(GlMouseCursor::Grabbing),
            MouseCursor::AllScroll => Some(GlMouseCursor::AllScroll),
            MouseCursor::ZoomIn => Some(GlMouseCursor::ZoomIn),
            MouseCursor::ZoomOut => Some(GlMouseCursor::ZoomOut),

            MouseCursor::EResize => Some(GlMouseCursor::EResize),
            MouseCursor::NResize => Some(GlMouseCursor::NResize),
            MouseCursor::NeResize => Some(GlMouseCursor::NeResize),
            MouseCursor::NwResize => Some(GlMouseCursor::NwResize),
            MouseCursor::SResize => Some(GlMouseCursor::SResize),
            MouseCursor::SeResize => Some(GlMouseCursor::SeResize),
            MouseCursor::SwResize => Some(GlMouseCursor::SwResize),
            MouseCursor::WResize => Some(GlMouseCursor::WResize),
            MouseCursor::EwResize => Some(GlMouseCursor::EwResize),
            MouseCursor::NsResize => Some(GlMouseCursor::NsResize),
            MouseCursor::NeswResize => Some(GlMouseCursor::NeswResize),
            MouseCursor::NwseResize => Some(GlMouseCursor::NwseResize),
            MouseCursor::ColResize => Some(GlMouseCursor::ColResize),
            MouseCursor::RowResize => Some(GlMouseCursor::RowResize),
        }
    }
}

impl Default for MouseCursor {
    fn default() -> Self { MouseCursor::Default }
}
