use crate::{
    Result,
    backend::{Backend, ImageData, SurfaceData, VERTEX_SIZE},
    geom::{Rectangle, Vector},
    error::QuicksilverError,
    graphics::{BlendMode, Color, GpuTriangle, Image, ImageScaleStrategy, PixelFormat, Surface, Vertex},
    input::MouseCursor,
};
use std::mem::size_of;
use stdweb::{
    web::{
        html_element::CanvasElement,
        TypedArray
    },
    unstable::TryInto
};
use webgl_stdweb::{
    WebGLBuffer,
    WebGLProgram,
    WebGLShader,
    WebGLTexture,
    WebGLUniformLocation
};
#[cfg(feature = "webgl1")]
use webgl_stdweb::WebGLRenderingContext as gl;
#[cfg(not(feature = "webgl1"))]
use webgl_stdweb::WebGL2RenderingContext as gl;
use stdweb::web::document;

pub struct WebGLBackend {
    canvas: CanvasElement,
    gl_ctx: gl,
    texture: Option<u32>,
    vertices: Vec<f32>,
    indices: Vec<u32>, 
    vertex_length: usize, 
    index_length: usize, 
    shader: WebGLProgram, 
    fragment: WebGLShader, 
    vertex: WebGLShader, 
    vbo: WebGLBuffer, 
    ebo: WebGLBuffer, 
    texture_location: Option<WebGLUniformLocation>,
    texture_mode: u32,
    initial_width: u32,
    initial_height: u32,
    textures: Vec<Option<WebGLTexture>>,
}

const DEFAULT_VERTEX_SHADER: &str = r#"attribute vec2 position;
attribute vec2 tex_coord;
attribute vec4 color;
attribute lowp float uses_texture;
varying vec2 Tex_coord;
varying vec4 Color;
varying lowp float Uses_texture;
void main() {
    gl_Position = vec4(position, 0, 1);
    Tex_coord = tex_coord;
    Color = color;
    Uses_texture = uses_texture;
}"#;

const DEFAULT_FRAGMENT_SHADER: &str = r#"varying highp vec4 Color;
varying highp vec2 Tex_coord;
varying lowp float Uses_texture;
uniform sampler2D tex;
void main() {
    highp vec4 tex_color = (int(Uses_texture) != 0) ? texture2D(tex, Tex_coord) : vec4(1, 1, 1, 1);
    gl_FragColor = Color * tex_color;
}"#;

fn format_gl(format: PixelFormat) -> u32 {
    match format {
        PixelFormat::RGB => gl::RGB,
        PixelFormat::RGBA => gl::RGBA
    }
}

fn try_opt<T>(opt: Option<T>, operation: &str) -> Result<T> {
    match opt {
        Some(val) => Ok(val),
        None => {
            let mut error = String::new();
            error.push_str("WebGL2 operation failed: ");
            error.push_str(operation);
            Err(QuicksilverError::ContextError(error))
        }
    }
}

impl Backend for WebGLBackend {
    type Platform = CanvasElement;

