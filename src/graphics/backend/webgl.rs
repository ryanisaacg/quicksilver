use geom::Vector;
use graphics::{
    backend::{Backend, BlendMode, ImageScaleStrategy, VERTEX_SIZE},
    Color, GpuTriangle, Image, PixelFormat, Vertex
};
use std::{
    ffi::CString,
    mem::size_of,
    os::raw::c_void,
    ptr::null,
    str::from_utf8
};
use webgl_stdweb::{
    WebGLBuffer,
    WebGLProgram,
    WebGL2RenderingContext,
    WebGLShader,
    WebGLTexture,
    WebGLUniformLocation
};


pub struct WebGLBackend {
    texture: Image,
    vertices: Vec<f32>,
    indices: Vec<u32>, 
    null: Image, 
    vertex_length: usize, 
    index_length: usize, 
    shader: WebGLProgram, 
    fragment: WebGLShader, 
    vertex: WebGLShader, 
    vbo: WebGLBuffer, 
    ebo: WebGLBuffer, 
    texture_location: WebGLUniformLocation,
    texture_mode: u32,
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

fn context() -> WebGL2RenderingContext {
    js! ( render_context ).try_into().into();
}

impl Backend for WebGLBackend {
    fn new(texture_mode: ImageScaleStrategy) -> WebGLBackend {
        let canvas: CanvasElement = document().query_selector("#canvas").unwrap().unwrap().try_into().unwrap();
        let gl_ctx: WebGL2RenderingContext = canvas.get_context().unwrap();
        js! { 
            render_context = @{&gl_ctx};
            render_context.texture_count = 0;
        }
        let texture_mode = match texture_mode {
            ImageScaleStrategy::Pixelate => WebGLRenderingContext::NEAREST,
            ImageScaleStrategy::Blur => WebGLRenderingContext::LINEAR
        };
        let vbo = gl_ctx.create_buffer().unwrap();
        let ebo = gl_ctx.create_buffer().unwrap();
        gl_ctx.bind_buffer(WebGLRenderingContext::ARRAY_BUFFER, Some(&vbo));
        gl_ctx.bind_buffer(WebGLRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        gl_ctx.blend_func(WebGLRenderingContext::SRC_ALPHA, WebGLRenderingContext::ONE_MINUS_SRC_ALPHA);
        gl_ctx.enable(WebGLRenderingContext::BLEND);
        let null = Image::new_null(1, 1, PixelFormat::RGBA);
        let texture = null.clone();
        let vertex = gl_ctx.create_shader(WebGLRenderingContext::VERTEX_SHADER);
        gl_ctx.shader_source(vertex, CString::new(DEFAULT_VERTEX_SHADER).unwrap().into_raw());
        gl_ctx.compile_shader(vertex);
        let fragment = gl_ctx.create_shader(WebGLRenderingContext::FRAGMENT_SHADER);
        gl_ctx.shader_source(fragment, CString::new(DEFAULT_FRAGMENT_SHADER).unwrap().into_raw());
        gl_ctx.compile_shader(fragment);
        let shader = gl_ctx.create_program();
        gl_ctx.attach_shader(shader, vertex);
        gl_ctx.attach_shader(shader, fragment);
        gl_ctx.link_program(shader);
        gl_ctx.use_program(shader);
        WebGLBackend {
            texture,
            vertices: Vec::with_capacity(1024),
            indices: Vec::with_capacity(1024), 
            null,
            vertex_length: 0, 
            index_length: 0, 
            shader, fragment, vertex, vbo, ebo, 
            texture_location: 0,
            texture_mode,
            texture_count: 1
        }
    }
    
    fn clear(&mut self, col: Color) {
        let gl_ctx = context();
        gl_ctx.clear_color(col.r, col.g, col.b, col.a);
        gl_ctx.clear(WebGLRenderingContext::COLOR_BUFFER_BIT | WebGLRenderingContext::DEPTH_BUFFER_BIT);
    }

    fn set_blend_mode(&mut self, blend: BlendMode) {
        let gl_ctx = context();
        gl_ctx.blend_func(WebGLRenderingContext::ONE, WebGLRenderingContext::ONE);
        gl_ctx.blend_equation_separate(blend as u32, WebGLRenderingContext::FUNC_ADD);
    }

    fn reset_blend_mode(&mut self) {
        let gl_ctx = context();
        gl_ctx.blend_func(WebGLRenderingContext::SRC_ALPHA, WebGLRenderingContext::ONE_MINUS_SRC_ALPHA);
        gl_ctx.blend_equation_separate(WebGLRenderingContext::FUNC_ADD, WebGLRenderingContext::FUNC_ADD);
    }

    fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]) {
        let gl_ctx = context();
        // Turn the provided vertex data into stored vertex data
        vertices.iter().for_each(|vertex| {
            self.vertices.push(vertex.pos.x);
            self.vertices.push(vertex.pos.y);
            let tex_pos = vertex.tex_pos.unwrap_or(Vector::zero());
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
            // Create strings for all of the shader attributes
            let position_string = CString::new("position").unwrap().into_raw();
            let tex_coord_string = CString::new("tex_coord").unwrap().into_raw();
            let color_string = CString::new("color").unwrap().into_raw();
            let tex_string = CString::new("tex").unwrap().into_raw();
            let use_texture_string = CString::new("uses_texture").unwrap().into_raw();
            // Create the vertex array
            gl_ctx.buffer_data(WebGLRenderingContext::ARRAY_BUFFER, self.vertex_length as i64, null(), WebGLRenderingContext::STREAM_DRAW);
            let stride_distance = (VERTEX_SIZE * size_of::<f32>()) as i32;
            // Set up the vertex attributes
            let pos_attrib = gl_ctx.get_attrib_location(self.shader, position_string as *const i8) as u32;
            gl_ctx.enable_vertex_attrib_array(pos_attrib);
            gl_ctx.vertex_attrib_pointer(pos_attrib, 2, WebGLRenderingContext::FLOAT, WebGLRenderingContext::FALSE, stride_distance, null());
            let tex_attrib = gl_ctx.get_attrib_location(self.shader, tex_coord_string as *const i8) as u32;
            gl_ctx.enable_vertex_attrib_array(tex_attrib);
            gl_ctx.vertex_attrib_pointer(tex_attrib, 2, WebGLRenderingContext::FLOAT, WebGLRenderingContext::FALSE, stride_distance, (2 * size_of::<f32>()) as *const c_void);
            let col_attrib = gl_ctx.get_attrib_location(self.shader, color_string as *const i8) as u32;
            gl_ctx.enable_vertex_attrib_array(col_attrib);
            gl_ctx.vertex_attrib_pointer(col_attrib, 4, WebGLRenderingContext::FLOAT, WebGLRenderingContext::FALSE, stride_distance, (4 * size_of::<f32>()) as *const c_void);
            let use_texture_attrib = gl_ctx.get_attrib_location(self.shader, use_texture_string as *const i8) as u32;
            gl_ctx.enable_vertex_attrib_array(use_texture_attrib);
            gl_ctx.vertex_attrib_pointer(use_texture_attrib, 1, WebGLRenderingContext::FLOAT, WebGLRenderingContext::FALSE, stride_distance, (8 * size_of::<f32>()) as *const c_void);
            self.texture_location = gl_ctx.get_uniform_location(self.shader, tex_string as *const i8);
        }
        // Upload all of the vertex data
        let vertex_data = self.vertices.as_ptr() as *const c_void;
        gl_ctx.buffer_sub_data(WebGLRenderingContext::ARRAY_BUFFER, 0, vertex_length as i64, vertex_data);
        // Scan through the triangles, adding the indices to the index buffer (every time the
        // texture switches, flush and switch the bound texture)
        for triangle in triangles.iter() {
            if let Some(ref img) = triangle.image {
                if self.texture.get_id() != self.null.get_id() && self.texture.get_id() != img.get_id() {
                    self.flush();
                }
                self.texture = img.clone();
            }
            self.indices.extend(triangle.indices.iter());
        }
        // Flush any remaining triangles
        self.flush();
        self.vertices.clear();
    }

    fn flush(&mut self) {
        let gl_ctx = context();
        if self.indices.len() != 0 {
            // Check if the index buffer is big enough and upload the data
            let index_length = size_of::<u32>() * self.indices.len();
            let index_data = self.indices.as_ptr() as *const c_void;
            if index_length > self.index_length {
                self.index_length = index_length * 2;
                gl_ctx.buffer_data(WebGLRenderingContext::ELEMENT_ARRAY_BUFFER, self.index_length as i64, WebGLRenderingContext::STREAM_DRAW);
            }
            gl_ctx.buffer_sub_data(WebGLRenderingContext::ELEMENT_ARRAY_BUFFER, 0, index_length as i64, index_data);
            // Upload the texture to the GPU
            gl_ctx.active_texture(WebGLRenderingContext::TEXTURE0);
            if self.texture.get_id() != 0 {
                gl_ctx.bind_texture(WebGLRenderingContext::TEXTURE_2D, Some(&self.texture.data().data));
                gl_ctx.tex_parameteri(WebGLRenderingContext::TEXTURE_2D, WebGLRenderingContext::TEXTURE_MIN_FILTER, self.texture_mode as i32);
                gl_ctx.tex_parameteri(WebGLRenderingContext::TEXTURE_2D, WebGLRenderingContext::TEXTURE_MAG_FILTER, self.texture_mode as i32);
            }
            gl_ctx.uniform1i(Some(&self.texture_location), 0);
            // Draw the triangles
            gl_ctx.draw_elements(WebGLRenderingContext::TRIANGLES, self.indices.len() as i32, WebGLRenderingContext::UNSIGNED_INT, 0);
        }
        self.indices.clear();
        self.texture = self.null.clone();
    }

    fn create_texture(data: &[u8], width: u32, height: u32, format: PixelFormat) -> ImageData where Self: Sized {
        let gl_ctx = context();
        let id = js! {
            var index = render_context.texture_count;
            render_context.texture_count += 1;
            return index;
        };
        let format = match format {
            PixelFormat::RGB => WebGLRenderingContext::RGB as i64,
            PixelFormat::RGBA => WebGLRenderingContext::RGBA as i64,
            PixelFormat::BGR => WebGLRenderingContext::BGR as i64,
            PixelFormat::BGRA => WebGLRenderingContext::BGRA as i64,
        };
        let data =  gl_ctx.create_texture();
        gl_ctx.bind_texture(WebGLRenderingContext::TEXTURE_2D, Some(&texture));
        gl_ctx.tex_parameteri(WebGLRenderingContext::TEXTURE_2D, WebGLRenderingContext::TEXTURE_WRAP_S, WebGLRenderingContext::CLAMP_TO_EDGE as i32);
        gl_ctx.tex_parameteri(WebGLRenderingContext::TEXTURE_2D, WebGLRenderingContext::TEXTURE_WRAP_T, WebGLRenderingContext::CLAMP_TO_EDGE as i32);
        gl_ctx.tex_parameteri(WebGLRenderingContext::TEXTURE_2D, WebGLRenderingContext::TEXTURE_MIN_FILTER, WebGLRenderingContext::NEAREST as i32);
        gl_ctx.tex_parameteri(WebGLRenderingContext::TEXTURE_2D, WebGLRenderingContext::TEXTURE_MAG_FILTER, WebGLRenderingContext::NEAREST as i32);
        let data = if data.len() == 0 {
            gl_ctx.tex_image2_d(WebGLRenderingContext::TEXTURE_2D, 0, WebGLRenderingContext::RGBA as i32, width as i32, 
                        height as i32, 0, format as u32, WebGLRenderingContext::UNSIGNED_BYTE, None);
        } else {
            let width = width as i32;
            let height = height as i32;
            let format = format as u32;
            let slice = unsafe { UnsafeTypedArray::new(data) };
            js! {
                let data = @{slice};
                let gl = @{gl_ctx};
                gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, @{width}, @{height}, 0, @{format}, gl.UNSIGNED_BYTE, data);
            }
        }
        gl_ctx.generate_mipmap(WebGLRenderingContext::TEXTURE_2D);
        ImageData { id, data, width, height }
    }

