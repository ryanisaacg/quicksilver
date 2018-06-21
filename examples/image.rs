// Draw an image to the screen
extern crate quicksilver;

use quicksilver::{
    Async, Future, Result, State, run,
    geom::Vector,
    graphics::{Color, Image, ImageLoader, Sprite, Window, WindowBuilder}
};

enum ImageViewer {
    Loading(ImageLoader),
    Loaded(Image)
}

impl State for ImageViewer {
    fn new() -> Result<ImageViewer> { 
        Ok(ImageViewer::Loading(Image::load("examples/assets/image.png")))
    }

   fn update(&mut self, _: &mut Window) -> Result<()> {
       // Check to see the progress of the loading image
       let result = match self {
           &mut ImageViewer::Loading(ref mut loader) => loader.poll().unwrap(),
           _ => Async::NotReady
       };
       // If the image has been loaded move to the loaded state
       if let Async::Ready(asset) = result {
           *self = ImageViewer::Loaded(asset);
       }
       Ok(())
   }

   fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::white());
        // If the image is loaded draw it
        if let &mut ImageViewer::Loaded(ref image) = self {
            window.draw(&Sprite::image(image, Vector::new(400, 300)));
        }
        window.present();
        Ok(())
   }
}

fn main() {
    run::<ImageViewer>(WindowBuilder::new("Image Example", 800, 600)).unwrap();
}
