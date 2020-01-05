// Example 2: The Image
// Draw an image to the screen
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics, Image},
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
    let image = Image::load(&gfx, "image.png").await?;
    gfx.clear(Color::WHITE);
    // Draw the image with the top-left at (100, 100)
    let region = Rectangle::new(Vector::new(100.0, 100.0), image.size());
    gfx.draw_image(&image, region);
    gfx.present(&window)?;

    loop {
        while let Some(_) = events.next_event().await {}
    }
}
