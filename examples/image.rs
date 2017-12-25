extern crate quicksilver;

use quicksilver::{Color, Image, WindowBuilder, Vector};

fn main() {
    let (mut window, mut canvas) = WindowBuilder::new()
        .with_clear_color(Color::white())
        .build("Basic Window", 800, 600);
    let image = Image::load("examples/image.png").unwrap();
    let area = image.area().translate(Vector::new(100.0, 100.0));
    while window.poll_events() {
        canvas.draw_image(&image, area);
        canvas.present(&window);
    }
}
