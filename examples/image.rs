// Draw an image to the screen
use mint::Vector2;
use quicksilver::{
    QuicksilverError,
    graphics::{Color, Image},
    lifecycle::{Event, EventStream, Settings, Window, run},
};

async fn app(window: Window, mut events: EventStream) -> Result<(), QuicksilverError> {
    let context = Context::new(&mut window);

    let image = Image::load("image.png").await?;

    while let Some(event) = events.next().await {
        if let Event::Draw = event {
            context.clear(Color::WHITE)?;
            context.draw(Vector2 { x: 400.0, y: 300.0 }, &image);
            context.present()?;
        }
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
