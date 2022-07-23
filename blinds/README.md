# blinds

`blinds` covers up the details of your windowing for you, by providing an async API.

```rust
use blinds::{run, Event, EventStream, Key, Settings, Window};

fn main() {
    run(Settings::default(), app);
}

async fn app(_window: Window, mut events: EventStream) {
    loop {
        while let Some(ev) = events.next_event().await {
            println!("{:?}", ev);
        }
    }
}
```
