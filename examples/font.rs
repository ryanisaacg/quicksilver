// Draw some sample text to the screen
extern crate quicksilver;

use quicksilver::{
    run, Asset, Future, Result, State,
    combinators::result,
    graphics::{Background::Img, Color, Font, FontStyle, Image, Window, WindowBuilder}
};

struct SampleText {
    asset: Asset<Image>,
}

impl State for SampleText {
    fn new() -> Result<SampleText> {
        let asset = Asset::new(Font::load("examples/assets/font.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::BLACK);
                result(font.render("Sample Text", &style))
            }));
        Ok(SampleText { asset })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })?;
        window.present()
    }
}

fn main() {
    run::<SampleText>(WindowBuilder::new("Font Example", (800, 600)));
}
