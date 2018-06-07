use geom::Vector;
use graphics::{
    backend::{Backend, BlendMode, ImageScaleStrategy, VERTEX_SIZE},
    Color, GpuTriangle, Image, ImageData, PixelFormat, Surface, SurfaceData, Vertex
};
use ffi::gl;
use std::{
    ffi::CString,
    mem::size_of,
    os::raw::c_void,
    ptr::null
};

pub struct GL3Backend {
    texture: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>, 
    null: Image, 
    vertex_length: usize, 
    index_length: usize, 
    shader: u32, 
    fragment: u32, 
    vertex: u32, 
    vbo: u32, 
    ebo: u32, 
    vao: u32, 
    texture_location: i32,
    texture_mode: u32
}

const DEFAULT_VERTEX_SHADER: &str = r#"#version 150
in vec2 position;
in vec2 tex_coord;
in vec4 color;
in float uses_texture;
out vec4 Color;
out vec2 Tex_coord;
out float Uses_texture;
void main() {
    Color = color;
    Tex_coord = tex_coord;
    Uses_texture = uses_texture;
    gl_Position = vec4(position, 0, 1);
}"#;

const DEFAULT_FRAGMENT_SHADER: &str = r#"#version 150
in vec4 Color;
in vec2 Tex_coord;
in float Uses_texture;
out vec4 outColor;
uniform sampler2D tex;
void main() {
    vec4 tex_color = (Uses_texture != 0) ? texture(tex, Tex_coord) : vec4(1, 1, 1, 1);
    outColor = Color * tex_color;
}"#;

impl Backend for GL3Backend {
    fn new(texture_mode: ImageScaleStrategy) -> GL3Backend {
        let texture_mode = match texture_mode {
            ImageScaleStrategy::Pixelate => gl::NEAREST,
            ImageScaleStrategy::Blur => gl::LINEAR
        };
        unsafe {
            let vao = gl::GenVertexArray();
            gl::BindVertexArray(vao);
            let vbo = gl::GenBuffer();
            let ebo = gl::GenBuffer();
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable( gl::BLEND );
            let null = Image::new_null(1, 1, PixelFormat::RGBA);
            let texture = null.get_id();
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_text = CString::new(DEFAULT_VERTEX_SHADER).unwrap().into_raw();
            gl::ShaderSource(vertex, vertex_text);
            CString::from_raw(vertex_text);
            gl::CompileShader(vertex);
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_text = CString::new(DEFAULT_FRAGMENT_SHADER).unwrap().into_raw();
            gl::ShaderSource(fragment, fragment_text);
            CString::from_raw(fragment_text);
            gl::CompileShader(fragment);
            let shader = gl::CreateProgram();
            gl::AttachShader(shader, vertex);
            gl::AttachShader(shader, fragment);
            let raw = CString::new("out_color").unwrap().into_raw();
            gl::BindFragDataLocation(shader, 0, raw as *mut i8);
            CString::from_raw(raw);
            gl::LinkProgram(shader);
            gl::UseProgram(shader);
            GL3Backend {
                texture,
                vertices: Vec::with_capacity(1024),
                indices: Vec::with_capacity(1024), 
                null,
                vertex_length: 0, 
                index_length: 0, 
                shader, fragment, vertex, 
                vbo, ebo, vao, 
                texture_location: 0,
                texture_mode
            }
        }
    }
    
