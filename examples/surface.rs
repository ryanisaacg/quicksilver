#[macro_use]
extern crate quicksilver;

use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::{Canvas, Color, Surface, Window, WindowBuilder};
use quicksilver::{InitialScreen, Screen};

struct Loading;

impl InitialScreen for Loading {
    fn new() -> Loading {
        Loading
    }

    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .build("Surface Example", 800, 600)
    }
}

impl Screen for Loading {
    fn update(&mut self, _window: &mut Window, canvas: &mut Canvas) -> Option<Box<Screen>> {
        let surface = Surface::new(600, 400);
        surface.render_to(|canvas| {
            canvas.clear(Color::white());
            canvas.draw_rect(Rectangle::new_sized(300, 200), Color::green());
        }, canvas);
        Some(Box::new(State { surface }))
    }

    fn draw(&mut self, _window: &mut Window, _canvas: &mut Canvas) {

    }
}

struct State {
    surface: Surface
}

impl Screen for State {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        None
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
        canvas.draw_image(self.surface.image(), Vector::new(400, 300));
        canvas.present(window);
    }
}

screens_loop!(Loading);
