//! A module to draw 2D graphics in a window
//!  It also includes image loading

use crate::QuicksilverError;

mod color;
mod image;
mod mesh;
mod projection;
mod vertex;

pub use self::color::Color;
pub use self::image::Image;
pub use self::mesh::Mesh;
pub use self::projection::orthographic;
pub use self::vertex::{DrawGroup, Element, Vertex};

use golem::*;
use crate::geom::*;
use mint::*;

pub use golem::ColorFormat as PixelFormat;

// TODO: should projection be handled GPU-side?
// TODO: image views

pub struct Graphics {
    ctx: Context,
    vb: VertexBuffer,
    eb: ElementBuffer,
    shader: ShaderProgram,
    vertex_data: Vec<f32>,
    index_data: Vec<u32>,
    image_changes: Vec<(usize, Image)>,
    projection_changes: Vec<(usize, ColumnMatrix3<f32>)>,
    geometry_mode: Vec<(usize, GeometryMode)>,
}

fn insert_if_changed<T: Clone>(
    buffer: &mut Vec<(usize, T)>,
    (index, value): (usize, &T),
    are_eq: impl FnOnce(&T, &T) -> bool
) {
    let insert = match buffer.last() {
        Some((_, buf_value)) => !are_eq(buf_value, value),
        None => true,
    };
    if insert {
        buffer.push((index, value.clone()));
    }
}

fn join_change_lists<'a, U, V> (
    u: impl 'a + Iterator<Item = (usize, U)>,
    v: impl 'a + Iterator<Item = (usize, V)>
) -> impl 'a + Iterator<Item = (usize, (Option<U>, Option<V>))> {
    let mut u = u.peekable();
    let mut v = v.peekable();
    std::iter::from_fn(move || {
        match (u.peek(), v.peek()) {
            (None, None) => None,
            (Some(_), None) => {
                let (idx, u_val) = u.next().expect("peek indicated an element");
                Some((idx, (Some(u_val), None)))
            },
            (None, Some(_)) => {
                let (idx, v_val) = v.next().expect("peek indicated an element");
                Some((idx, (None, Some(v_val))))
            },
            (Some((u_idx, _)), Some((v_idx, _))) => {
                if u_idx <= v_idx {
                    let (idx, u_val) = u.next().expect("peek indicated an element");
                    Some((idx, (Some(u_val), None)))
                } else {
                    let (idx, v_val) = v.next().expect("peek indicated an element");
                    Some((idx, (None, Some(v_val))))
                }
            }
        }
    })
}


impl Graphics {
    pub(crate) fn new(ctx: Context) -> Result<Graphics, QuicksilverError> {
        use Dimension::*;
        let mut shader = ShaderProgram::new(&ctx, ShaderDescription {
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
            fragment_shader:
            r#" void main() {
                vec4 tex = vec4(1);
                if(frag_uv.x >= 0.0 && frag_uv.y >= 0.0) {
                    tex = texture(image, frag_uv);
                }
                gl_FragColor = tex * frag_color;
            }"#
        })?;
        let vb = VertexBuffer::new(&ctx)?;
        let eb = ElementBuffer::new(&ctx)?;
        shader.bind();

