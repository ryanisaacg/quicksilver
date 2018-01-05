extern crate quicksilver;

use quicksilver::graphics::{Canvas, Color, Image, WindowBuilder, Window};
use quicksilver::geom::Rectangle;
use quicksilver::input::{Key, Viewport};

pub struct State {
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
        self.canvas.present(&self.window);
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() -> *mut State {
    let (window, canvas) = WindowBuilder::new()
        .build("WASM", 800, 600);
    let image = Image::load("image.png").unwrap();
    let viewport = window.viewport().build(Rectangle::newi_sized(800, 600));
    Box::into_raw(Box::new(State { window, canvas, viewport, image }))
}

#[no_mangle]
pub unsafe extern "C" fn draw(state: *mut State) {
    let mut state = Box::from_raw(state);
    state.draw();
    Box::into_raw(state);
}

fn main() {}