    unsafe fn new(canvas: CanvasElement, texture_mode: ImageScaleStrategy, _multisample: bool) -> Result<WebGLBackend> {
        let gl_ctx: gl = match canvas.get_context() {
            Ok(ctx) => ctx,
            _ => return Err(QuicksilverError::ContextError("Could not create WebGL2 context".to_owned()))
        };
        if cfg!(feature = "webgl1") && gl_ctx.get_extension::<webgl_stdweb::OES_element_index_uint>().is_none() {
            return Err(QuicksilverError::ContextError("Could not aquire OES_element_index_uint extension.".to_owned()))
        }
        let texture_mode = match texture_mode {
            ImageScaleStrategy::Pixelate => gl::NEAREST,
            ImageScaleStrategy::Blur => gl::LINEAR
        };
        let vbo = try_opt(gl_ctx.create_buffer(), "Create vertex buffer")?;
        let ebo = try_opt(gl_ctx.create_buffer(), "Create index buffer")?;
        gl_ctx.bind_buffer(gl::ARRAY_BUFFER, Some(&vbo));
        gl_ctx.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        gl_ctx.blend_func_separate(
            gl::SRC_ALPHA,
            gl::ONE_MINUS_SRC_ALPHA,
            gl::ONE,
            gl::ONE_MINUS_SRC_ALPHA,
        );
        gl_ctx.enable(gl::BLEND);
        let vertex = try_opt(gl_ctx.create_shader(gl::VERTEX_SHADER), "Create vertex shader")?;
        gl_ctx.shader_source(&vertex, DEFAULT_VERTEX_SHADER);
        gl_ctx.compile_shader(&vertex);
        let fragment = try_opt(gl_ctx.create_shader(gl::FRAGMENT_SHADER), "Create fragment shader")?;
        gl_ctx.shader_source(&fragment, DEFAULT_FRAGMENT_SHADER);
        gl_ctx.compile_shader(&fragment);
        let shader = try_opt(gl_ctx.create_program(), "Create shader program")?;
        gl_ctx.attach_shader(&shader, &vertex);
        gl_ctx.attach_shader(&shader, &fragment);
        gl_ctx.link_program(&shader);
        gl_ctx.use_program(Some(&shader));
        let initial_width = canvas.width();
        let initial_height = canvas.height();
        Ok(WebGLBackend {
            canvas,
            gl_ctx,
            texture: None,
            vertices: Vec::with_capacity(1024),
            indices: Vec::with_capacity(1024), 
            vertex_length: 0, 
            index_length: 0, 
            shader, fragment, vertex, vbo, ebo, 
            texture_location: None,
            texture_mode,
            initial_width,
            initial_height,
            textures: Vec::new(),
        })
    }

