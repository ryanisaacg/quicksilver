// Draw an image to the screen
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::Shape,
    graphics::{Background::Img, Color, Image, Window, WindowBuilder},
    lifecycle::{Asset, State, run},
};

struct ImageViewer {
    asset: Asset<Image>,
}

impl State for ImageViewer {
    fn new() -> Result<ImageViewer> {
        let asset = Asset::new(Image::load("image.png"));
        Ok(ImageViewer { asset })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })
    }
}

fn main() {
    run::<ImageViewer>(WindowBuilder::new("Image Example", (800, 600)));
}
