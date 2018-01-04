#[macro_use]
extern crate lazy_static;
extern crate quicksilver;

use quicksilver::graphics::{Canvas, Color, Image, WindowBuilder};
use quicksilver::geom::{Rectangle, Vector};
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

struct State {
    canvas: Canvas,
    image: Image
}

impl State {
    pub fn draw(&mut self) {
        self.canvas.set_clear_color(Color::white());
        self.canvas.clear();
        self.canvas.draw_rect(Rectangle::newi_sized(100, 100), Color::green());
        self.canvas.draw_image(&self.image, Vector::newi(100, 100));
        self.canvas.present();
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let (_, canvas) = WindowBuilder::new()
        .build("WASM", 800, 600);
    let image = Image::load("image.png").unwrap();
    *STATE.lock().unwrap() = Some(State { canvas, image });
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    match *STATE.lock().unwrap() {
        Some(ref mut state) => state.draw(),
        None => ()
    }
}

fn main() {}
