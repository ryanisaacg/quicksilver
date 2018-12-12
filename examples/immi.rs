use quicksilver::{
    Result, Future,
    combinators::result,
    geom::Vector,
    graphics::{Color, create_immi_ctx, ImmiStatus, ImmiRender, Font, FontStyle, Image},
    lifecycle::{Settings, State, Window, Asset, run},
};
use immi::{
    Alignment,
    widgets::{Interaction, image_button}
};

struct ImmiExample {
    ui_state: immi::UiState,
    assets: Asset<(Font, ButtonState)>,
}

struct ButtonState {
    normal: Image,
    hovered: Image,
    active: Image,
}

impl ButtonState {
    fn new(font: Font, style: &FontStyle) -> Result<(Font, ButtonState)> {
        let normal = font.render("Normal Button", &style)?;
        let hovered = font.render("Hovered Button", &style)?;
        let active = font.render("Active Button", &style)?;
        Ok((font, ButtonState { normal, hovered, active }))
    }
}

impl State for ImmiExample {
    fn new() -> Result<ImmiExample> {
        Ok(ImmiExample {
            ui_state: Default::default(),
            // Load the font and draw the text for the 3 button states
            assets: Asset::new(
                Font::load("font.ttf").and_then(|font| {
                    let style = FontStyle::new(48.0, Color::BLACK);
                    result(ButtonState::new(font, &style))
                }))
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let ui_state = &mut self.ui_state;
        self.assets.execute(|(font, button)| {
            // Set up IMMI resources
            let ui_status = ImmiStatus::new(window);
            let mut ui_render = ImmiRender::new_with_window(window, font);
            let ui_context = create_immi_ctx(ui_status, &mut ui_render)
                // Only take up half the screen with the immi widgets
                .rescale(0.5, 0.5, &Alignment::center());

            // Draw a button widget and if it's clicked, print test
            match image_button::draw(&ui_context, ui_state, &button.normal, &button.hovered, &button.active, &Alignment::center()) {
                Interaction::Clicked => println!("Test!"),
                _ => ()
            }

            Ok(())
        })?;
        Ok(())
    }
}

pub fn main() {
    run::<ImmiExample>("Immi Example", Vector::new(800, 600), Settings::default());
}
