//! Draw 2D graphics to the screen
//!
//! The main type is [`Graphics`], which is provided to your application by [`run`]. It handles
//! drawing shapes via methods like [`Graphics::fill_rect`] and [`Graphics::stroke_rect`]. If the
//! existing drawing methods don't fit your needs, try [`Graphics::draw_elements`] for manual
//! control over the shapes drawn.
//!
//! For loading and drawing images, to the screen, use [`Image`].
//!
//! [`run`]: crate::run::run

use crate::QuicksilverError;

mod circle_points;
mod color;
#[cfg(feature = "font")]
mod font;
mod image;
mod mesh;
mod resize_handler;
mod surface;
mod vertex;

pub use self::color::Color;
#[cfg(feature = "ttf")]
pub use self::font::VectorFont;
#[cfg(feature = "font")]
pub use self::font::{FontRenderer, LayoutGlyph};
pub use self::image::Image;
pub use self::mesh::Mesh;
pub use self::resize_handler::ResizeHandler;
pub use self::surface::Surface;
pub use self::vertex::{Element, Vertex};

use crate::geom::*;
use crate::Window;
use golem::*;
use std::iter;
use std::mem::size_of;

pub use golem::ColorFormat as PixelFormat;

/// Options to configure custom blending pipelines
///
/// By default, pixels are blended based on the alpha of the new pixel. However, with
/// [`Graphics::set_blend_mode`], that can be changed.
pub mod blend {
    /// The overall state of the blend pipeline
    ///
    /// See [`Graphics::set_blend_mode`]
    ///
    /// [`Graphics::set_blend_mode`]: super::Graphics::set_blend_mode
    pub type BlendMode = golem::blend::BlendMode;

    pub use golem::blend::{
        BlendChannel, BlendEquation, BlendFactor, BlendFunction, BlendInput, BlendOperation,
    };
}

/// The struct that handles sending draw calls to the GPU
///
/// The basic flow of using `Graphics` is a loop of [`Graphics::clear`], draw, and [`Graphics::present`].
///
/// When drawing, keep in mind the projection and transformation. The projection is set by
/// [`Graphics::set_projection`], and usually [`Transform::orthographic`]. It is a transformation
/// applied to every single vertex, and it's advised to keep it constant as much as possible. The
/// transformation is used to rotate, scale, or translate a handful of draw calls, and is set by
/// [`Graphics::set_transform`].
///
/// For best performance, try to reduce unnecessary state changes. Sources of state changes include
/// changing the image you're drawing, changing the projection, or changing the type of geomety
/// you're drawing.
pub struct Graphics {
    ctx: Context,
    vb: VertexBuffer,
    eb: ElementBuffer,
    shader: ShaderProgram,
    vertex_data: Vec<f32>,
    index_data: Vec<u32>,
    image_changes: Vec<(usize, Image)>,
    view_changes: Vec<(usize, Transform)>,
    geom_mode_changes: Vec<(usize, GeometryMode)>,
    clear_changes: Vec<(usize, Color)>,
    blend_mode_changes: Vec<(usize, Option<blend::BlendMode>)>,
    transform: Transform,
    resize: ResizeHandler,
    world_size: Vector,
    projection: Transform,
}

const VERTEX_SIZE: usize = 8;

