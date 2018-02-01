extern crate futures;
#[macro_use]
extern crate quicksilver;

use futures::{Async, Future};
use quicksilver::geom::{Vector, Transform};
use quicksilver::graphics::{Canvas, Color, Image, ImageLoader, Window, WindowBuilder};
use quicksilver::{InitialScreen, Screen};

struct Loading {
    image: ImageLoader
}

impl InitialScreen for Loading {
    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .build("Image Example", 800, 600)
    }

    fn new() -> Loading {
        Loading {
            image: Image::load("examples/image.png")
        }
    }
}

impl Screen for Loading {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        if let Async::Ready(image) = self.image.poll().unwrap() {
            Some(Box::new(Loaded { image }))
        } else {
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::black());
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
        canvas.draw_image_trans(&self.image, Vector::new(100, 100), Color::white(), Transform::scale(Vector::new(2, 2)));
        canvas.present(&window);
    }
}

screens_loop!(Loading);
