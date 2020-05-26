use crate::geom::Vector;
use blinds::CursorIcon;

/// The window on the user's desktop or in the browser tab
pub struct Window(pub(crate) blinds::Window);

impl Window {
    /// Set the cursor icon to some value, or set it to invisible (None)
    pub fn set_cursor_icon(&self, icon: Option<CursorIcon>) {
        self.0.set_cursor_icon(icon);
    }

    /// Get the size of the window in logical units
    ///
    /// On a high-dpi display, this doesn't correspond to physical pixels and must be multiplied by
    /// [`scale_factor`] when passing sizes to functions like [`set_viewport`]
    ///
    /// [`scale_factor`]: Window::scale_factor
    /// [`set_viewport`]: crate::Graphics::set_viewport
    pub fn size(&self) -> Vector {
        self.0.size().into()
    }

    /// Set the size of the inside of the window in logical units
    pub fn set_size(&self, size: Vector) {
        self.0.set_size(size.into());
    }

    /// Set the title of the window or browser tab
    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    /// Set if the window should be fullscreen or not
    ///
    /// On desktop, it will instantly become fullscreen (borderless windowed on Windows and Linux,
    /// and fullscreen on macOS). On web, it will become fullscreen after the next user
    /// interaction, due to browser API restrictions.
    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.0.set_fullscreen(fullscreen);
    }

    /// The DPI scale factor of the window
    ///
    /// Mostly, this isn't important to you. Some computer screens have more "physical" pixels than
    /// "logical" pixels, allowing them to draw sharper-looking images. Quicksilver abstracts this
    /// away. However, if you are manually [setting the viewport], you need to take this into
    /// account.
    ///
    ///
    /// [setting the viewport]: crate::Graphics::set_viewport
    pub fn scale_factor(&self) -> f32 {
        self.0.scale_factor()
    }

    /// Draw the current frame to the screen
    ///
    /// If vsync is enabled, this will block until the frame is completed on desktop. On web, there
    /// is no way to control vsync, or to manually control presentation, so this function is a
    /// no-op.
    pub fn present(&self) {
        self.0.present();
    }
}
