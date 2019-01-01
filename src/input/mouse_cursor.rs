#[cfg(not(target_arch = "wasm32"))]
use glutin::MouseCursor as GlMouseCursor;

/// Mouse cursor modes
#[derive(Copy, Clone, Debug)]
pub enum MouseCursor {
    /// Default cursor
    Default,

    /// Arrow cursor
    Arrow,

    /// Copy cursor
    Copy,

    /// Crosshair cursor
    Crosshair,

    /// Grab cursor
    Grab,

    /// Grabbing cursor
    Grabbing,

    /// Hand cursor
    Hand,

    /// Help cursor
    Help,

    /// Move cursor
    Move,

    /// Progress cursor
    Progress,

    /// Text cursor
    Text,

    /// Wait cursor
    Wait,
}

impl MouseCursor {
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub(crate) fn into_css_style(self) -> &'static str {
        match self {
            MouseCursor::Default => "auto",
            MouseCursor::Arrow => "arrow",
            MouseCursor::Copy => "copy",
            MouseCursor::Crosshair => "crosshair",
            MouseCursor::Grab => "grab",
            MouseCursor::Grabbing => "grabbing",
            MouseCursor::Hand => "hand",
            MouseCursor::Help => "help",
            MouseCursor::Move => "move",
            MouseCursor::Progress => "progress",
            MouseCursor::Text => "text",
            MouseCursor::Wait => "wait",
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[inline]
    pub(crate) fn into_gl_cursor(self) -> GlMouseCursor {
        match self {
            MouseCursor::Default => GlMouseCursor::Default,
            MouseCursor::Arrow => GlMouseCursor::Arrow,
            MouseCursor::Copy => GlMouseCursor::Copy,
            MouseCursor::Crosshair => GlMouseCursor::Crosshair,
            MouseCursor::Grab => GlMouseCursor::Grab,
            MouseCursor::Grabbing => GlMouseCursor::Grabbing,
            MouseCursor::Hand => GlMouseCursor::Hand,
            MouseCursor::Help => GlMouseCursor::Help,
            MouseCursor::Move => GlMouseCursor::Move,
            MouseCursor::Progress => GlMouseCursor::Progress,
            MouseCursor::Text => GlMouseCursor::Text,
            MouseCursor::Wait => GlMouseCursor::Wait,
        }
    }
}

impl Default for MouseCursor {
    fn default() -> Self { MouseCursor::Default }
}