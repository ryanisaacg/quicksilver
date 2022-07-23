use blinds::{run, CachedEventStream, EventStream, Key, Settings, Window};

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, events: EventStream) {
    let mut events = CachedEventStream::new(events);
    loop {
        while let Some(ev) = events.next_event().await {
            println!("{:?}", ev);
        }
        if events.cache().key(Key::Escape) {
            break;
        }
    }
}
