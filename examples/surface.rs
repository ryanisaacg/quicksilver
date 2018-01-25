#[macro_use]
extern crate quicksilver;

use quicksilver::asset::LoadingAsset;
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::graphics::{Canvas, Color, Image, Surface, Window, WindowBuilder};
use quicksilver::{InitialScreen, Screen};

struct Loading {
    image: LoadingAsset<Image>
}

impl InitialScreen for Loading {
    fn new() -> Loading {
        Loading {
            image: Image::load("examples/image.png")
        }
    }

    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .build("Surface Example", 800, 600)
    }
}

impl Screen for Loading {
    fn update(&mut self, _window: &mut Window, canvas: &mut Canvas) -> Option<Box<Screen>> {
        self.image.update();
        if let LoadingAsset::Loaded(ref image) = self.image {
            let surface = Surface::new(600, 400);
            surface.render_to(|canvas| {
                canvas.clear(Color::white());
                canvas.draw_image(image, Vector::zero());
                canvas.draw_rect(Rectangle::newi_sized(300, 200), Color::green());
            }, canvas);
            Some(Box::new(State { surface }))
        } else {
            None
        }
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
        canvas.draw_image(self.surface.image(), Vector::newi(400, 300));
        canvas.present(window);
    }
}

screens_loop!(Loading);
