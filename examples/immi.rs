use quicksilver::{
    Result,
    Future,
    geom::Vector,
    graphics::{Color, create_immi_ctx, ImmiStatus, ImmiRender, Font, Image},
    lifecycle::{Settings, State, Window, Asset, run},
};
use immi::{widgets::image, Alignment};

#[derive(Clone)]
struct UiState {
    immi_state: immi::UiState,
}

struct ImmiTest {
    ui_state: UiState,
    assets: Asset<(Font, Image)>,
}

impl State for ImmiTest {
    fn new() -> Result<ImmiTest> {
        Ok(ImmiTest {
            ui_state: UiState {
                immi_state: Default::default(),
            },
            assets: Asset::new(
                Font::load("font.ttf").join(
                    Image::load("image.png"))),
        })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;

        let ui_state = self.ui_state.clone();
        self.assets.execute(|(font, image)| {
            let ui_status = ImmiStatus::new(window);
            let mut ui_render = ImmiRender::new_with_window(window, font);
            let ui_context = create_immi_ctx(ui_status, &mut ui_render);
            image::draw(&ui_context, image, &Alignment::center());
            Ok(())
        })?;
        Ok(())
    }
}

pub fn main() {
    let mut settings = Settings::default();
    settings.fullscreen = true;
    run::<ImmiTest>(
        "ImmiTest",
        Vector::new(1680.0, 1050.0),
        Settings {
            fullscreen: true,
            ..Settings::default()
        },
    );
}
