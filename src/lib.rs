extern crate gl;
extern crate imagefmt;
extern crate glutin;

pub mod geom;
pub mod graphics;
pub mod input;

mod manager;
pub use manager::AssetManager;

use input::{Keyboard, Mouse};
use graphics::Graphics;
use std::time::Duration;

pub trait State {
    fn new(&mut AssetManager) -> Self;
    fn tick(&mut self, keyboard: &Keyboard, mouse: &Mouse) -> Duration;
    fn draw(&mut self, frontend: &mut Graphics);
}

pub fn run<T: State + Send + 'static>(title: &str, width: u32, height: u32) {
    use AssetManager;
    use geom::*;
    use graphics::*;
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

    let mut backend = GLBackend::new();
    let rect = Rectangle::new_sized(width as f32, height as f32);
    let mut frontend = Graphics::new(&mut backend, &gl_window, Camera::new(rect, rect));
    let mut assets = AssetManager::new();
    let state = Arc::new(Mutex::new(T::new(&mut assets)));
    let keyboard = Arc::new(Mutex::new(Keyboard::new()));
    let mouse = Arc::new(Mutex::new(Mouse::new()));
    let update_keyboard = keyboard.clone();
    let update_mouse = mouse.clone();
    let update_state = state.clone();
    thread::spawn(move || {
        let keyboard = update_keyboard;
        let mouse = update_mouse;
        let state = update_state;
        loop {
            let delay = {
                let mut keyboard = keyboard.lock().unwrap();
                let mut mouse = mouse.lock().unwrap();
                let delay = state.lock().unwrap().tick(&keyboard, &mouse);
                keyboard.clear_temporary_states();
                mouse.clear_temporary_states();
                delay
            };
            thread::sleep(delay);
        }
    });
    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::KeyboardInput { device_id: _, input: event } => {
                        keyboard.lock().unwrap().process_event(&event);
                    },
                    glutin::WindowEvent::MouseMoved { position, .. } => {
                        mouse.lock().unwrap().set_position(position);
                    },
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        mouse.lock().unwrap().process_button(state, button);
                    },
                    glutin::WindowEvent::Closed => {
                        running = false;
                    },
                    glutin::WindowEvent::Resized(new_width, new_height) => {
                        let target_ratio = width as f32 / height as f32;
                        let window_ratio = new_width as f32 / new_height as f32;
                        let (w, h) = if window_ratio > target_ratio {
                            ((target_ratio * new_height as f32) as i32, new_height as i32)
                        } else if window_ratio < target_ratio {
                            (new_width as i32, (new_width as f32 / target_ratio) as i32)
                        } else {
                            (new_width as i32, new_height as i32)
                        };
                        let offset_x = (new_width as i32 - w) / 2;
                        let offset_y = (new_height as i32 - h) / 2;
                        unsafe {
                            gl::Viewport(offset_x, offset_y, w, h);
                        }
                    },
                    _ => ()
                },
                _ => ()
            }
        });
        state.lock().unwrap().draw(&mut frontend);
        thread::sleep(Duration::from_millis(1));
    }
}
