// Draw some sample text to the screen
extern crate quicksilver;

use quicksilver::{
    Future, Result,
    combinators::result,
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Font, FontStyle, Image},
    lifecycle::{Asset, Event, Settings, State, Window, run},
};

struct SampleText {
    font: Font,
    image: Image,
    string: String,
}

fn render(font: &Font, string: &str) -> Result<Image> {
    font.render(string, &FontStyle::new(72.0, Color::BLACK))
}

impl State for SampleText {
    fn new() -> Result<SampleText> {
        let font = Font::from_slice(include_bytes!("../static/font.ttf"))?;
        let image = render(&font, "Sample Text")?;
        let string = "Sample Text".to_owned();
        Ok(SampleText {
            font,
            image,
            string,
        })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Typed(character) => {
                self.string.push(*character);
                self.image = render(&self.font, &self.string)?;
            }
            _ => ()
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw(&self.image.area().with_center((400, 300)), Img(&self.image));
        Ok(())
    }
}

fn main() {
    run::<SampleText>("Font Example", Vector::new(800, 600), Settings::default());
}
