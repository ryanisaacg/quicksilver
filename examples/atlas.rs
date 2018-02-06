extern crate futures;
#[macro_use]
extern crate quicksilver;

use futures::{Async, Future};
use quicksilver::geom::Vector;
use quicksilver::graphics::{Atlas, AtlasLoader, Canvas, Color, Image, Window, WindowBuilder};
use quicksilver::{InitialScreen, Screen};

struct LoadingScreen {
    atlas: AtlasLoader
}

impl InitialScreen for LoadingScreen {
    fn new() -> LoadingScreen {
        LoadingScreen {
            atlas: Atlas::load("examples/image.atlas")
        }
    }

    fn configure() -> (Window, Canvas) {
        WindowBuilder::new()
            .build("Texture Atlas Test", 800, 600)
    }
}

impl Screen for LoadingScreen {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        if let Async::Ready(atlas) = self.atlas.poll().unwrap() {
            let black_and_white = atlas.get("blackandwhite").unwrap().unwrap_image();
            let yellow = atlas.get("yellow").unwrap().unwrap_image();
            let green = atlas.get("green").unwrap().unwrap_image();
            let purple = atlas.get("purple").unwrap().unwrap_image();
            Some(Box::new(MainScreen {
                black_and_white,
                yellow,
                green,
                purple
            }))
        } else {
            None
        }
    }

    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::white());
        canvas.present(window);
    }
}

struct MainScreen {
    black_and_white: Image,
    yellow: Image,
    green: Image,
    purple: Image
}

impl Screen for MainScreen {
    fn update(&mut self, _window: &mut Window, _canvas: &mut Canvas) -> Option<Box<Screen>> {
        None
    }
    
    fn draw(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.clear(Color::red());
        canvas.draw_image(&self.black_and_white, Vector::new(50, 50));
        canvas.draw_image(&self.yellow, Vector::new(100, 100));
        canvas.draw_image(&self.green, Vector::new(150, 150));
        canvas.draw_image(&self.purple, Vector::new(200, 200));
        canvas.present(window);
    }
}

screens_loop!(LoadingScreen);
