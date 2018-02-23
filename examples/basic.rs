// The most basic example- it should just open a black window and set the window title to Hello
// world!
#[macro_use]
extern crate quicksilver;

use quicksilver::State;
use quicksilver::graphics::{Window, WindowBuilder};

// An empty structure because we don't need to store any state
struct BlackScreen;

impl State for BlackScreen {
    fn configure() -> Window {
        // Create a Window with the title "Hello world!" that is 800 x 600 pixels
        WindowBuilder::new()
            .with_fullscreen(true)
            .build("Hello world!", 500, 300)
    }

   fn new() -> BlackScreen { BlackScreen }
}

//Run the application on both desktop and web
run!(BlackScreen);
