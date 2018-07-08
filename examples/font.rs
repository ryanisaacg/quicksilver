// Draw some sample text to the screen
extern crate futures;
extern crate quicksilver;

use quicksilver::{run, Asset, Future, Result, State, geom::Vector,
                  graphics::{Color, Font, FontStyle, Image, Sprite, Window, WindowBuilder}};

struct SampleText {
    asset: Asset<Image>,
}

impl State for SampleText {
    fn new() -> Result<SampleText> {
        let asset = Asset::new(Font::load("examples/assets/font.ttf").map(|font| {
            let style = FontStyle::new(72.0, Color::BLACK);
            font.render("Sample Text", style)
        }));
        Ok(SampleText { asset })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE);
        self.asset.execute(|image| {
            window.draw(&Sprite::image(image, Vector::new(400, 300)));
            Ok(())
        })?;
        window.present()
    }
}

fn main() {
    run::<SampleText>(WindowBuilder::new("Font Example", 800, 600)).unwrap();
}
