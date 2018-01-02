extern crate quicksilver;

use quicksilver::graphics::WindowBuilder;

fn main() {
    let (mut window, _) = WindowBuilder::new().build("Basic Window", 800, 600);
    while window.poll_events() {
    }
}
