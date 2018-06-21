// Draw some sample text to the screen
extern crate futures;
extern crate quicksilver;

use futures::{Async, Future};
use quicksilver::{
    Result, State, run,
    geom::Vector,
    graphics::{Color, Font, FontLoader, Image, Sprite, Window, WindowBuilder}
};

enum SampleText {
    Loading(FontLoader),
    Loaded(Image)
}

impl State for SampleText {
    fn new() -> Result<SampleText> { 
        Ok(SampleText::Loading(Font::load("examples/assets/font.ttf")))
    }

   fn update(&mut self, _: &mut Window) -> Result<()> {
       // Check to see the progress of the loading font 
       let result = match self {
           &mut SampleText::Loading(ref mut loader) => loader.poll().unwrap(),
           _ => Async::NotReady
       };
       // If the image has been loaded move to the loaded state
       if let Async::Ready(font) = result {
           *self = SampleText::Loaded(font.render("Sample Text", 72.0, Color::black()));
       }
       Ok(())
   }

   fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::white());
        // If the image is loaded draw it
        if let &mut SampleText::Loaded(ref image) = self {
            window.draw(&Sprite::image(image, Vector::new(400, 300)));
        }
        window.present();
        Ok(())
   }
}

fn main() {
    run::<SampleText>(WindowBuilder::new("Font Example", 800, 600)).unwrap();
}
