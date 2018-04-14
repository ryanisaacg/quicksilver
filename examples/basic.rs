// The most basic example- it should just open a black window and set the window title to Hello
// world!
extern crate quicksilver;

use quicksilver::{State, run, graphics::WindowBuilder};

// An empty structure because we don't need to store any state
struct BlackScreen;

impl State for BlackScreen {
    fn configure() -> WindowBuilder {
        // Create a Window with the title "Hello world!" that is 800 x 600 pixels
        WindowBuilder::new("Hello world!", 800, 600)
    }

   fn new() -> BlackScreen { BlackScreen }
}

fn main() {
    run::<BlackScreen>();
}
