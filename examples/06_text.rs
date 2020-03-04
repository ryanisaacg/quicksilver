// Example 6: Text
// Write some text on the screen
use quicksilver::{
    geom::Vector,
    graphics::{Color, Font, Graphics},
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

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Load the Font, just like loading any other asset
    let mut font = Font::load_ttf(&gfx, "font.ttf").await?;
    gfx.clear(Color::WHITE);
    // Use the font rendering API to draw some text
    gfx.draw_text(
        &mut font,
        "Hello world!\nHello Quicksilver!",
        72.0,
        None,
        Color::BLACK,
        Vector::new(100.0, 100.0),
    );
    gfx.present(&window)?;

    loop {
        while let Some(_) = events.next_event().await {}
    }
}
