extern crate gl;
extern crate imagefmt;
extern crate sdl2;

pub mod geom;
pub mod graphics;

mod manager;
pub use manager::AssetManager;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

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

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(title, width, height)
        .opengl()
        .build()
        .unwrap();
    let canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();

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
        bridge.process_drawable(&mut backend, &canvas.window());
    }
}
