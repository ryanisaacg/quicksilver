use geom::{Rectangle, Transform};
use super::{Keyboard, Mouse, Viewport, ViewportBuilder};

pub struct InputBuilder<'a> {
    pub(crate) keyboard: &'a Keyboard, 
    pub(crate) mouse: Mouse,
    pub(crate) viewport: ViewportBuilder
}

impl<'a> InputBuilder<'a> {
    pub fn transform(self, trans: Transform) -> InputBuilder<'a> {
        InputBuilder {
            viewport: self.viewport.transform(trans),
            ..self
        }
    }

    pub fn build(&self, area: Rectangle) -> (&Keyboard, Mouse, Viewport) {
        let viewport = self.viewport.build(area);
        let mouse = self.mouse.with_position(viewport.project() * self.mouse.pos);
        (self.keyboard, mouse, viewport)
    }
}

