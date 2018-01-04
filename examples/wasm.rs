#[macro_use]
extern crate lazy_static;
extern crate quicksilver;

use quicksilver::graphics::{Canvas, Color, Image, WindowBuilder, Window};
use quicksilver::geom::{Rectangle, Vector};
use quicksilver::input::{Key, Viewport};
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

struct State {
    window: Window,
    canvas: Canvas,
    viewport: Viewport,
    image: Image
}

impl State {
    pub fn draw(&mut self) {
        self.window.poll_events();
        if self.window.keyboard()[Key::A].is_down() {
            self.canvas.set_clear_color(Color::blue());
        } else {
            self.canvas.set_clear_color(Color::white());
        }
        self.canvas.clear();
        self.canvas.draw_rect(Rectangle::newi_sized(100, 100), Color::green());
        self.canvas.draw_image(&self.image, self.window.mouse(&self.viewport).pos);
        self.canvas.present();
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let (window, canvas) = WindowBuilder::new()
        .build("WASM", 800, 600);
    let image = Image::load("image.png").unwrap();
    let viewport = window.viewport().build(Rectangle::newi_sized(800, 600));
    *STATE.lock().unwrap() = Some(State { window, canvas, viewport, image });
}

#[no_mangle]
pub unsafe extern "C" fn draw() {
    match *STATE.lock().unwrap() {
        Some(ref mut state) => state.draw(),
        None => ()
    }
}

fn main() {}
