extern crate gl;
extern crate image;
extern crate glutin;
extern crate tiled;

mod geom;
mod graphics;
mod input;
mod timer;

pub use geom::*;
pub use graphics::*;
pub use input::*;
pub use timer::Timer;
pub use std::time::Duration;

pub trait State {
    fn configure(WindowBuilder) -> WindowBuilder;
    fn new() -> Self;
    fn tick(&mut self, InputBuilder) -> Duration;
    fn draw(&mut self, &mut Window);
}

pub fn run<T: State>(title: &str, width: u32, height: u32) {
    let window_builder = T::configure(WindowBuilder::new());
    let mut window = Window::new(window_builder, title, width, height);
    let mut state = T::new();
    let mut keyboard = Keyboard::new();
    let mut mouse = Mouse::new();
    let mut timer = Timer::new();
    while window.running() {
        window.poll_events(&mut keyboard, &mut mouse);
        timer.tick(|| state.tick(InputBuilder { keyboard: &keyboard, mouse: mouse.clone(), viewport: window.viewport() }));
        state.draw(&mut window);
        window.present();
    }
}


