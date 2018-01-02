#[macro_use]
extern crate lazy_static;
extern crate quicksilver;

use quicksilver::graphics::{Canvas, Color, WindowBuilder};
use quicksilver::geom::Rectangle;
use std::sync::Mutex;

lazy_static! {
    static ref CANVAS: Mutex<Option<Canvas>> = Mutex::new(None);
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let (_, canvas) = WindowBuilder::new()
        .build("WASM", 800, 600);
    *CANVAS.lock().unwrap() = Some(canvas);
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    match *CANVAS.lock().unwrap() {
        Some(ref mut canvas) => {
            canvas.draw_rect(Rectangle::newi_sized(100, 100), Color::green());
            canvas.present()
        },
        None => ()
    }
}

fn main() {}