impl Graphics {
    pub(crate) fn new(ctx: Context, world_size: Vector) -> Result<Graphics, QuicksilverError> {
        use Dimension::*;
        let mut shader = ShaderProgram::new(
            &ctx,
            ShaderDescription {
                vertex_input: &[
                    Attribute::new("vert_color", AttributeType::Vector(D4)),
                    Attribute::new("vert_position", AttributeType::Vector(D2)),
                    Attribute::new("vert_uv", AttributeType::Vector(D2)),
                ],
                fragment_input: &[
                    Attribute::new("frag_color", AttributeType::Vector(D4)),
                    Attribute::new("frag_uv", AttributeType::Vector(D2)),
                ],
                uniforms: &[
                    Uniform::new("image", UniformType::Sampler2D),
                    Uniform::new("projection", UniformType::Matrix(D3)),
                    Uniform::new("view", UniformType::Matrix(D3)),
                ],
                vertex_shader: r#" void main() {
                vec3 transformed = projection * view * vec3(vert_position, 1.0);
                gl_Position = vec4(transformed.xy, 0, 1);
                frag_uv = vert_uv;
                frag_color = vert_color;
            }"#,
                fragment_shader: r#" void main() {
                vec4 tex = vec4(1);
                if(frag_uv.x >= 0.0 && frag_uv.y >= 0.0) {
                    tex = texture(image, frag_uv);
                }
                gl_FragColor = tex * frag_color;
            }"#,
            },
        )?;
        let vb = VertexBuffer::new(&ctx)?;
        let eb = ElementBuffer::new(&ctx)?;
        shader.bind();
        ctx.set_blend_mode(Some(Default::default()));

        Ok(Graphics {
            ctx,
            shader,
            vb,
            eb,
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            image_changes: Vec::new(),
            view_changes: Vec::new(),
            geom_mode_changes: Vec::new(),
            clear_changes: Vec::new(),
            blend_mode_changes: Vec::new(),
            transform: Transform::IDENTITY,
            resize: ResizeHandler::Fit {
                aspect_width: world_size.x,
                aspect_height: world_size.y,
            },
            world_size,
            projection: Transform::IDENTITY,
        })
    }

    /// Turn this high-level graphics object into a low-level graphics context
    ///
    /// It is by design that this is a one-way operation. In order for the graphics API to be safe,
    /// Quicksilver takes full control of the context and all shaders. If you want to use custom
    /// shaders or rendering setups, you can no longer use the high-level graphics API.
    ///
    /// The context returned is the context from the golem rendering library, which is the library
    /// Quicksilver's graphics stack is built on. The main advantage you gain is custom shaders, as
    /// well as being able to manage multiple different GPU buffers. See the
    /// [`golem`](https://crates/io/crates/golem) crate for more details.
    pub fn into_raw_context(self) -> Context {
        self.ctx
    }

    /// Clear the screen to the given color
    pub fn clear(&mut self, color: Color) {
        let head = self.index_data.len();
        self.clear_changes.push((head, color));
    }

    /// Set the view matrix, which is applied to all vertices on the GPU
    ///
    /// Most of the time, you won't need this at all. However, if you want to apply a change to a
    /// great many objects (screen shake, rotations, etc.) setting the view matrix is a good way to
    /// do that.
    pub fn set_view(&mut self, transform: Transform) {
        let head = self.index_data.len();
        self.view_changes.push((head, transform));
    }

    /// Set the transformation matrix, which is applied to all vertices on the CPU
    ///
    /// Use this to rotate, scale, or translate individual draws or small groups of draws.
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    /// Project a point from the screen to the world
    ///
    /// Use this when checking the mouse position against rendered objects, like a game or UI
    pub fn screen_to_camera(&self, window: &Window, position: Vector) -> Vector {
        let viewport = self.calculate_viewport(window);
        let mut projected = position - viewport.top_left();

        projected.x *= self.world_size.x / viewport.width();
        projected.y *= self.world_size.y / viewport.height();

        projected
    }

    /// Set the size of the virtual camera
    ///
    /// Regardless of the size of the actual window, the draw functions all work on a virtual
    /// camera size. By default, this is the initial size in your Settings. If you start at
    /// 400x300, a 400x300 Rectangle will fill the drawable area. If the Window is doubled in size,
    /// a 400x300 Rectangle will still fill the drawable area. This function changes the size of
    /// the 'virtual camera.'
    ///
    /// If you want to position a camera at an arbitrary point within world space, or apply
    /// rotations or scaling, use [`set_view`].
    ///
    /// [`set_view`]: Self::set_view
    pub fn set_camera_size(&mut self, size: Vector) {
        self.world_size = size;
    }

    /// Change how to respond to the window resizing
    ///
    /// The default method of handling resizes is `ResizeHandler::Fit`, which maximizes the area
    /// drawn on screen while maintaining aspect ratio. There are a variety of other
    /// [`ResizeHandler`] options to choose from.
    ///
    /// [`ResizeHandler`]: crate::graphics::ResizeHandler
    pub fn set_resize_handler(&mut self, resize: ResizeHandler) {
        self.resize = resize;
    }

    /// Set the blend mode, which determines how pixels mix when drawn over each other
    ///
    /// Pass `None` to disable blending entirely
    pub fn set_blend_mode(&mut self, blend_mode: Option<blend::BlendMode>) {
        let head = self.index_data.len();
        self.blend_mode_changes.push((head, blend_mode));
    }

    /// Draw a collection of vertices
    ///
    /// Elements determines how to interpret the vertices. While it is convenient to mix-and-match
    /// within a single call, be aware that this can incur a performance penalty.
    ///
    /// If any of the provided vertices reference an image, they will use the provided image.
    pub fn draw_elements(
        &mut self,
        vertices: impl Iterator<Item = Vertex>,
        elements: impl Iterator<Item = Element>,
        image: Option<&Image>,
    ) {
        // We need to offset every triangle
        // In the input, the 0th index is the 0th provided vertex
        // In the GL buffer, the 0th index will be the first vertex we ever inserted
        // The number of vertices we've inserted is the length over the size of one insertion
        let offset = self.vertex_data.len() / VERTEX_SIZE;

        for vertex in vertices {
            let uv = vertex.uv.unwrap_or(Vector { x: -1.0, y: -1.0 });
            let pos = self.transform * vertex.pos;
            self.vertex_data.extend_from_slice(&[
                vertex.color.r,
                vertex.color.g,
                vertex.color.b,
                vertex.color.a,
                pos.x,
                pos.y,
                uv.x,
                uv.y,
            ]);
        }

        // It's important to keep this above the next block:
        // the image change should apply to the whole shape, which means it needs the starting
        // element index
        if let Some(img) = image {
            let index = self.index_data.len();
            insert_if_changed(&mut self.image_changes, (index, img), |a, b| a.ptr_eq(b));
        }

        let tri_offset = offset as u32;
        for element in elements {
            // Get the index before the rest of the list
            let index = self.index_data.len();
            let mode = match element {
                Element::Point(a) => {
                    self.index_data.push(a + tri_offset);
                    GeometryMode::Points
                }
                Element::Line([a, b]) => {
                    self.index_data
                        .extend_from_slice(&[a + tri_offset, b + tri_offset]);
                    GeometryMode::Lines
                }
                Element::Triangle([a, b, c]) => {
                    self.index_data.extend_from_slice(&[
                        a + tri_offset,
                        b + tri_offset,
                        c + tri_offset,
                    ]);
                    GeometryMode::Triangles
                }
            };
            insert_if_changed(
                &mut self.geom_mode_changes,
                (index, &mode),
                GeometryMode::eq,
            );
        }
    }

    /// Draw a single, pixel-sized point
    pub fn draw_point(&mut self, pos: Vector, color: Color) {
        let vertex = Vertex {
            pos,
            uv: Some(Vector { x: -1.0, y: -1.0 }),
            color,
        };
        self.draw_elements(iter::once(vertex), iter::once(Element::Point(0)), None);
    }

    /// Draw a mesh, which is shorthand for passing the [`Mesh`]'s data to
    /// [`Graphics::draw_elements`]
    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        self.draw_elements(
            mesh.vertices.iter().cloned(),
            mesh.elements.iter().cloned(),
            mesh.image.as_ref(),
        );
    }

    /// Draw a filled-in polygon of a given color
    ///
    /// The provided points must form a clockwise or counter-clockwise set of points in a convex
    /// polygon
    pub fn fill_polygon(&mut self, points: &[Vector], color: Color) {
        assert!(points.len() >= 3);
        let vertices = points.iter().cloned().map(|pos| Vertex {
            pos,
            uv: None,
            color,
        });
        let len = points.len() as u32;
        let indices = (0..(len - 2)).map(|idx| Element::Triangle([0, idx + 1, idx + 2]));
        self.draw_elements(vertices, indices, None);
    }

    /// Draw a series of lines that connect the given points, in order
    pub fn stroke_path(&mut self, points: &[Vector], color: Color) {
        let vertices = points.iter().cloned().map(|pos| Vertex {
            pos,
            uv: None,
            color,
        });
        let len = points.len() as u32;
        let indices = (0..(len - 1)).map(|idx| Element::Line([idx, idx + 1]));
        self.draw_elements(vertices, indices, None);
    }

    /// Draw an outline of a polygon of a given color
    ///
    /// The provided points must form a clockwise or counter-clockwise set of points in a convex
    /// polygon
    pub fn stroke_polygon(&mut self, points: &[Vector], color: Color) {
        assert!(points.len() >= 3);
        let vertices = points.iter().cloned().map(|pos| Vertex {
            pos,
            uv: None,
            color,
        });
        let len = points.len() as u32;
        let indices = (0..len).map(|idx| Element::Line([idx, (idx + 1) % len]));
        self.draw_elements(vertices, indices, None);
    }

    fn rect_to_poly(rect: &Rectangle) -> [Vector; 4] {
        [
            rect.pos,
            rect.pos + rect.size.x_comp(),
            rect.pos + rect.size,
            rect.pos + rect.size.y_comp(),
        ]
    }

    /// Draw a filled-in rectangle of a given color
    pub fn fill_rect(&mut self, rect: &Rectangle, color: Color) {
        self.fill_polygon(&Self::rect_to_poly(rect), color);
    }

    /// Outline a rectangle with a given color
    pub fn stroke_rect(&mut self, rect: &Rectangle, color: Color) {
        self.stroke_polygon(&Self::rect_to_poly(rect), color);
    }

    /// Draw a filled-in circle of a given color
    pub fn fill_circle(&mut self, circle: &Circle, color: Color) {
        self.fill_polygon(&Self::circle_points(circle)[..], color);
    }

    /// Outline a circle with a given color
    pub fn stroke_circle(&mut self, circle: &Circle, color: Color) {
        self.stroke_polygon(&Self::circle_points(circle)[..], color);
    }

    fn circle_points(circle: &Circle) -> [Vector; circle_points::CIRCLE_LEN] {
        let mut points = circle_points::CIRCLE_POINTS;
        for point in points.iter_mut() {
            *point = circle.center() + (*point * circle.radius);
        }

        points
    }

    /// Drawn an image to the given area, stretching if necessary
    pub fn draw_image(&mut self, image: &Image, location: Rectangle) {
        let region = Rectangle::new_sized(image.size());
        self.draw_subimage_tinted(image, region, location, Color::WHITE);
    }

    /// Drawn a tinted image to the given area, stretching if necessary
    ///
    /// The tint is applied by multiplying the color components at each pixel. If the Color has
    /// (r, g, b, a) of (1.0, 0.5, 0.0, 1.0), all the pixels will have their normal red value, half
    /// their green value, and no blue value.
    pub fn draw_image_tinted(&mut self, image: &Image, location: Rectangle, tint: Color) {
        let region = Rectangle::new_sized(image.size());
        self.draw_subimage_tinted(image, region, location, tint);
    }

    /// Draw a given part of an image to the screen, see [`Graphics::draw_image`]
    pub fn draw_subimage(&mut self, image: &Image, region: Rectangle, location: Rectangle) {
        self.draw_subimage_tinted(image, region, location, Color::WHITE);
    }

    /// Draw a given part of a tinted image to the screen, see [`Graphics::draw_image_tinted`]
    pub fn draw_subimage_tinted(
        &mut self,
        image: &Image,
        region: Rectangle,
        location: Rectangle,
        tint: Color,
    ) {
        let size = image.size();
        // Calculate the region of the image to draw
        let size_recip = size.recip();
        let min_uv = region.pos.times(size_recip);
        let max_uv = (region.pos + region.size).times(size_recip);
        // Calculate how big to draw it
        let vertices = [
            Vertex {
                pos: location.pos,
                uv: Some(min_uv),
                color: tint,
            },
            Vertex {
                pos: location.pos + location.size.x_comp(),
                uv: Some(max_uv.x_comp() + min_uv.y_comp()),
                color: tint,
            },
            Vertex {
                pos: location.pos + location.size,
                uv: Some(max_uv),
                color: tint,
            },
            Vertex {
                pos: location.pos + location.size.y_comp(),
                uv: Some(max_uv.y_comp() + min_uv.x_comp()),
                color: tint,
            },
        ];
        let indices = [Element::Triangle([0, 1, 2]), Element::Triangle([2, 3, 0])];
        self.draw_elements(
            vertices.iter().cloned(),
            indices.iter().cloned(),
            Some(image),
        );
    }

    /// Draw to a Surface
    pub fn flush_surface(&mut self, surface: &Surface) -> Result<(), QuicksilverError> {
        if let (Some(width), Some(height)) = (surface.0.width(), surface.0.height()) {
            self.ctx.set_viewport(0, 0, width, height);
            let flip = Transform::scale(Vector::new(1.0, -1.0));
            let ortho = Transform::orthographic(Rectangle::new_sized(Vector::new(
                width as f32,
                height as f32,
            )));
            self.projection = flip * ortho;
        } else {
            return Err(QuicksilverError::NoSurfaceImageBound);
        }
        surface.0.bind();
        self.flush_gpu()?;
        Ok(())
    }

    /// Draw to the Window, without writing those changes to the screen
    pub fn flush_window(&mut self, window: &Window) -> Result<(), QuicksilverError> {
        self.projection = Transform::orthographic(Rectangle::new_sized(self.world_size));
        let viewport = self.calculate_viewport(window);
        let offset = viewport.top_left() * window.scale_factor();
        let size = viewport.size() * window.scale_factor();
        dbg!(offset);
        dbg!(size);
        self.ctx.set_viewport(
            offset.x as u32,
            offset.y as u32,
            size.x as u32,
            size.y as u32,
        );
        golem::Surface::unbind(&self.ctx);
        self.flush_gpu()?;
        Ok(())
    }

    fn calculate_viewport(&self, window: &Window) -> Rectangle {
        let size = self.resize.content_size(window.size());
        Rectangle::new((window.size() - size) / 2.0, size)
    }

    /// Send the accumulated draw data to the GPU
    ///
    /// Except when rendering to a [`Surface`], this should almost never be necessary for a user
    /// to call directly. Use [`Graphics::present`] to draw to the window instead. When rendering
    /// to a [`Surface`], remember to set the viewport via [`Graphics::set_viewport`]
    fn flush_gpu(&mut self) -> Result<(), QuicksilverError> {
        const TEX_BIND_POINT: u32 = 1;
        let max_index = (self.vertex_data.len() / VERTEX_SIZE) as u32;
        for index in self.index_data.iter() {
            assert!(*index < max_index, "Element index out of bounds: are you calling draw_elements with invalid index values?");
        }
        let vertex_data_size = self.vertex_data.len() * size_of::<f32>();
        let index_data_size = self.index_data.len() * size_of::<f32>();
        if vertex_data_size >= self.vb.size() || index_data_size >= self.eb.size() {
            self.vb.set_data(self.vertex_data.as_slice());
            self.eb.set_data(self.index_data.as_slice());
            self.shader.prepare_draw(&self.vb, &self.eb)?;
        } else {
            self.vb.set_sub_data(0, self.vertex_data.as_slice());
            self.eb.set_sub_data(0, self.index_data.as_slice());
        }

        self.shader
            .set_uniform("image", UniformValue::Int(TEX_BIND_POINT as i32))?;
        self.shader.set_uniform(
            "projection",
            UniformValue::Matrix3(Self::transform_to_gl(self.projection)),
        )?;

        let mut previous = 0;
        let mut element_mode = GeometryMode::Triangles;
        // We need to store the images while we're drawing them. Otherwise, their destructors will
        // run and they will get freed before the draw call goes through. That's bad.
        // So we hold the image if necessary
        let mut retained_image = None;
        let change_list = join_change_lists(
            join_change_lists(
                join_change_lists(
                    join_change_lists(self.image_changes.drain(..), self.view_changes.drain(..)),
                    self.geom_mode_changes.drain(..),
                ),
                self.clear_changes.drain(..),
            ),
            self.blend_mode_changes.drain(..),
        );
        for (index, changes) in change_list {
            // Before we change state, draw the old state
            if previous != index {
                unsafe {
                    self.shader.draw_prepared(previous..index, element_mode);
                }
                previous = index;
            }
            // Change the render state
            if let Some(changes) = changes.0 {
                if let Some(changes) = changes.0 {
                    if let Some(changes) = changes.0 {
                        // If we're switching what image to use, do so now
                        if let Some(image) = changes.0 {
                            let bind_point = std::num::NonZeroU32::new(TEX_BIND_POINT).unwrap();
                            image.raw().set_active(bind_point);
                            retained_image = Some(image);
                        }
                        // If we're switching what projection to use, do so now
                        if let Some(view) = changes.1 {
                            let matrix = Self::transform_to_gl(view);
                            self.shader
                                .set_uniform("view", UniformValue::Matrix3(matrix))?;
                        }
                    }
                    // If we're switching the element mode, do so now
                    if let Some(g_m) = changes.1 {
                        element_mode = g_m;
                    }
                }
                if let Some(color) = changes.1 {
                    self.ctx.set_clear_color(color.r, color.g, color.b, color.a);
                    self.ctx.clear();
                }
            }
            if let Some(blend_mode) = changes.1 {
                self.ctx.set_blend_mode(blend_mode);
            }
        }
        if previous != self.index_data.len() {
            unsafe {
                self.shader
                    .draw_prepared(previous..self.index_data.len(), element_mode);
            }
        }
        drop(retained_image); // Now we don't need the image anymore
        golem::Surface::unbind(&self.ctx);
        self.vertex_data.clear();
        self.index_data.clear();

        Ok(())
    }

    // Handle converting a row-matrix transformation to a column-major array
    fn transform_to_gl(trans: Transform) -> [f32; 9] {
        let matrix: mint::RowMatrix3<f32> = trans.into();
        let matrix: mint::ColumnMatrix3<f32> = matrix.into();

        matrix.into()
    }

    /// Send the draw data to the GPU and paint it to the Window
    ///
    /// On desktop, this will block until drawing has completed. If vsync is enabled, it will block
    /// until the frame completes. **Call this at the end of your frame.**
    pub fn present(&mut self, win: &Window) -> Result<(), QuicksilverError> {
        self.flush_window(win)?;
        win.present();

        Ok(())
    }
}

