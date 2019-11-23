// Draw an image to the screen
use mint::Vector2;
use quicksilver::{
    QuicksilverError,
    graphics::{Color, Context, Image, ImageDraw},
    lifecycle::{Event, EventStream, Settings, Window, run},
};

fn main() {
    // If we encounter an error while executing the app, unwrap it
    let handler = |window, gfx, events| async move {
        app(window, gfx, events).unwrap()
    };
    run(handler , Settings {
        size: Vector2 { x: 800, 600 },
        title: "Image Example",
        icon_path: Some("image.png"),
        ..Settings::default()
    });
}

// This time we might return an error, so we use a Result
async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<(), QuicksilverError> {
    // Load the image and wait for it to finish
    // We also use '?' to handle errors like file-not-found
    let image = Image::load(&mut gfx, "image.png").await?;

    while let Some(_) = events.next().await {
        gfx.clear(Color::WHITE);
        // Draw the image with the top-left at (100, 100)
        gfx.draw_image(&image, Vector2 { x: 400.0, y: 300.0 });
    }

    Ok(())
}

