extern crate qs;
extern crate gl;
extern crate sdl2;

use qs::geom::{Rectangle, Vector, Transform};
use qs::graphics::{Backend, Bridge, Frontend, Camera, Color, Drawable, Texture, PixelFormat, WHITE};

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

    let texture = Texture::from_raw(&[255, 255, 255, 255], 1, 1, PixelFormat::RGBA);
    {
        let mut backend = Backend::new();
        let bridge = Bridge::new();
        let rect = Rectangle::new_sized(800f32, 600f32);
        let frontend = Frontend::new(bridge.get_front().clone(), Camera::new(rect, rect));
        let region = texture.region();
        loop {
            backend.clear(Color {r: 0f32, g: 1f32, b: 1f32, a: 1f32});
            backend.switch_texture(region.get_id());
            frontend.draw_image(region, Rectangle::new_sized(32f32, 32f32), Transform::identity(), WHITE);
            bridge.process_drawable(&mut backend);
            backend.flip();
            canvas.window().gl_swap_window();
        }
    }
}