    fn destroy_texture(data: &mut ImageData) where Self: Sized {
        let gl_ctx = context();
        gl_ctx.delete_texture(Some(&data.data));
    }

    fn create_surface(image: &Image) -> SurfaceData where Self: Sized {
        let gl_ctx = context();
    }
    
    fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized {
        let gl_ctx = context();
        let mut viewport = [0, 0, 0, 0];
        gl::GetViewport((&mut viewport).as_mut_ptr());
        gl::BindFramebuffer(gl::FRAMEBUFFER, surface.data.framebuffer);
        gl::Viewport(0, 0, surface.image.source_width() as i32, surface.image.source_height() as i32);
        viewport
    }

    fn unbind_surface(surface: &Surface, viewport: &[i32]) where Self: Sized {
        let gl_ctx = context();
        gl_ctx.bind_framebuffer(WebGLRenderingContext::FRAMEBUFFER, None); 
        gl_ctx.viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
    }

    fn destroy_surface(surface: &SurfaceData) where Self: Sized {
        context().delete_framebuffer(Some(&surface.framebuffer));
    }

    fn viewport(x: i32, y: i32, width: i32, height: i32) where Self: Sized {
        context().viewport(x, y, width, height);
    }
}

impl Drop for WebGLBackend {
    fn drop(&mut self) { 
        let gl_ctx = context();
        gl_ctx.delete_program(Some(&self.shader));
        gl_ctx.delete_shader(Some(&self.fragment));
        gl_ctx.delete_shader(Some(&self.vertex));
        gl_ctx.delete_buffer(Some(&self.vbo));
        gl_ctx.delete_buffer(Some(&self.ebo));
    }
}

