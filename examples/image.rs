// Draw an image to the screen
extern crate image;
extern crate quicksilver;

use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

struct ImageViewer {
    asset: Asset<Image>,
}

impl State for ImageViewer {
    type Message = quicksilver::lifecycle::Event;

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
    run::<ImageViewer>(
        "Image Example",
        Vector::new(800, 600),
        Settings {
            icon_path: Some("image.png"),
            ..Settings::default()
        },
    );
}
