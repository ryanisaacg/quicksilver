// Example 6: draw a progress bar while loading the application
//
// This (pretends to) load some data when starting and in between each step (eg. each texture or
// model or whatever), it updates a progress bar.
//
// Alternatively, one could also do loading in a background thread or threads and do only the
// redrawing in the main one, to separate it.
use std::thread::sleep;
use std::time::Duration;

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    run, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Square Example",
            ..Settings::default()
        },
        app,
    )
}

const STEPS: usize = 10;

fn load_something() {
    /*
     * Here would be some actual loading of something, like textures.
     * In this example we simply call sleep as a placeholder.
     *
     * TODO: Something better than sleep here would be nice.
     * Also, there's a problem with sleeps:
     * https://github.com/ryanisaacg/quicksilver/issues/580
     */
    sleep(Duration::from_secs(1));
}

fn draw_loader(window: &Window, gfx: &mut Graphics, progress: usize, total: usize) -> Result<()> {
    gfx.clear(Color::BLACK);
    gfx.fill_rect(
        &Rectangle::new(Vector::new(50.0, 500.0), Vector::new(700.0, 25.0)),
        Color::YELLOW,
    );

    let width = 700.0 * progress as f32 / total as f32;
    gfx.fill_rect(
        &Rectangle::new(Vector::new(50.0, 500.0), Vector::new(width, 25.0)),
        Color::BLUE,
    );

    /*
     * In real game, this might be a good place to make the loading screen nicer, possibly by
     * adding an image. We stick with just the progress bar in the example for simplicity.
     */

    gfx.present(&window)?;

    Ok(())
}

async fn app(window: Window, mut gfx: Graphics, mut events: Input) -> Result<()> {
    for i in 0..STEPS {
        draw_loader(&window, &mut gfx, i, STEPS)?;
        load_something();
    }

    // Now we have everything loaded. The rest is not interesting for this example, so it just
    // fills the whole window with green.

    gfx.clear(Color::GREEN);
    gfx.present(&window)?;

    loop {
        while let Some(_) = events.next_event().await {}
    }
}
