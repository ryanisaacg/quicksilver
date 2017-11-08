extern crate gl;
extern crate imagefmt;
extern crate glutin;

mod assets;
mod geom;
mod graphics;
mod input;

pub use assets::*;
pub use geom::*;
pub use graphics::*;
pub use input::*;

use std::time::Duration;

pub trait State {
    fn new(&mut AssetManager, frontend: &mut Graphics) -> Self;
    fn tick(&mut self, keyboard: &Keyboard, mouse: &Mouse, builder: &ViewportBuilder) -> Duration;
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
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }

    let screen_size = Vector::new(width as f32, height as f32);
    let camera = Camera::new(Rectangle::newv_sized(screen_size));
    let mut frontend = Graphics::new(Box::new(GLBackend::new()), camera);
    let mut assets = AssetManager::new();
    let state = Arc::new(Mutex::new(T::new(&mut assets, &mut frontend)));
    let keyboard = Arc::new(Mutex::new(Keyboard::new()));
    let mouse = Arc::new(Mutex::new(Mouse::new()));
    let screen_size = Arc::new(Mutex::new(screen_size));
    let scale_factor = Arc::new(Mutex::new(gl_window.hidpi_factor()));
    let update_keyboard = keyboard.clone();
    let update_mouse = mouse.clone();
    let update_state = state.clone();
    let update_size = screen_size.clone();
    let update_factor = scale_factor.clone();
    let mut offset = Vector::zero();
    thread::spawn(move || {
        let keyboard = update_keyboard;
        let mouse = update_mouse;
        let state = update_state;
        let screen_size = update_size;
        let scale_factor = update_factor;
        loop {
            let delay = {
                let mut keyboard = keyboard.lock().unwrap();
                let mut mouse = mouse.lock().unwrap();
                let builder = ViewportBuilder::new(*screen_size.lock().unwrap() / *scale_factor.lock().unwrap());
                let delay = state.lock().unwrap().tick(&keyboard, &mouse, &builder);
                keyboard.clear_temporary_states();
                mouse.clear_temporary_states();
                delay
            };
            thread::sleep(delay);
        }
    });
    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input: event,
                    } => {
                        keyboard.lock().unwrap().process_event(&event);
                    }
                    glutin::WindowEvent::MouseMoved { position, .. } => {
                        let (x, y) = position;
                        let mut mouse = mouse.lock().unwrap();
                        let mut scale_factor = scale_factor.lock().unwrap();
                        *scale_factor = gl_window.hidpi_factor();
                        mouse.set_position(
                            Vector::new(x as f32, y as f32) - offset,
                           *scale_factor 
                        );
                    }
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        mouse.lock().unwrap().process_button(state, button);
                    }
                    glutin::WindowEvent::Closed => {
                        running = false;
                    }
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
                        {
                            *screen_size.lock().unwrap() = Vector::new(w as f32, h as f32);
                        }
                        offset = Vector::newi(offset_x, offset_y);
                        unsafe {
                            gl::Viewport(offset_x, offset_y, w, h);
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        });
        state.lock().unwrap().draw(&mut frontend);
        frontend.present(&gl_window);
        thread::sleep(Duration::from_millis(1));
    }
}
