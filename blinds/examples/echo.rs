use blinds::{run, Event, EventStream, Key, Settings, Window};

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventStream) {
    'outer: loop {
        while let Some(ev) = events.next_event().await {
            match ev {
                Event::KeyboardInput(e) if e.key() == Key::Escape => {
                    break 'outer;
                }
                ev => println!("{:?}", ev),
            }
        }
    }
}
