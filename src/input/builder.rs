use geom::{Rectangle, Transform};
use super::{Mouse, Viewport, ViewportBuilder};

pub struct MouseBuilder {
    pub(crate) mouse: Mouse,
    pub(crate) viewport: ViewportBuilder
}

impl MouseBuilder {
    pub fn transform(self, trans: Transform) -> MouseBuilder {
        MouseBuilder {
            viewport: self.viewport.transform(trans),
            ..self
        }
    }

    pub fn build(&self, area: Rectangle) -> (Mouse, Viewport) {
        let viewport = self.viewport.build(area);
        let mouse = self.mouse.with_position(viewport.project() * self.mouse.pos);
        (mouse, viewport)
    }
}

