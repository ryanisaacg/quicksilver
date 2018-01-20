#[macro_use]
extern crate quicksilver;

use quicksilver::asset::{Loadable, LoadingAsset};
use quicksilver::geom::Vector;
use quicksilver::graphics::{Canvas, Color, Image, Window, WindowBuilder};
use quicksilver::Screen;

struct Loading {
    image: LoadingAsset<Image>
}


impl Loading {
    fn new() -> Loading {
        Loading {
            image: Image::load("examples/image.png")
        }
    }
}


impl Screen for Loading {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        self.image.update();
        match self.image {
            LoadingAsset::Loaded(ref image) => {
                let image = image.clone();
                Some(Box::new(Loaded { image }))
            },
            _ => None
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
        canvas.draw_image(&self.image, Vector::newi(100, 100));
        canvas.present(&window);
    }
}

screens_loop!(Loading, "Image Example", 800, 600);
