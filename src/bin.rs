extern crate qs;
extern crate gl;
extern crate sdl2;

use qs::geom::Vector;
use qs::graphics::{Backend, Color, Texture, PixelFormat, Vertex, WHITE};

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap();
    let canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();

    let mut backend = Backend::new();
    let texture = Texture::from_raw(&[255, 255, 255, 255], 1, 1, PixelFormat::RGBA);
    let region = texture.region();
    loop {
        backend.clear(Color {r: 0f32, g: 1f32, b: 1f32, a: 1f32});
        backend.add(region.get_id(), 
                    &[Vertex {pos: Vector::new(0f32, 0f32), tex_pos: Vector::new(0f32, 0f32), col: WHITE},
                        Vertex {pos: Vector::new(1f32, 0f32), tex_pos: Vector::new(0f32, 0f32), col: WHITE},
                        Vertex {pos: Vector::new(1f32, 1f32), tex_pos: Vector::new(0f32, 0f32), col: WHITE}], 
                    &[0, 1, 2]);
        backend.flip();
        canvas.window().gl_swap_window();
    }
}
