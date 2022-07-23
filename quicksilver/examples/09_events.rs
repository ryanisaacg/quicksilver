// Example 9: Events
// Draw user-typed text to the screen via events
use quicksilver::{
    geom::Vector,
    graphics::{Color, VectorFont},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Event Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // We'll need a font to render text and a string to store it
    let ttf = VectorFont::load("font.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 36.0)?;
    let mut string = String::new();
    // Instead of looping forever, terminate on a given input
    let mut running = true;
    while running {
        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) if key.is_down() => {
                    if key.key() == Key::Escape {
                        // If the user strikes escape, end the program
                        running = false;
                    } else if key.key() == Key::Back {
                        // If the user strikes Backspace, remove a character from our string
                        string.pop();
                    }
                }
                Event::ReceivedCharacter(c) => {
                    // If the user types a printable character, put it into the string
                    let chr = c.character();
                    if !chr.is_control() {
                        string.push(chr);
                    }
                }
                _ => (),
            }
        }

        // Draw our string to the screen, wrapping at word boundaries
        gfx.clear(Color::WHITE);
        font.draw_wrapping(
            &mut gfx,
            &string,
            Some(500.0),
            Color::BLACK,
            Vector::new(100.0, 100.0),
        )?;
        gfx.present(&window)?;
    }

    // Unlike all our earlier examples, our game loop might end early (e.g. before the user closes
    // the window.) We have to return Ok(()) because of this
    Ok(())
}
