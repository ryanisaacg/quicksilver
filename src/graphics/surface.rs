use ffi::gl;
use geom::{Transform, Vector};
use graphics::{Image, PixelFormat, Window, View};
use std::rc::Rc;

struct SurfaceData {
    framebuffer: u32
}

impl Drop for SurfaceData {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffer(self.framebuffer) };
    }
}

#[derive(Clone)]
///A possible render target that can be drawn to the screen
pub struct Surface {
    image: Image,
    data: Rc<SurfaceData>,
}

impl Surface {
    ///Create a new surface with a given width and height
    pub fn new(width: i32, height: i32) -> Surface {
        let image = Image::new_null(width, height, PixelFormat::RGBA);
        let surface = SurfaceData {
            framebuffer: unsafe { gl::GenFramebuffer() }
        };
        unsafe { 
            gl::BindFramebuffer(gl::FRAMEBUFFER, surface.framebuffer);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, image.get_id(), 0);
            gl::DrawBuffer(gl::COLOR_ATTACHMENT0);
        }
        Surface {
            image,
            data: Rc::new(surface)
        }
    }

    ///Render data to the surface
    ///
    ///Do not attempt to use the surface or its image within the function, because it is undefined behavior
    pub fn render_to<F>(&self, func: F, window: &mut Window) where F: FnOnce(&mut Window) {
        let viewport = &mut [0, 0, 0, 0];
        let view = window.view();
        unsafe {
            gl::GetViewport(viewport.as_mut_ptr());
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.data.framebuffer);
            gl::Viewport(0, 0, self.image.source_width(), self.image.source_height());
            window.set_view(View::new_transformed(self.image.area(), Transform::scale(Vector::new(1, -1))));
        }
        func(window);
        window.set_view(view);
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); 
            gl::Viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
        }
    }

    ///Get a reference to the Image that contains the data drawn to the Surface
    pub fn image(&self) -> &Image {
        &self.image
    }
}
