#[cfg(not(target_arch = "wasm32"))]
use glutin::MouseCursor as GlMouseCursor;

/// Mouse cursor styles
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum MouseCursor {
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
    pub(crate) fn into_gl_cursor(self) -> GlMouseCursor {
        match self {
            MouseCursor::Default => GlMouseCursor::Default,
            MouseCursor::Crosshair => GlMouseCursor::Crosshair,
            MouseCursor::Hand => GlMouseCursor::Hand,
            MouseCursor::Arrow => GlMouseCursor::Arrow,
            MouseCursor::Move => GlMouseCursor::Move,
            MouseCursor::Text => GlMouseCursor::Text,
            MouseCursor::Wait => GlMouseCursor::Wait,
            MouseCursor::Help => GlMouseCursor::Help,
            MouseCursor::Progress => GlMouseCursor::Progress,

            MouseCursor::NotAllowed => GlMouseCursor::NotAllowed,
            MouseCursor::ContextMenu => GlMouseCursor::ContextMenu,
            MouseCursor::Cell => GlMouseCursor::Cell,
            MouseCursor::VerticalText => GlMouseCursor::VerticalText,
            MouseCursor::Alias => GlMouseCursor::Alias,
            MouseCursor::Copy => GlMouseCursor::Copy,
            MouseCursor::NoDrop => GlMouseCursor::NoDrop,
            MouseCursor::Grab => GlMouseCursor::Grab,
            MouseCursor::Grabbing => GlMouseCursor::Grabbing,
            MouseCursor::AllScroll => GlMouseCursor::AllScroll,
            MouseCursor::ZoomIn => GlMouseCursor::ZoomIn,
            MouseCursor::ZoomOut => GlMouseCursor::ZoomOut,

            MouseCursor::EResize => GlMouseCursor::EResize,
            MouseCursor::NResize => GlMouseCursor::NResize,
            MouseCursor::NeResize => GlMouseCursor::NeResize,
            MouseCursor::NwResize => GlMouseCursor::NwResize,
            MouseCursor::SResize => GlMouseCursor::SResize,
            MouseCursor::SeResize => GlMouseCursor::SeResize,
            MouseCursor::SwResize => GlMouseCursor::SwResize,
            MouseCursor::WResize => GlMouseCursor::WResize,
            MouseCursor::EwResize => GlMouseCursor::EwResize,
            MouseCursor::NsResize => GlMouseCursor::NsResize,
            MouseCursor::NeswResize => GlMouseCursor::NeswResize,
            MouseCursor::NwseResize => GlMouseCursor::NwseResize,
            MouseCursor::ColResize => GlMouseCursor::ColResize,
            MouseCursor::RowResize => GlMouseCursor::RowResize,
        }
    }
}

impl Default for MouseCursor {
    fn default() -> Self { MouseCursor::Default }
}
