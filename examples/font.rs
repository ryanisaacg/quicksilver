// Draw some sample text to the screen
use mint::Vector2;
use quicksilver::{
    QuicksilverError,
    graphics::{Color, Font, FontStyle},
    lifecycle::{Event, EventStream, Settings, State, Window, run},
};

async fn app(window: Window, mut events: EventStream) -> Result<(), QuicksilverError> {
    let context = Context::new(&mut window);

    let font = Font::load("font.ttf").await?;
    let sample = font.render("Sample Text", FontStyle::new(72.0, Color::BLACK));
    let multiline = font.render("First line\nSecond line\nThird line", FontStyle::new(48.0, Color::BLACK));

    while let Some(event) = events.next().await {
        if let Event::Draw = event {
            context.clear(Color::WHITE)?;
            context.draw(Vector2 { x: 0.0, y: 0.0 }, &multiline);
            context.draw(Vector2 { x: 300.0, y: 400.0 }, &sample);
            context.present()?;
        }
    }
}

fn main() {
    run(app, Settings {
        title: "Font Example",
        size: Vector2 { x: 800, y: 600 },
        ..Settings::default()
    });
}
