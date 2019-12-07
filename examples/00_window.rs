// Example 0: The Window
// The simplest example: Do absolutely nothing other than just opening a window

use mint::Vector2;
use quicksilver::{
    Result,
    graphics::Graphics,
    lifecycle::{EventStream, Settings, Window, run},
    traits::*,
};

// main() serves as our kicking-off point, but it doesn't have our application logic
// Actual logic goes in our app function, which is async
// 'run' manages loading resources asynchronously and dealing with the event loop
fn main() {
    run(Settings {
        size: Vector2 { x: 800.0, y: 600.0 },
        title: "Window Example",
        ..Settings::default()
    }, app);
}

// Our actual logic! Not much to see for this example
async fn app(_window: Window, _gfx: Graphics, mut events: EventStream) -> Result<()> {
    while let Some(_) = events.next().await {
        // Normally we'd do some processing here
        // When this loop ends, the window will close
        // That happens either when the user closes the window or when the loop is broken
    }
    Ok(())
}