        Ok(Graphics {
            ctx,
            shader,
            vb,
            eb,
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            image_changes: Vec::new(),
            projection_changes: Vec::new(),
            geometry_mode: Vec::new(),
        })
    }

    pub fn clear(&mut self, color: Color) {
        self.ctx.set_clear_color(color.r, color.g, color.b, color.a);
        self.ctx.clear();
    }

    pub fn set_projection(&mut self, transform: ColumnMatrix3<f32>) {
        let head = self.index_data.len();
        self.projection_changes.push((head, transform));
    }

    pub fn draw_elements(&mut self, vertices: impl Iterator<Item = Vertex>, elements: impl Iterator<Item = Element>, image: Option<&Image>) {
        // We need to offset every triangle
        // In the input, the 0th index is the 0th provided vertex
        // In the GL buffer, the 0th index will be the first vertex we ever inserted
        // The number of vertices we've inserted is the length over the size of one insertion
        let offset = self.vertex_data.len() / 8;

        for vertex in vertices {
            let uv = vertex.uv.unwrap_or(Vector2 { x: -1.0, y: -1.0 });
            self.vertex_data.extend_from_slice(&[
                vertex.color.r, vertex.color.g, vertex.color.b, vertex.color.a,
                vertex.pos.x, vertex.pos.y,
                uv.x, uv.y,
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
                Element::Lines([a, b]) => {
                    self.index_data.extend_from_slice(&[a + tri_offset, b + tri_offset]);
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
            insert_if_changed(&mut self.geometry_mode, (index, &mode), GeometryMode::eq);
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        self.draw_elements(
            mesh.vertices.iter().cloned(),
            mesh.group.elements.iter().cloned(),
            mesh.group.image.as_ref()
        );
    }

    pub fn draw_polygon(&mut self, points: &[Vector2<f32>], color: Color) {
        assert!(points.len() >= 3);
        let vertices = points.iter().cloned().map(|pos| Vertex { pos, uv: None, color });
        let indices = (0..(points.len() - 2))
            .map(|idx| idx as u32)
            .map(|idx| Element::Triangle([0, idx + 1, idx + 2]));
        self.draw_elements(vertices, indices, None);
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.draw_polygon(&[
            rect.min,
            Vector2 { x: rect.min.x, y: rect.max.y },
            rect.max,
            Vector2 { x: rect.max.x, y: rect.min.y },
        ], color);
    }

    pub fn draw_image(&mut self, image: &Image, top_left: Vector2<f32>) {
        let size = image.size();
        let vertices = [
            Vertex {
                pos: top_left,
                uv: Some(Vector2 { x: 0.0, y: 0.0 }),
                color: Color::WHITE,
            },
            Vertex {
                pos: Vector2 { x: top_left.x + size.x, y: top_left.y },
                uv: Some(Vector2 { x: 1.0, y: 0.0 }),
                color: Color::WHITE,
            },
            Vertex {
                pos: Vector2 { x: top_left.x + size.x, y: top_left.y + size.y },
                uv: Some(Vector2 { x: 1.0, y: 1.0 }),
                color: Color::WHITE,
            },
            Vertex {
                pos: Vector2 { x: top_left.x, y: top_left.y + size.y },
                uv: Some(Vector2 { x: 0.0, y: 1.0 }),
                color: Color::WHITE,
            },
        ];
        let indices = [
            Element::Triangle([0, 1, 2]),
            Element::Triangle([2, 3, 0]),
        ];
        self.draw_elements(
            vertices.iter().cloned(),
            indices.iter().cloned(),
            Some(image)
        );
    }

    pub fn flush(&mut self) -> Result<(), QuicksilverError> {
        println!("{:?}", self.vertex_data);
        println!("{:?}", self.index_data);
        const TEX_BIND_POINT: u32 = 1;
        // TODO: check that all indices are valid
        if self.vertex_data.len() > self.vb.size() || self.index_data.len() > self.eb.size() {
            self.vb.set_data(self.vertex_data.as_slice());
            self.eb.set_data(self.index_data.as_slice());
            self.shader.prepare_draw(&self.vb, &self.eb)?;
        } else {
            self.vb.set_sub_data(0, self.vertex_data.as_slice());
            self.eb.set_sub_data(0, self.index_data.as_slice());
        }
        self.shader.set_uniform("image", UniformValue::Int(0))?;
        let mut previous = 0;
        let mut element_mode = GeometryMode::Triangles;
        let change_list = join_change_lists(
            join_change_lists(self.image_changes.drain(..), self.projection_changes.drain(..)),
            self.geometry_mode.drain(..)
        );
        for (index, changes) in change_list {
            println!("{:?}", index);
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
                    let matrix = projection.into();
                    self.shader.set_uniform("projection", UniformValue::Matrix3(matrix))?;
                } 
            }
            // If we're switching the element mode, do so now
            if let Some(g_m) = changes.1 {
                element_mode = g_m;
            }
        }
        if previous != self.index_data.len() {
            unsafe {
                self.shader.draw_prepared(previous..self.index_data.len(), element_mode);
            }
        }
        self.vertex_data.clear();
        self.index_data.clear();

        Ok(())
    }


    pub fn present(&mut self, win: &blinds::Window) -> Result<(), QuicksilverError> {
        self.flush()?;
        win.present();

        Ok(())
    }

    pub(crate) fn create_image(&self, data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<Texture, GolemError> {
        let mut texture = Texture::new(&self.ctx)?;
        texture.set_image(Some(data), width, height, format);

        Ok(texture)
    }
}
/*
mod animation;
mod atlas;
mod blend_mode;
mod color;
mod drawable;
#[cfg(feature="fonts")] mod font;
#[cfg(feature="lyon")] mod lyon;
mod image;
mod image_scale_strategy;
#[cfg(feature="immi")] mod immi;
mod mesh;
mod resize;
mod surface;
mod vertex;
mod view;

pub use self::{
    animation::Animation,
    atlas::{Atlas, AtlasError, AtlasItem},
    blend_mode::BlendMode,
    color::Color,
    drawable::{Background, Drawable},
    image::{Image, ImageError, PixelFormat},
    image_scale_strategy::ImageScaleStrategy,
    mesh::Mesh,
    resize::ResizeStrategy,
    surface::Surface,
    vertex::{Vertex, GpuTriangle},
    view::View,
};
#[cfg(feature="fonts")] pub use self::font::{Font, FontStyle};
#[cfg(feature="lyon")] pub use self::lyon::ShapeRenderer;
#[cfg(feature = "immi")] pub use self::immi::{create_immi_ctx, ImmiStatus, ImmiRender};
*/