fn insert_if_changed<T: Clone>(
    buffer: &mut Vec<(usize, T)>,
    (index, value): (usize, &T),
    are_eq: impl FnOnce(&T, &T) -> bool,
) {
    let insert = match buffer.last() {
        Some((_, buf_value)) => !are_eq(buf_value, value),
        None => true,
    };
    if insert {
        buffer.push((index, value.clone()));
    }
}

fn join_change_lists<'a, U, V>(
    u: impl 'a + Iterator<Item = (usize, U)>,
    v: impl 'a + Iterator<Item = (usize, V)>,
) -> impl 'a + Iterator<Item = (usize, (Option<U>, Option<V>))> {
    let mut u = u.peekable();
    let mut v = v.peekable();
    iter::from_fn(move || match (u.peek(), v.peek()) {
        (None, None) => None,
        (Some(_), None) => {
            let (idx, u_val) = u.next().expect("peek indicated an element");
            Some((idx, (Some(u_val), None)))
        }
        (None, Some(_)) => {
            let (idx, v_val) = v.next().expect("peek indicated an element");
            Some((idx, (None, Some(v_val))))
        }
        (Some((u_idx, _)), Some((v_idx, _))) => {
            if u_idx <= v_idx {
                let (idx, u_val) = u.next().expect("peek indicated an element");
                Some((idx, (Some(u_val), None)))
            } else {
                let (idx, v_val) = v.next().expect("peek indicated an element");
                Some((idx, (None, Some(v_val))))
            }
        }
    })
}
