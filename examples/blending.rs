#[macro_use]
extern crate quicksilver;

use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::{BlendMode, Canvas, Color, Surface, Window, WindowBuilder};
use quicksilver::{InitialScreen, Screen};

struct LoadingScreen;

impl InitialScreen for LoadingScreen {
    fn new() -> LoadingScreen { LoadingScreen }

    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .build("Blend Mode", 800, 600)
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _window: &mut Window, canvas: &mut Canvas) -> Option<Box<Screen>>  {
        let surface = Surface::new(800, 600);
        surface.render_to(|canvas| {
            canvas.clear(Color::white());
            canvas.draw_rect(Rectangle::new_sized(600, 500).with_center(Vector::new(400, 300)), Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 });
            canvas.draw_rect(Rectangle::new_sized(400, 300).with_center(Vector::new(400, 300)), Color::black());
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
        canvas.draw_rect(Rectangle::new_sized(800, 600), Color::green());
        canvas.draw_surface(&self.surface, Vector::new(400, 300), BlendMode::Subtractive);
        canvas.present(window);
    }
}

screens_loop!(LoadingScreen);

