// Example 2: The Image
// Draw an image to the screen
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Font, Graphics, Image},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Image Example",
            ..Settings::default()
        },
        app,
    );
}

// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let mut font = Font::load_ttf(&gfx, "font.ttf").await?;
    gfx.clear(Color::WHITE);
    gfx.draw_text(&mut font, "Hello world!", 72.0, Color::BLACK, Vector::new(100.0, 100.0));
    gfx.present(&window)?;

    loop {
        while let Some(_) = events.next_event().await {}
    }
}
