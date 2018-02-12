extern crate futures;
#[macro_use]
extern crate quicksilver;

use futures::{Async, Future};
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::{Canvas, Color, Font, FontLoader, Image, Window, WindowBuilder};
use quicksilver::{InitialScreen, Screen};

struct Loading {
    font: FontLoader
}

impl InitialScreen for Loading {
    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .build("Font Example", 800, 600)
    }

    fn new() -> Loading {
        Loading {
            font: Font::load("examples/font.ttf")
        }
    }
}

impl Screen for Loading {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        if let Async::Ready(font) = self.font.poll().unwrap() {
            let image = font.render("Hello world!", 32.0, Color::black());
            Some(Box::new(Loaded { image }))
        } else {
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
        canvas.draw_rect(Rectangle::new(0, 0, 10, 10), Color::blue());
        canvas.present(&window);
    }
}

struct Loaded {
    image: Image
}

impl Screen for Loaded {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> { None }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::white());
        canvas.draw_image(&self.image, Vector::new(100, 100));
        canvas.present(&window);
    }
}

screens_loop!(Loading);
