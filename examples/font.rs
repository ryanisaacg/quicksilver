// Draw some sample text to the screen
extern crate quicksilver;

use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Settings, State, Window, run},
};

struct SampleText {
    asset: Asset<Image>,
    multiline: Asset<Image>,
}

impl State for SampleText {
    fn new() -> Result<SampleText> {
        let asset = Asset::new(Font::load("font.ttf")
            .and_then(|font| {
                let style = FontStyle::new(72.0, Color::BLACK);
                result(font.render("Sample Text", &style))
            }));
        let multiline = Asset::new(Font::load("font.ttf")
            .and_then(|font| {
                let style = FontStyle::new(48.0, Color::BLACK);
                result(font.render("First line\nSecond line\nThird line", &style))
            }));
        Ok(SampleText { asset, multiline })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((400, 300)), Img(&image));
            Ok(())
        })?;
        self.multiline.execute(|image| {
            window.draw(&image.area(), Img(&image));
            Ok(())
        })
    }
}

fn main() {
    run::<SampleText>("Font Example", Vector::new(800, 600), Settings::default());
}
