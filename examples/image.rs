// Draw an image to the screen
use mint::Vector2;
use quicksilver::{
    QuicksilverError,
    graphics::{Color, Context, Image, ImageDraw},
    lifecycle::{Event, EventStream, Settings, Window, run},
};

async fn app(window: Window, mut events: EventStream) -> Result<(), QuicksilverError> {
    let image = Image::load("image.png").await?;

    while let Some(_) = events.next().await {
        let cmd = DrawCommand::new(&mut window);
        cmd.clear(Color::WHITE)?;
        cmd.draw_image(Vector2 { x: 400.0, y: 300.0 }, image);
    }
}

fn main() {
    run(app, Settings {
        size: Vector2 { x: 800, 600 },
        title: "Image Example",
        icon_path: Some("image.png"),
        ..Settings::default()
    });
}
