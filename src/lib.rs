#[cfg(not(target_arch="wasm32"))]
extern crate image;
#[cfg(not(target_arch="wasm32"))]
extern crate glutin;
#[cfg(not(target_arch="wasm32"))]
extern crate tiled;

mod assets;
mod geom;
mod gl;
mod graphics;
mod input;

pub use assets::*;
pub use geom::*;
pub use graphics::*;
pub use input::*;

use std::time::Duration;

pub trait State {
    fn new(&mut AssetManager, frontend: &mut Graphics) -> Self;
    fn tick(&mut self, input: InputBuilder) -> Duration;
    fn draw(&mut self, frontend: &mut Graphics);
}

pub struct UpdateInformation<'a> {
    pub keyboard: &'a Keyboard,
    pub mouse: &'a Mouse,
    pub builder: &'a ViewportBuilder
}

#[cfg(not(target_arch="wasm32"))]
pub fn run<T: State + Send + 'static>(title: &str, width: u32, height: u32) {
    use AssetManager;
    use geom::*;
    use graphics::*;

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

    let mut screen_size = Vector::new(width as f32, height as f32);
    let mut frontend = Graphics::new(Box::new(GLBackend::new()), Camera::new(Rectangle::newv_sized(screen_size)));
    let mut assets = AssetManager::new();
    let mut state = T::new(&mut assets, &mut frontend);
    let mut keyboard = Keyboard::new();
    let mut mouse = Mouse::new();
    let mut scale_factor = gl_window.hidpi_factor();
    let mut offset = Vector::zero();
    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input: event,
                    } => {
                        keyboard.process_event(&event);
                    }
                    glutin::WindowEvent::MouseMoved { position, .. } => {
                        let (x, y) = position;
                        scale_factor = gl_window.hidpi_factor();
                        mouse = mouse.with_position((Vector::new(x as f32, y as f32) - offset) / scale_factor);
                    }
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        mouse.process_button(state, button);
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
                        screen_size = Vector::new(w as f32, h as f32);
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
        let viewport = ViewportBuilder::new(screen_size / scale_factor);
        {
            let input_builder = InputBuilder {
                keyboard: &keyboard,
                mouse: mouse.clone(),
                viewport
            };   
            state.tick(input_builder);
        }
        keyboard.clear_temporary_states();
        mouse.clear_temporary_states(); 
        state.draw(&mut frontend);
        frontend.present(&gl_window);
    }
}


