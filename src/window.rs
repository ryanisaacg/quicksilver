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
    /// [`scale`] when passing sizes to functions like `glViewport`.
    ///
    /// [`scale`]: Window::scale_factor
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
    /// For a good example of DPI scale factors, see the [winit docs] on the subject
    ///
    /// [winit docs]: winit::dpi
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
