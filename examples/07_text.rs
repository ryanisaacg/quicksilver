// Example 7: Text
// Write some text on the screen
use quicksilver::{
    geom::Vector,
    graphics::{Color, Graphics, VectorFont},
    input::{Input, Window},
    run, Result, Settings,
};

fn main() {
    run(
        Settings {
            title: "Font Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Load the Font, just like loading any other asset
    let ttf = VectorFont::load("font.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 72.0)?;
    gfx.clear(Color::WHITE);
    // Use the font rendering API to draw some text
    font.draw(
        &mut gfx,
        "Hello world!\nHello Quicksilver!",
        Color::BLACK,
        Vector::new(100.0, 100.0),
    )?;
    gfx.present(&window)?;

    loop {
        while let Some(_) = input.next_event().await {}
    }
}
