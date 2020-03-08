// Example 6: The Window
// Use timers to know when to draw and to have a consistent update cycle.
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    lifecycle::{run, EventStream, Settings, Window},
    Result, Timer,
};

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Square Example",
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> Result<()> {
    // Clear the screen to a blank, white color
    gfx.clear(Color::WHITE);

    //Here we make 2 kinds of timers.
    //One to provide an consistant update time, so our example updates 30 times per second
    //the other informs us when to draw the next frame, this causes our example to draw 60 times per second
    let mut update_timer = Timer::time_per_second(30.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut rect = Rectangle::new(Vector::new(0.0, 100.0), Vector::new(100.0, 100.0));

    loop {
        while let Some(_) = events.next_event().await {}

        //We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            rect.pos.x += 5.0;
        }

        while draw_timer.tick() {
            gfx.clear(Color::WHITE);
            gfx.fill_rect(&rect, Color::BLUE);
            gfx.stroke_rect(&rect, Color::RED);
            gfx.present(&window)?;
        }
    }
}
