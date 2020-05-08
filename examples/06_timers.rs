// Example 6: The Window
// Use timers to know when to draw and to have a consistent update cycle.
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Timer, Window,
};

fn main() {
    run(
        Settings {
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);

    // Here we make 2 kinds of timers.
    // One to provide an consistant update time, so our example updates 30 times per second
    // the other informs us when to draw the next frame, this causes our example to draw 60 times per second
    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut rect = Rectangle::new(Vector::new(0.0, 100.0), Vector::new(100.0, 100.0));

    loop {
        while let Some(_) = input.next_event().await {}

        // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            rect.pos.x += 5.0;
        }

        // Unlike the update cycle drawing doesn't change our state
        // Because of this there is no point in trying to catch up if we are ever 2 frames late
        // Instead it is better to drop/skip the lost frames
        if draw_timer.exhaust().is_some() {
            gfx.clear(Color::WHITE);
            gfx.fill_rect(&rect, Color::BLUE);
            gfx.stroke_rect(&rect, Color::RED);
            gfx.present(&window)?;
        }
    }
}
