// Example 1: The Square
// Open a window, and draw a colored square in it
use mint::Vector2;
use quicksilver::{
    geom::Rect,
    graphics::{Color, Graphics},
    lifecycle::{EventStream, Settings, Window, run},
    traits::*,
};

fn main() {
    run(Settings {
        size: Vector2 { x: 800.0, y: 600.0 },
        title: "Square Example",
        ..Settings::default()
    }, app);
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) {
    while let Some(_) = events.next().await {
        // Clear the screen to a blank, white color
        gfx.clear(Color::WHITE);
        // Paint a blue square in the center of our screen
        // It should have a top-left of (350, 100) and a bottom-left of (450, 200)
        gfx.draw_rect(Rect {
            min: Vector2 { x: 350.0, y: 100.0 },
            max: Vector2 { x: 450.0, y: 200.0 },
        }, Color::BLUE);
        // Send the data to be drawn
        gfx.present(&window);
    }
}