    unsafe fn clear(&mut self, col: Color) {
        self.gl_ctx.clear_color(col.r, col.g, col.b, col.a);
        self.gl_ctx.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    unsafe fn set_blend_mode(&mut self, blend: BlendMode) {
        self.gl_ctx.blend_func(gl::ONE, gl::ONE);
        self.gl_ctx.blend_equation_separate(blend as u32, gl::FUNC_ADD);
    }

    unsafe fn reset_blend_mode(&mut self) {
        self.gl_ctx.blend_func_separate(
            gl::SRC_ALPHA,
            gl::ONE_MINUS_SRC_ALPHA,
            gl::ONE,
            gl::ONE_MINUS_SRC_ALPHA,
        );
        self.gl_ctx.blend_equation_separate(gl::FUNC_ADD, gl::FUNC_ADD);
    }

    unsafe fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]) -> Result<()> {
        // Turn the provided vertex data into stored vertex data
        vertices.iter().for_each(|vertex| {
            self.vertices.push(vertex.pos.x);
            self.vertices.push(vertex.pos.y);
            let tex_pos = vertex.tex_pos.unwrap_or(Vector::ZERO);
            self.vertices.push(tex_pos.x);
            self.vertices.push(tex_pos.y);
            self.vertices.push(vertex.col.r);
            self.vertices.push(vertex.col.g);
            self.vertices.push(vertex.col.b);
            self.vertices.push(vertex.col.a);
            self.vertices.push(if vertex.tex_pos.is_some() { 1f32 } else { 0f32 });
        });
        let vertex_length = size_of::<f32>() * self.vertices.len();
        // If the GPU can't store all of our data, re-create the GPU buffers so they can
        if vertex_length > self.vertex_length {
            self.vertex_length = vertex_length * 2;
            // Create the vertex array
            self.gl_ctx.buffer_data(gl::ARRAY_BUFFER, self.vertex_length as i64, gl::STREAM_DRAW);
            let stride_distance = (VERTEX_SIZE * size_of::<f32>()) as i32;
            // Set up the vertex attributes
            let pos_attrib = self.gl_ctx.get_attrib_location(&self.shader, "position") as u32;
            self.gl_ctx.enable_vertex_attrib_array(pos_attrib);
            self.gl_ctx.vertex_attrib_pointer(pos_attrib, 2, gl::FLOAT, false, stride_distance, 0);
            let tex_attrib = self.gl_ctx.get_attrib_location(&self.shader, "tex_coord") as u32;
            self.gl_ctx.enable_vertex_attrib_array(tex_attrib);
            self.gl_ctx.vertex_attrib_pointer(tex_attrib, 2, gl::FLOAT, false, stride_distance, 2 * size_of::<f32>() as i64);
            let col_attrib = self.gl_ctx.get_attrib_location(&self.shader, "color") as u32;
            self.gl_ctx.enable_vertex_attrib_array(col_attrib);
            self.gl_ctx.vertex_attrib_pointer(col_attrib, 4, gl::FLOAT, false, stride_distance, 4 * size_of::<f32>() as i64);
            let use_texture_attrib = self.gl_ctx.get_attrib_location(&self.shader, "uses_texture") as u32;
            self.gl_ctx.enable_vertex_attrib_array(use_texture_attrib);
            self.gl_ctx.vertex_attrib_pointer(use_texture_attrib, 1, gl::FLOAT, false, stride_distance, 8 * size_of::<f32>() as i64);
            self.texture_location = Some(try_opt(self.gl_ctx.get_uniform_location(&self.shader, "tex"), "Get texture uniform")?);
        }
        // Upload all of the vertex data
        let array: TypedArray<f32> = self.vertices.as_slice().into();
        self.gl_ctx.buffer_sub_data(gl::ARRAY_BUFFER, 0, &array.buffer());
        // Scan through the triangles, adding the indices to the index buffer (every time the
        // texture switches, flush and switch the bound texture)
        for triangle in triangles.iter() {
            if let Some(ref img) = triangle.image {
                let should_flush = match self.texture {
                    Some(val) => img.get_id() != val,
                    None => true
                };
                if should_flush {
                    self.flush();
                }
                self.texture = Some(img.get_id());
            }
            self.indices.extend(triangle.indices.iter());
        }
        // Flush any remaining triangles
        self.flush();
        self.vertices.clear();
        Ok(())
    }

    unsafe fn flush(&mut self) {
        if self.indices.len() != 0 {
            // Check if the index buffer is big enough and upload the data
            let index_length = size_of::<u32>() * self.indices.len();
            if index_length > self.index_length {
                self.index_length = index_length * 2;
                self.gl_ctx.buffer_data(gl::ELEMENT_ARRAY_BUFFER, self.index_length as i64, gl::STREAM_DRAW);
            }
            let array: TypedArray<u32> = self.indices.as_slice().into();
            self.gl_ctx.buffer_sub_data(gl::ELEMENT_ARRAY_BUFFER, 0, &array.buffer());
            // Upload the texture to the GPU
            self.gl_ctx.active_texture(gl::TEXTURE0);
            if let Some(index) = self.texture {
                self.gl_ctx.bind_texture(gl::TEXTURE_2D, self.textures[index as usize].as_ref());
                self.gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.texture_mode as i32);
                self.gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.texture_mode as i32);
            }
            match self.texture_location {
                Some(ref location) => self.gl_ctx.uniform1i(Some(location), 0),
                None => self.gl_ctx.uniform1i(None, 0)
            }
            // Draw the triangles
            self.gl_ctx.draw_elements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0);
        }
        self.indices.clear();
        self.texture = None;
    }

    unsafe fn create_texture(&mut self, data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<ImageData> {
        let id = self.textures.len() as u32;
        let format = format_gl(format) as i64;
        let maybe_data =
            if data.len() == 0 {
                None
            } else {
                Some(data)
            };
        let texture = try_opt(self.gl_ctx.create_texture(), "Create GL texture")?;
        self.gl_ctx.bind_texture(gl::TEXTURE_2D, Some(&texture));
        self.gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        self.gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        self.gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        self.gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        let format = format as u32;
        self.gl_ctx.tex_image2_d(gl::TEXTURE_2D, 0, format as i32, width as i32, height as i32, 0, format, gl::UNSIGNED_BYTE, maybe_data);
        self.gl_ctx.generate_mipmap(gl::TEXTURE_2D);
        self.gl_ctx.bind_texture(gl::TEXTURE_2D, None);
        self.textures.push(Some(texture));
        Ok(ImageData { id, width, height })
    }

    unsafe fn destroy_texture(&mut self, data: &mut ImageData) {
        self.gl_ctx.delete_texture(self.textures[data.id as usize].as_ref());
    }

    unsafe fn create_surface(&mut self, image: &Image) -> Result<SurfaceData> {
        let surface = SurfaceData {
            framebuffer: try_opt(self.gl_ctx.create_framebuffer(), "Create GL framebuffer")?
        };
        self.gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&surface.framebuffer));
        self.gl_ctx.framebuffer_texture2_d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, self.textures[image.get_id() as usize].as_ref(), 0);
        self.gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, None);
        Ok(surface)
    }
    
    unsafe fn bind_surface(&mut self, surface: &Surface) {
        self.gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&surface.data.framebuffer));
        self.gl_ctx.viewport(0, 0, surface.image.source_width() as i32, surface.image.source_height() as i32);
    }

    unsafe fn unbind_surface(&mut self, _surface: &Surface, viewport: &[i32]) {
        self.gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, None); 
        self.gl_ctx.viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
    }

    unsafe fn destroy_surface(&mut self, surface: &SurfaceData) {
        self.gl_ctx.delete_framebuffer(Some(&surface.framebuffer));
    }

    unsafe fn viewport(&self) -> [i32; 4] {
        let viewport_data = self.gl_ctx.get_parameter(gl::VIEWPORT);
        [
            js! { return @{&viewport_data}[0]; }.try_into().expect("Malformed GL viewport attribute"),
            js! { return @{&viewport_data}[1]; }.try_into().expect("Malformed GL viewport attribute"),
            js! { return @{&viewport_data}[2]; }.try_into().expect("Malformed GL viewport attribute"),
            js! { return @{&viewport_data}[3]; }.try_into().expect("Malformed GL viewport attribute"),
        ]
    }

    unsafe fn set_viewport(&mut self, area: Rectangle) {
        self.gl_ctx.viewport(
            area.x() as i32,
            area.y() as i32,
            area.width() as i32,
            area.height() as i32
        );
    }

    unsafe fn screenshot(&self, format: PixelFormat) -> (Vector, Vec<u8>) {
        let bytes_per_pixel = match format {
            PixelFormat::RGBA => 4,
            PixelFormat::RGB => 3
        };
        let format = format_gl(format);
        let [x, y, width, height] = self.viewport();
        let length = (width * height * bytes_per_pixel) as usize;
        let mut buffer: Vec<u8> = Vec::with_capacity(length);
        let pointer = buffer.as_slice();
        self.gl_ctx.read_pixels(x, y, width, height, format, gl::UNSIGNED_BYTE, Some(pointer));
        buffer.set_len(length);
        (Vector::new(width, height), buffer)
    }

    fn set_cursor(&mut self, cursor: MouseCursor) {
        js!( @{&self.canvas}.style.cursor = @{cursor.into_css_style()} );
    }

    fn set_title(&mut self, title: &str) {
        document().set_title(title);
    }

    fn present(&self) -> Result<()> { Ok(()) }

    fn set_fullscreen(&mut self, fullscreen: bool) -> Option<Vector> {
        let (width, height) = if fullscreen {
            let window = stdweb::web::window();
            (window.inner_width() as u32, window.inner_height() as u32)
        } else {
            (self.initial_width, self.initial_height)
        };
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        Some(Vector::new(width, height))
    }

    fn resize(&mut self, size: Vector) {
        self.canvas.set_width(size.x as u32);
        self.canvas.set_height(size.y as u32);
    }
}

impl Drop for WebGLBackend {
    fn drop(&mut self) { 
        self.gl_ctx.delete_program(Some(&self.shader));
        self.gl_ctx.delete_shader(Some(&self.fragment));
        self.gl_ctx.delete_shader(Some(&self.vertex));
        self.gl_ctx.delete_buffer(Some(&self.vbo));
        self.gl_ctx.delete_buffer(Some(&self.ebo));
    }
}