    fn clear(&mut self, col: Color) {
        unsafe {
            gl::ClearColor(col.r, col.g, col.b, col.a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn set_blend_mode(&mut self, blend: BlendMode) {
        unsafe { 
            gl::BlendFunc(gl::ONE, gl::ONE);
            gl::BlendEquationSeparate(blend as u32, gl::FUNC_ADD);
        }
    }

    fn reset_blend_mode(&mut self) {
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
        }
    }

    fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]) {
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
            unsafe {
                self.vertex_length = vertex_length * 2;
                // Create strings for all of the shader attributes
                let position_string = CString::new("position").unwrap().into_raw();
                let tex_coord_string = CString::new("tex_coord").unwrap().into_raw();
                let color_string = CString::new("color").unwrap().into_raw();
                let tex_string = CString::new("tex").unwrap().into_raw();
                let use_texture_string = CString::new("uses_texture").unwrap().into_raw();
                // Create the vertex array
                gl::BufferData(gl::ARRAY_BUFFER, self.vertex_length as isize, null(), gl::STREAM_DRAW);
                let stride_distance = (VERTEX_SIZE * size_of::<f32>()) as i32;
                // Set up the vertex attributes
                let pos_attrib = gl::GetAttribLocation(self.shader, position_string as *const i8) as u32;
                gl::EnableVertexAttribArray(pos_attrib);
                gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, null());
                let tex_attrib = gl::GetAttribLocation(self.shader, tex_coord_string as *const i8) as u32;
                gl::EnableVertexAttribArray(tex_attrib);
                gl::VertexAttribPointer(tex_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, (2 * size_of::<f32>()) as *const c_void);
                let col_attrib = gl::GetAttribLocation(self.shader, color_string as *const i8) as u32;
                gl::EnableVertexAttribArray(col_attrib);
                gl::VertexAttribPointer(col_attrib, 4, gl::FLOAT, gl::FALSE, stride_distance, (4 * size_of::<f32>()) as *const c_void);
                let use_texture_attrib = gl::GetAttribLocation(self.shader, use_texture_string as *const i8) as u32;
                gl::EnableVertexAttribArray(use_texture_attrib);
                gl::VertexAttribPointer(use_texture_attrib, 1, gl::FLOAT, gl::FALSE, stride_distance, (8 * size_of::<f32>()) as *const c_void);
                self.texture_location = gl::GetUniformLocation(self.shader, tex_string as *const i8);
                // Make sure to deallocate the attribute strings
                CString::from_raw(position_string);
                CString::from_raw(tex_coord_string);
                CString::from_raw(color_string);
                CString::from_raw(tex_string);
                CString::from_raw(use_texture_string);
            }
        }
        // Upload all of the vertex data
        let vertex_data = self.vertices.as_ptr() as *const c_void;
        unsafe { gl::BufferSubData(gl::ARRAY_BUFFER, 0, vertex_length as isize, vertex_data) };
        // Scan through the triangles, adding the indices to the index buffer (every time the
        // texture switches, flush and switch the bound texture)
        for triangle in triangles.iter() {
            if let Some(ref img) = triangle.image {
                if self.texture != self.null.get_id() && self.texture != img.get_id() {
                    self.flush();
                }
                self.texture = img.get_id();
            }
            self.indices.extend(triangle.indices.iter());
        }
        // Flush any remaining triangles
        self.flush();
        self.vertices.clear();
    }

    fn flush(&mut self) {
        if self.indices.len() != 0 {
            unsafe {
                // Check if the index buffer is big enough and upload the data
                let index_length = size_of::<u32>() * self.indices.len();
                let index_data = self.indices.as_ptr() as *const c_void;
                if index_length > self.index_length {
                    self.index_length = index_length * 2;
                    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, self.index_length as isize, null(), gl::STREAM_DRAW);
                }
                gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, index_length as isize, index_data);
                // Upload the texture to the GPU
                gl::ActiveTexture(gl::TEXTURE0);
                if self.texture != 0 {
                    gl::BindTexture(gl::TEXTURE_2D, self.texture);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.texture_mode as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.texture_mode as i32);
                }
                gl::Uniform1i(self.texture_location, 0);
                // Draw the triangles
                gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, null());
            }
            self.indices.clear();
            self.texture = self.null.get_id();
        }
    }

    fn create_texture(data: *const c_void, width: u32, height: u32, format: PixelFormat) -> ImageData where Self: Sized {
        let format = match format {
            PixelFormat::RGB => gl::RGB as isize,
            PixelFormat::RGBA => gl::RGBA as isize,
            PixelFormat::BGR => gl::BGR as isize,
            PixelFormat::BGRA => gl::BGRA as isize,
        };
        unsafe {
            let id = gl::GenTexture();
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, 
                           height as i32, 0, format as u32, gl::UNSIGNED_BYTE, data);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            ImageData { id, width, height }
        }
    }

    fn destroy_texture(data: &mut ImageData) where Self: Sized {
        unsafe {
            gl::DeleteTexture(data.id);
        }
    }

    fn create_surface(image: &Image) -> SurfaceData where Self: Sized {
        let surface = SurfaceData {
            framebuffer: unsafe { gl::GenFramebuffer() }
        };
        unsafe { 
            gl::BindFramebuffer(gl::FRAMEBUFFER, surface.framebuffer);
            gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, image.get_id(), 0);
            gl::DrawBuffer(gl::COLOR_ATTACHMENT0);
        }
        surface
    }

    fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized {
        let mut viewport = [0, 0, 0, 0];
        unsafe {
            gl::GetViewport((&mut viewport).as_mut_ptr());
            gl::BindFramebuffer(gl::FRAMEBUFFER, surface.data.framebuffer);
            gl::Viewport(0, 0, surface.image.source_width() as i32, surface.image.source_height() as i32);
        }
        viewport
    }

    fn unbind_surface(_surface: &Surface, viewport: &[i32]) where Self: Sized {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0); 
            gl::Viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
        }
    }

    fn destroy_surface(surface: &SurfaceData) where Self: Sized {
        unsafe { 
            gl::DeleteFramebuffer(surface.framebuffer);
        }
    }

    fn viewport(x: i32, y: i32, width: i32, height: i32) where Self: Sized {
        unsafe { 
            gl::Viewport(x, y, width, height); 
        }
    }
}

impl Drop for GL3Backend {
    fn drop(&mut self) { 
        unsafe {
            gl::DeleteProgram(self.shader);
            gl::DeleteShader(self.fragment);
            gl::DeleteShader(self.vertex);
            gl::DeleteBuffer(self.vbo);
            gl::DeleteBuffer(self.ebo);
            gl::DeleteVertexArray(self.vao);
        }
    } 
}

