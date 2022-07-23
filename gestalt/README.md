# gestalt

Cross-platform configuration and data saving between desktop and web

On desktop, saving is backed by filesystem and APIs and uses the platform-specific data
locations. On web, saving is backed by the LocalStorage browser API.
As an end user, all you need to worry about is which `Location` you want to save to:
- `Cache`, which is short-lived and may not persist between runs of the program
- `Config`, for storing long-term configuration
- `Data`, for storing long-term large data blobs.

To save and load some data:

```rust
use gestalt::{Location, save, load};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Player {
name: String,
score: u32
}

let player1 = Player { name: "Bob".to_string(), score: 21 };
save(Location::Cache, "mygame", "player1", &player1).expect("Could not save Player 1");

let player2 = Player { name: "Alice".to_string(), score: 200 };
save(Location::Cache, "mygame", "player2", &player2).expect("Could not save Player 2");

// Now reload.
let player1 = load::<Player>(Location::Cache, "mygame", "player1").expect("Could not load Player 1");
let player2 = load::<Player>(Location::Cache, "mygame", "player2").expect("Could not load Player 2");
```
