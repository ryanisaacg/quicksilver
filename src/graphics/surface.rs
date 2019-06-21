use crate::{
    Result,
    backend::{Backend, SurfaceData, instance},
    geom::{Transform, Vector},
    graphics::{Image, PixelFormat, View},
    lifecycle::Window,
};
use std::rc::Rc;

#[derive(Clone, Debug)]
///A possible render target that can be drawn to the screen
pub struct Surface {
    pub(crate) image: Image,
    pub(crate) data: Rc<SurfaceData>,
}

impl Surface {
    ///Create a new surface with a given width and height
    pub fn new(width: u32, height: u32) -> Result<Surface> {
        let image = Image::new_null(width, height, PixelFormat::RGBA)?;
        let data = unsafe {
            Rc::new(instance().create_surface(&image)?)
        };
        Ok(Surface {
            image,
            data
        })
    }

    ///Render data to the surface
    ///
    ///Do not attempt to use the surface or its image within the function, because it is undefined behavior
    pub fn render_to(&self, window: &mut Window, func: impl FnOnce(&mut Window) -> Result<()>) -> Result<()> {
        let backend = unsafe { instance() };
        let view = window.view();
        let viewport = unsafe {
            let view = backend.viewport();
            backend.bind_surface(self);
            view
        };
        window.flush()?;
        window.set_view(View::new_transformed(self.image.area(), Transform::scale(Vector::new(1, -1))));
        func(window)?;
        window.flush()?;
        window.set_view(view);
        unsafe {
            backend.unbind_surface(self, &viewport);
        }
        Ok(())
    }

    ///Get a reference to the Image that contains the data drawn to the Surface
    pub fn image(&self) -> &Image {
        &self.image
    }
}
