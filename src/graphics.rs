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

use std::iter;

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
    geom_mode_changes: Vec<(usize, GeometryMode)>,
}

const VERTEX_SIZE: usize = 8;

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
            geom_mode_changes: Vec::new(),
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
        let offset = self.vertex_data.len() / VERTEX_SIZE;

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
                Element::Line([a, b]) => {
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
            insert_if_changed(&mut self.geom_mode_changes, (index, &mode), GeometryMode::eq);
        }
    }

    pub fn draw_point(&mut self, pos: Vector2<f32>, color: Color) {
        let vertex = Vertex {
            pos,
            uv: Some(Vector2 { x: -1.0, y: -1.0 }),
            color,
        };
        self.draw_elements(iter::once(vertex), iter::once(Element::Point(0)), None);
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh) {
        self.draw_elements(
            mesh.vertices.iter().cloned(),
            mesh.group.elements.iter().cloned(),
            mesh.group.image.as_ref()
        );
    }

    pub fn fill_polygon(&mut self, points: &[Vector2<f32>], color: Color) {
        assert!(points.len() >= 3);
        let vertices = points.iter().cloned().map(|pos| Vertex { pos, uv: None, color });
        let len = points.len() as u32;
        let indices = (0..(len - 2))
            .map(|idx| Element::Triangle([0, idx + 1, idx + 2]));
        self.draw_elements(vertices, indices, None);
    }

    pub fn stroke_path(&mut self, points: &[Vector2<f32>], color: Color) {
        let vertices = points.iter().cloned().map(|pos| Vertex { pos, uv: None, color });
        let len = points.len() as u32;
        let indices = (0..(len - 1))
            .map(|idx| Element::Line([idx, idx + 1]));
        self.draw_elements(vertices, indices, None);
    }
    
    pub fn stroke_polygon(&mut self, points: &[Vector2<f32>], color: Color) {
        assert!(points.len() >= 3);
        let vertices = points.iter().cloned().map(|pos| Vertex { pos, uv: None, color });
        let len = points.len() as u32;
        let indices = (0..len)
            .map(|idx| Element::Line([idx, (idx + 1) % len]));
        self.draw_elements(vertices, indices, None);
    }

    fn rect_to_poly(rect: &Rect) -> [Vector2<f32>; 4] {
        [
            rect.min,
            Vector2 { x: rect.min.x, y: rect.max.y },
            rect.max,
            Vector2 { x: rect.max.x, y: rect.min.y },
        ]
    }

    pub fn fill_rect(&mut self, rect: &Rect, color: Color) {
        self.fill_polygon(&Self::rect_to_poly(rect), color);
    }
    
    pub fn stroke_rect(&mut self, rect: &Rect, color: Color) {
        self.stroke_polygon(&Self::rect_to_poly(rect), color);
    }

    pub fn stroke_circle(&mut self, center: Vector2<f32>, radius: f32, color: Color) {
        self.stroke_polygon(&Self::circle_points(center, radius)[..], color);
    }

    pub fn fill_circle(&mut self, center: Vector2<f32>, radius: f32, color: Color) {
        self.fill_polygon(&Self::circle_points(center, radius)[..], color);
    }
    
    fn circle_points<'a>(center: Vector2<f32>, radius: f32) -> [Vector2<f32>; CIRCLE_LEN] {
        let mut points = CIRCLE_POINTS.clone();
        for point in points.iter_mut() {
            *point = Vector2 {
                x: center.x + radius * point.x,
                y: center.y + radius * point.y,
            }
        }
        points
    }

    pub fn draw_image(&mut self, image: &Image, top_left: Vector2<f32>) {
        let region = Rect {
            min: Vector2 { x: 0.0, y: 0.0 },
            max: image.size(),
        };
        self.draw_subimage_tinted(image, region, top_left, Color::WHITE);
    }

    pub fn draw_image_tinted(&mut self, image: &Image, top_left: Vector2<f32>, tint: Color) {
        let region = Rect {
            min: Vector2 { x: 0.0, y: 0.0 },
            max: image.size(),
        };
        self.draw_subimage_tinted(image, region, top_left, tint);
    }

    pub fn draw_subimage(&mut self, image: &Image, region: Rect, top_left: Vector2<f32>) {
        self.draw_subimage_tinted(image, region, top_left, Color::WHITE);
    }
    
    pub fn draw_subimage_tinted(&mut self, image: &Image, region: Rect, top_left: Vector2<f32>, tint: Color) {
        let size = image.size();
        // Calculate the region of the image to draw
        let min_u = region.min.x / size.x;
        let min_v = region.min.y / size.y;
        let max_u = region.max.x / size.x;
        let max_v = region.max.y / size.y;
        // Calculate how big to draw it
        let region_width = region.max.x - region.min.x;
        let region_height = region.max.y - region.min.y;
        let vertices = [
            Vertex {
                pos: top_left,
                uv: Some(Vector2 { x: min_u, y: min_v }),
                color: tint,
            },
            Vertex {
                pos: Vector2 { x: top_left.x + region_width, y: top_left.y },
                uv: Some(Vector2 { x: max_u, y: min_v }),
                color: tint,
            },
            Vertex {
                pos: Vector2 { x: top_left.x + region_width, y: top_left.y + region_height },
                uv: Some(Vector2 { x: max_u, y: max_v }),
                color: tint,
            },
            Vertex {
                pos: Vector2 { x: top_left.x, y: top_left.y + region_height },
                uv: Some(Vector2 { x: min_u, y: max_v }),
                color: tint,
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
        self.shader.set_uniform("image", UniformValue::Int(TEX_BIND_POINT as i32))?;
        let mut previous = 0;
        let mut element_mode = GeometryMode::Triangles;
        let change_list = join_change_lists(
            join_change_lists(self.image_changes.drain(..), self.projection_changes.drain(..)),
            self.geom_mode_changes.drain(..)
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
    iter::from_fn(move || {
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

const CIRCLE_LEN: usize = 63;

const CIRCLE_POINTS: [Vector2<f32>; CIRCLE_LEN] = [
    Vector2 { x: 1.0, y: 0.0 },
    Vector2 { x: 0.9950307753654014, y: 0.09956784659581666 },
    Vector2 { x: 0.9801724878485438, y: 0.19814614319939758 },
    Vector2 { x: 0.9555728057861407, y: 0.2947551744109042 },
    Vector2 { x: 0.9214762118704076, y: 0.38843479627469474 },
    Vector2 { x: 0.8782215733702285, y: 0.47825397862131824 },
    Vector2 { x: 0.8262387743159949, y: 0.5633200580636221 },
    Vector2 { x: 0.766044443118978, y: 0.6427876096865394 },
    Vector2 { x: 0.6982368180860729, y: 0.7158668492597184 },
    Vector2 { x: 0.6234898018587336, y: 0.7818314824680298 },
    Vector2 { x: 0.5425462638657594, y: 0.8400259231507714 },
    Vector2 { x: 0.4562106573531629, y: 0.8898718088114687 },
    Vector2 { x: 0.365341024366395, y: 0.9308737486442042 },
    Vector2 { x: 0.27084046814300516, y: 0.962624246950012 },
    Vector2 { x: 0.17364817766693022, y: 0.9848077530122081 },
    Vector2 { x: 0.07473009358642417, y: 0.9972037971811801 },
    Vector2 { x: -0.024930691738072913, y: 0.9996891820008162 },
    Vector2 { x: -0.12434370464748516, y: 0.9922392066001721 },
    Vector2 { x: -0.22252093395631434, y: 0.9749279121818236 },
    Vector2 { x: -0.31848665025168454, y: 0.9479273461671317 },
    Vector2 { x: -0.41128710313061156, y: 0.9115058523116731 },
    Vector2 { x: -0.5000000000000002, y: 0.8660254037844385 },
    Vector2 { x: -0.58374367223479, y: 0.8119380057158564 },
    Vector2 { x: -0.6616858375968595, y: 0.7497812029677341 },
    Vector2 { x: -0.7330518718298263, y: 0.6801727377709194 },
    Vector2 { x: -0.7971325072229225, y: 0.6038044103254774 },
    Vector2 { x: -0.8532908816321556, y: 0.5214352033794981 },
    Vector2 { x: -0.900968867902419, y: 0.43388373911755823 },
    Vector2 { x: -0.9396926207859084, y: 0.3420201433256685 },
    Vector2 { x: -0.969077286229078, y: 0.24675739769029342 },
    Vector2 { x: -0.9888308262251285, y: 0.14904226617617428 },
    Vector2 { x: -0.9987569212189223, y: 0.04984588566069704 },
    Vector2 { x: -0.9987569212189223, y: -0.04984588566069723 },
    Vector2 { x: -0.9888308262251285, y: -0.14904226617617447 },
    Vector2 { x: -0.969077286229078, y: -0.24675739769029362 },
    Vector2 { x: -0.9396926207859084, y: -0.34202014332566866 },
    Vector2 { x: -0.9009688679024191, y: -0.433883739117558 },
    Vector2 { x: -0.8532908816321555, y: -0.5214352033794983 },
    Vector2 { x: -0.7971325072229224, y: -0.6038044103254775 },
    Vector2 { x: -0.7330518718298262, y: -0.6801727377709195 },
    Vector2 { x: -0.6616858375968594, y: -0.7497812029677342 },
    Vector2 { x: -0.5837436722347898, y: -0.8119380057158565 },
    Vector2 { x: -0.4999999999999996, y: -0.8660254037844388 },
    Vector2 { x: -0.4112871031306116, y: -0.9115058523116731 },
    Vector2 { x: -0.3184866502516841, y: -0.9479273461671318 },
    Vector2 { x: -0.2225209339563146, y: -0.9749279121818236 },
    Vector2 { x: -0.12434370464748495, y: -0.9922392066001721 },
    Vector2 { x: -0.024930691738073156, y: -0.9996891820008162 },
    Vector2 { x: 0.07473009358642436, y: -0.9972037971811801 },
    Vector2 { x: 0.17364817766693083, y: -0.984807753012208 },
    Vector2 { x: 0.2708404681430051, y: -0.962624246950012 },
    Vector2 { x: 0.3653410243663954, y: -0.9308737486442041 },
    Vector2 { x: 0.45621065735316285, y: -0.8898718088114687 },
    Vector2 { x: 0.5425462638657597, y: -0.8400259231507713 },
    Vector2 { x: 0.6234898018587334, y: -0.7818314824680299 },
    Vector2 { x: 0.698236818086073, y: -0.7158668492597183 },
    Vector2 { x: 0.7660444431189785, y: -0.6427876096865389 },
    Vector2 { x: 0.8262387743159949, y: -0.563320058063622 },
    Vector2 { x: 0.8782215733702288, y: -0.4782539786213178 },
    Vector2 { x: 0.9214762118704076, y: -0.38843479627469474 },
    Vector2 { x: 0.9555728057861408, y: -0.2947551744109039 },
    Vector2 { x: 0.9801724878485438, y: -0.19814614319939772 },
    Vector2 { x: 0.9950307753654014, y: -0.09956784659581641 },
];
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
