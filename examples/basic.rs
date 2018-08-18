// The most basic example- it should just open a black window and set the window title to Hello
// world!
extern crate quicksilver;

use quicksilver::{
    Result,
    geom::Vector,
    lifecycle::{Settings, State, run}
};

// An empty structure because we don't need to store any state
struct BlackScreen;

impl State for BlackScreen {
    fn new() -> Result<BlackScreen> {
        Ok(BlackScreen)
    }
}

fn main() {
    run::<BlackScreen>("Hello World", Vector::new(800, 600), Settings::default());
}
