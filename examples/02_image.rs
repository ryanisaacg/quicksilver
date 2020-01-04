// Example 2: The Image
// Draw an image to the screen
use mint::Vector2;
use quicksilver::{
    graphics::{Color, Graphics, Image},
    lifecycle::{run, EventStream, Settings, Window},
    Result,
};

fn main() {
    run(
        Settings {
            size: Vector2 { x: 800.0, y: 600.0 },
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
    let image = Image::load(&gfx, "image.png").await?;

    loop {
        while let Some(_) = events.next_event().await {}
        gfx.clear(Color::WHITE);
        // Draw the image with the top-left at (100, 100)
        gfx.draw_image(&image, Vector2 { x: 400.0, y: 300.0 });
        gfx.present(&window)?;
    }
}
