extern crate gl;
extern crate imagefmt;
extern crate glfw;

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

pub fn run<T: State + Send + 'static>(title: &str, width: u32, height: u32) {
    use AssetManager;
    use geom::Rectangle;
    use glfw::{Context, Window};
    use graphics::{Backend, Bridge, Camera, Frontend};
    use std::thread;
    
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|name| window.get_proc_address(name) as *const _);

    let bridge = Bridge::new();
    let mut backend = Backend::new();
    let rect = Rectangle::new_sized(width as f32, height as f32);
    let mut frontend = Frontend::new(bridge.get_front(), Camera::new(rect, rect));
    let mut assets = AssetManager::new();
    let mut state = T::new(&mut assets);
    thread::spawn(move || {
        loop {
            state.tick(&mut frontend);
            thread::sleep(state.get_tick_delay());
        }
    });
    loop {
        bridge.process_drawable(&mut backend, &mut window);
    }
}
