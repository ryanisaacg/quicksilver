extern crate quicksilver;

fn main() {
    let (mut window, _) = quicksilver::WindowBuilder::new().build("Basic Window", 800, 600);
    while window.poll_events() {
    }
}
