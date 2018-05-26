// The most basic example- it should just open a black window and set the window title to Hello
// world!
extern crate quicksilver;

use quicksilver::{State, run, graphics::WindowBuilder};

// An empty structure because we don't need to store any state
struct BlackScreen;

impl State for BlackScreen {
   fn new() -> BlackScreen { BlackScreen }
}

fn main() {
    // Create a Window with the title "Hello world!" that is 800 x 600 pixels
    run::<BlackScreen>(WindowBuilder::new("Hello world!", 800, 600));
}
