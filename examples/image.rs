extern crate quicksilver;

use quicksilver::geom::Vector;
use quicksilver::graphics::{Color, Image, WindowBuilder};

fn main() {
    let (mut window, mut canvas) = WindowBuilder::new()
        .with_clear_color(Color::white())
        .build("Basic Window", 800, 600);
    let image = Image::load("examples/image.png").unwrap();
    while window.poll_events() {
        canvas.draw_image(&image, Vector::new(100.0, 100.0));
        canvas.present(&window);
    }
}
