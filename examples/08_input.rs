// Example 8: Input
// Respond to user keyboard and mouse input onscreen
use quicksilver::{
    geom::{Circle, Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Input Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Keep track of the position of the square
    let mut square_position = Vector::new(300.0, 300.0);
    loop {
        while let Some(_) = input.next_event().await {}
        // Check the state of the keys, and move the square accordingly
        const SPEED: f32 = 2.0;
        if input.key_down(Key::A) {
            square_position.x -= SPEED;
        }
        if input.key_down(Key::D) {
            square_position.x += SPEED;
        }
        if input.key_down(Key::W) {
            square_position.y -= SPEED;
        }
        if input.key_down(Key::S) {
            square_position.y += SPEED;
        }

        gfx.clear(Color::WHITE);
        // Paint a blue square at the given position
        gfx.fill_rect(
            &Rectangle::new(square_position, Vector::new(64.0, 64.0)),
            Color::BLUE,
        );
        // Paint a red square at the mouse position
        let mouse = gfx.screen_to_world(&window, input.mouse().location());
        gfx.fill_circle(&Circle::new(mouse, 32.0), Color::RED);
        gfx.present(&window)?;
    }
}
