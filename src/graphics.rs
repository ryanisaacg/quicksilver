//! Draw 2D graphics to the screen
//!
//! The main type is [`Graphics`], which is provided to your application by [`run`]. It handles
//! drawing shapes via methods like [`Graphics::fill_rect`] and [`Graphics::stroke_rect`]. If the
//! existing drawing methods don't fit your needs, try [`Graphics::draw_elements`] for manual
//! control over the shapes drawn.
//!
//! For loading and drawing images, to the screen, use [`Image`].
//!
//! [`run`]: crate::lifecycle::run

use crate::QuicksilverError;

mod circle_points;
mod color;
mod mesh;
mod vertex;

pub use self::color::Color;
pub use self::mesh::Mesh;
pub use self::vertex::{Element, Vertex};

use crate::geom::*;
use golem::*;

pub use golem::ColorFormat as PixelFormat;

use std::cell::{Ref, RefCell};
use std::iter;
use std::path::Path;
use std::rc::Rc;

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
    projection_changes: Vec<(usize, Transform)>,
    geom_mode_changes: Vec<(usize, GeometryMode)>,
    transform: Transform,
}

const VERTEX_SIZE: usize = 8;

impl Graphics {
    pub(crate) fn new(ctx: Context) -> Result<Graphics, QuicksilverError> {
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
                ],
                vertex_shader: r#" void main() {
                vec3 transformed = projection * vec3(vert_position, 1.0);
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
            projection_changes: Vec::new(),
            geom_mode_changes: Vec::new(),
            transform: Transform::IDENTITY,
        })
    }

    /// Clear the screen to the given color
    pub fn clear(&mut self, color: Color) {
        self.ctx.set_clear_color(color.r, color.g, color.b, color.a);
        self.ctx.clear();
    }

    /// Set the projection matrix, which is applied to all vertices on the GPU
    ///
    /// Use this to determine the drawable area with [`Transform::orthographic`], which is
    /// important to handling resizes.
    pub fn set_projection(&mut self, transform: Transform) {
        let head = self.index_data.len();
        self.projection_changes.push((head, transform));
    }

    /// Set the transformation matrix, which is applied to all vertices on the CPU
    ///
    /// Use this to rotate, scale, or translate individual draws or small groups of draws
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
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
    pub fn fill_circle(&mut self, center: Vector, radius: f32, color: Color) {
        self.fill_polygon(&Self::circle_points(center, radius)[..], color);
    }

    /// Outline a circle with a given color
    pub fn stroke_circle(&mut self, center: Vector, radius: f32, color: Color) {
        self.stroke_polygon(&Self::circle_points(center, radius)[..], color);
    }

    fn circle_points(center: Vector, radius: f32) -> [Vector; circle_points::CIRCLE_LEN] {
        let mut points = circle_points::CIRCLE_POINTS;
        for point in points.iter_mut() {
            *point = Vector {
                x: center.x + radius * point.x,
                y: center.y + radius * point.y,
            }
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

    /// Send the accumulated draw data to the GPU
    ///
    /// Except when rendering to a [`Surface`], this should almost never be necessary for a user
    /// to call directly. Use [`Graphics::present`] to draw to the window instead.
    pub fn flush(&mut self, surface: Option<&Surface>) -> Result<(), QuicksilverError> {
        // Either bind a surface or draw directly to the default framebuffer, depending on the
        // argument
        match surface {
            Some(surface) => surface.0.bind(),
            None => golem::Surface::unbind(&self.ctx),
        }
        const TEX_BIND_POINT: u32 = 1;
        let max_index = (self.vertex_data.len() / VERTEX_SIZE) as u32;
        for index in self.index_data.iter() {
            assert!(*index < max_index, "Element index out of bounds: are you calling draw_elements with invalid index values?");
        }
        if self.vertex_data.len() > self.vb.size() || self.index_data.len() > self.eb.size() {
            self.vb.set_data(self.vertex_data.as_slice());
            self.eb.set_data(self.index_data.as_slice());
            self.shader.prepare_draw(&self.vb, &self.eb)?;
        } else {
            self.vb.set_sub_data(0, self.vertex_data.as_slice());
            self.eb.set_sub_data(0, self.index_data.as_slice());
        }
        self.shader
            .set_uniform("image", UniformValue::Int(TEX_BIND_POINT as i32))?;
        let mut previous = 0;
        let mut element_mode = GeometryMode::Triangles;
        let change_list = join_change_lists(
            join_change_lists(
                self.image_changes.drain(..),
                self.projection_changes.drain(..),
            ),
            self.geom_mode_changes.drain(..),
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
            if let Some(first) = changes.0 {
                // If we're switching what image to use, do so now
                if let Some(image) = first.0 {
                    let bind_point = std::num::NonZeroU32::new(TEX_BIND_POINT).unwrap();
                    image.raw().set_active(bind_point);
                }
                // If we're switching what projection to use, do so now
                if let Some(projection) = first.1 {
                    let matrix = Self::transform_to_gl(projection);
                    self.shader
                        .set_uniform("projection", UniformValue::Matrix3(matrix))?;
                }
            }
            // If we're switching the element mode, do so now
            if let Some(g_m) = changes.1 {
                element_mode = g_m;
            }
        }
        if previous != self.index_data.len() {
            unsafe {
                self.shader
                    .draw_prepared(previous..self.index_data.len(), element_mode);
            }
        }
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
    pub fn present(&mut self, win: &blinds::Window) -> Result<(), QuicksilverError> {
        self.flush(None)?;
        win.present();

        Ok(())
    }
}

/// A 2D image, stored on the GPU
///
/// See [`Graphics::draw_image`] to draw it
#[derive(Clone)]
pub struct Image(Rc<RefCell<Texture>>);

impl Image {
    fn new(texture: Texture) -> Image {
        Image(Rc::new(RefCell::new(texture)))
    }

    /// Create an image with a given width and height
    ///
    /// Either source the data from an array of bytes, or create a blank image.
    /// `format` determines how to interpet the bytes when creating the image
    pub fn from_raw(
        gfx: &Graphics,
        data: Option<&[u8]>,
        width: u32,
        height: u32,
        format: PixelFormat,
    ) -> Result<Image, GolemError> {
        let mut texture = Texture::new(&gfx.ctx)?;
        texture.set_image(data, width, height, format);

        Ok(Image::new(texture))
    }

    /// Create an image from an encoded image format
    ///
    /// JPEG and PNG are supported
    pub fn from_encoded_bytes(gfx: &Graphics, raw: &[u8]) -> Result<Image, QuicksilverError> {
        let img = image::load_from_memory(raw)?.to_rgba();
        let width = img.width();
        let height = img.height();
        Ok(Image::from_raw(
            gfx,
            Some(img.into_raw().as_slice()),
            width,
            height,
            PixelFormat::RGBA,
        )?)
    }

    /// Load an image from a file at the given path
    ///
    /// JPEG and PNG file formats are supported
    pub async fn load(gfx: &Graphics, path: impl AsRef<Path>) -> Result<Image, QuicksilverError> {
        let file_contents = platter::load_file(path).await?;
        Image::from_encoded_bytes(gfx, file_contents.as_slice())
    }

    /// Replace the backing data for the image, or create a blank image
    pub fn set_data(&mut self, data: Option<&[u8]>, width: u32, height: u32, color: ColorFormat) {
        self.0.borrow_mut().set_image(data, width, height, color);
    }

    /// Set the data for some region of this image, without clearing it
    pub fn set_sub_data(
        &self,
        data: &[u8],
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        color: ColorFormat,
    ) {
        self.raw().set_subimage(data, x, y, width, height, color);
    }

    pub(crate) fn raw(&self) -> Ref<Texture> {
        self.0.borrow()
    }

    pub(crate) fn ptr_eq(&self, other: &Image) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }

    /// Get the size of the image
    pub fn size(&self) -> Vector {
        Vector {
            x: self.raw().width() as f32,
            y: self.raw().height() as f32,
        }
    }

    /// Determine how the texture should scale down
    pub fn set_minification(&self, min: TextureFilter) {
        self.raw().set_minification(min);
    }

    /// Determine how the texture should scale up
    pub fn set_magnification(&self, max: TextureFilter) {
        self.raw().set_magnification(max);
    }

    /// Determine how the texture is wrapped horizontally
    pub fn set_wrap_h(&self, wrap: TextureWrap) {
        self.raw().set_wrap_h(wrap);
    }

    /// Determine how the texture is wrapped vertically
    pub fn set_wrap_v(&self, wrap: TextureWrap) {
        self.raw().set_wrap_v(wrap);
    }

    fn into_raw(self) -> Result<Texture, Rc<RefCell<Texture>>> {
        Ok(Rc::try_unwrap(self.0)?.into_inner())
    }
}

/// A Surface is the core struct for rendering to textures, or getting data from them.
///
/// If you want to render to a texture, [`attach`] it and then pass the surface to
/// [`Graphics::flush`].
/// 
/// If you want to get data from a texture, [`attach`] it and use [`Surface::screenshot`].
///
/// [`attach`]: Surface::attach
pub struct Surface(golem::Surface);

impl Surface {
    /// Create a Surface with an attached Image
    ///
    /// The image must not have any other references to it, or this function will return an error.
    pub fn new(gfx: &Graphics, attachment: Image) -> Result<Surface, QuicksilverError> {
        let tex = attachment
            .into_raw()
            .map_err(|_| QuicksilverError::SurfaceImageError)?;
        Ok(Surface(golem::Surface::new(&gfx.ctx, tex)?))
    }

    /// Use the attached image as the backing data for this Surface
    ///
    /// To either get the data for an image via [`Surface::screenshot`] or set it via
    /// [`Graphics::flush`], an image needs to be attached to this Surface.
    ///
    /// The image must not have any other references to it, or this function will return an error.
    ///
    /// It's generally faster to create one [`Surface`] per [`Image`], and only attach and
    /// [`detach`] when necessary.
    ///
    /// [`detach`]: Surface::detach
    pub fn attach(&mut self, attachment: Image) -> Result<(), QuicksilverError> {
        let tex = attachment
            .into_raw()
            .map_err(|_| QuicksilverError::SurfaceImageError)?;
        self.0.put_texture(tex);

        Ok(())
    }
    
    /// Take the Image out of this Surface
    ///
    /// To use the data that has been rendered to a Surface, its attachment has to be removed to
    /// avoid creating a loop (where the Image is both being drawn *from* and being drawn *to*.)
    pub fn detach(&mut self) -> Option<Image> {
        Some(Image::new(self.0.take_texture()?))
    }

    /// Get the pixel data of a given region of this surface
    pub fn screenshot(
        &self,
        gfx: &Graphics,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        format: ColorFormat,
    ) -> Vec<u8> {
        self.0.bind();
        let mut buffer = vec![0; (width * height * format.bytes_per_pixel()) as usize];
        self.0.get_pixel_data(x, y, width, height, format, &mut buffer[..]);
        golem::Surface::unbind(&gfx.ctx);

        buffer
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
