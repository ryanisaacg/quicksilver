// Draw an image to the screen
extern crate quicksilver;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Image, Sprite, Window, WindowBuilder},
    run,
    Asset,
    Result,
    State,
};

struct ImageViewer {
    asset: Asset<Image>,
}

impl State for ImageViewer {
    fn new() -> Result<ImageViewer> {
        let asset = Asset::new(Image::load("examples/assets/image.png"));
        Ok(ImageViewer { asset })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::white());
        self.asset.execute(|image| {
                                window.draw(&Sprite::image(image, Vector::new(400, 300)));
                                Ok(())
                            })?;
        window.present()
    }
}

fn main() { run::<ImageViewer>(WindowBuilder::new("Image Example", 800, 600)).unwrap(); }
