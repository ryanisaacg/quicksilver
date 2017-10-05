extern crate gl;
extern crate imagefmt;
extern crate glutin;

pub mod geom;
pub mod graphics;

mod manager;
pub use manager::AssetManager;

use graphics::Frontend;
use std::time::Duration;

pub trait State {
    fn new(&mut AssetManager) -> Self;
    fn tick(&mut self, frontend: &mut Frontend) -> ();
    fn get_tick_delay(&self) -> Duration;
}

pub fn run<T: State>(title: &str, width: u32, height: u32) {
    use AssetManager;
    use geom::Rectangle;
    use graphics::{Backend, Bridge, Camera, Frontend};
    use std::thread;
    use glutin::GlContext;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(width, height);
    let context = glutin::ContextBuilder::new()
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }
    
    let bridge = Bridge::new();
    let mut backend = Backend::new();
    let rect = Rectangle::new_sized(width as f32, height as f32);
    let mut frontend = Frontend::new(bridge.get_front(), Camera::new(rect, rect));
    thread::spawn(move || {
        let mut assets = AssetManager::new();
        let mut state = T::new(&mut assets);
        loop {
            state.tick(&mut frontend);
            thread::sleep(state.get_tick_delay());
        }
    });
    loop {
        bridge.process_drawable(&mut backend, &gl_window);
    }
}
