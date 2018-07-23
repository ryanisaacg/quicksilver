// Draw an image to the screen
extern crate quicksilver;

use quicksilver::{
    run, Asset, Result, State,
    geom::Transform,
    graphics::{Color, Image, RenderTarget, Window, WindowBuilder}
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
        window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            window.draw_ex(image, Transform::translate((400, 300)), Color::WHITE, 0);
            Ok(())
        })?;
        window.present()
    }
}

fn main() {
    run::<ImageViewer>(WindowBuilder::new("Image Example", (800, 600)));
}
