extern crate gl;
extern crate imagefmt;
extern crate glutin;

pub mod geom;
pub mod graphics;
pub mod input;

mod manager;
pub use manager::AssetManager;

use input::Keyboard;
use graphics::Frontend;
use std::time::Duration;

pub trait State {
    fn new(&mut AssetManager) -> Self;
    fn tick(&mut self, frontend: &mut Frontend, keyboard: &Keyboard) -> ();
    fn get_tick_delay(&self) -> Duration;
}

pub fn run<T: State + Send + 'static>(title: &str, width: u32, height: u32) {
    use AssetManager;
    use geom::Rectangle;
    use graphics::{Backend, Bridge, Camera, Frontend};
    use std::sync::{Arc, Mutex};
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

    let bridge = Arc::new(Mutex::new(Bridge::new()));
    let mut backend = Backend::new();
    let rect = Rectangle::new_sized(width as f32, height as f32);
    let mut frontend = Frontend::new(bridge.clone(), Camera::new(rect, rect));
    let mut assets = AssetManager::new();
    let mut state = T::new(&mut assets);
    let running = Arc::new(Mutex::new(true));
    let keyboard = Arc::new(Mutex::new(Keyboard::new()));
    let update_keyboard = keyboard.clone();
    thread::spawn(move || {
        let keyboard = update_keyboard;
        loop {
            state.tick(&mut frontend, &keyboard.lock().unwrap());
            thread::sleep(state.get_tick_delay());
        }
    });
    let events_running = running.clone();
    thread::spawn(move || {
        let running = events_running;
        events_loop.run_forever(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput { device_id: _, input: event } => {
                        keyboard.lock().unwrap().process_event(&event);
                    },
                    glutin::WindowEvent::Closed => {
                        *(running.lock().unwrap()) = false;
                    },
                    _ => ()
                },
                _ => ()
            }
            glutin::ControlFlow::Continue
        });
    });
    loop {
        bridge.lock().unwrap().process_drawable(&mut backend, &gl_window);
        if !(*running.lock().unwrap()) {
            break;
        }
    }
}
