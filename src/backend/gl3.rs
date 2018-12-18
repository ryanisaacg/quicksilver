use crate::{
    Result,
    backend::{Backend, ImageData, SurfaceData, VERTEX_SIZE},
    geom::{Rectangle, Vector},
    graphics::{BlendMode, Color, GpuTriangle, Image, ImageScaleStrategy, PixelFormat, Surface, Vertex}
};
use glutin::{GlWindow, dpi::LogicalSize};
use std::{
    ffi::CString,
    mem::size_of,
    os::raw::c_void,
    ptr::null as nullptr,
};

pub struct GL3Backend {
    context: GlWindow,
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

fn format_gl(format: PixelFormat) -> u32 {
    match format {
        PixelFormat::RGB => gl::RGB,
        PixelFormat::RGBA => gl::RGBA
    }
}

impl Backend for GL3Backend {
    type Platform = GlWindow;

    unsafe fn new(context: GlWindow, texture_mode: ImageScaleStrategy, multisample: bool) -> Result<GL3Backend> {
        let texture_mode = match texture_mode {
            ImageScaleStrategy::Pixelate => gl::NEAREST,
            ImageScaleStrategy::Blur => gl::LINEAR
        };
        let vao = {
            let mut array = 0;
            gl::GenVertexArrays(1, &mut array as *mut u32);
            array
        };
        gl::BindVertexArray(vao);
        let mut buffers = [0, 0];
        gl::GenBuffers(2, (&mut buffers).as_mut_ptr());
        let [vbo, ebo] = buffers;
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BlendFuncSeparate(
            gl::SRC_ALPHA,
            gl::ONE_MINUS_SRC_ALPHA,
            gl::ONE,
            gl::ONE_MINUS_SRC_ALPHA,
        );
        gl::Enable(gl::BLEND);
        if multisample {
            gl::Enable(gl::MULTISAMPLE);
        }
        let null = Image::new_null(1, 1, PixelFormat::RGBA)?;
        let texture = null.get_id();
        let vertex = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_text: *mut i8 = CString::new(DEFAULT_VERTEX_SHADER).expect("No interior null bytes in shader").into_raw();
        gl::ShaderSource(vertex, 1, &(vertex_text as *const i8) as *const *const i8, nullptr());
        CString::from_raw(vertex_text);
        gl::CompileShader(vertex);
        let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_text: *mut i8 = CString::new(DEFAULT_FRAGMENT_SHADER).expect("No interior null bytes in shader").into_raw();
        gl::ShaderSource(fragment, 1, &(fragment_text as *const i8) as *const *const i8, nullptr());
        CString::from_raw(fragment_text);
        gl::CompileShader(fragment);
        let shader = gl::CreateProgram();
        gl::AttachShader(shader, vertex);
        gl::AttachShader(shader, fragment);
        let raw = CString::new("out_color").expect("No interior null bytes in shader").into_raw();
        gl::BindFragDataLocation(shader, 0, raw as *mut i8);
        CString::from_raw(raw);
        gl::LinkProgram(shader);
        gl::UseProgram(shader);
        Ok(GL3Backend {
            context,
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
        })
    }
    
    unsafe fn clear(&mut self, col: Color) {
        gl::ClearColor(col.r, col.g, col.b, col.a);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    unsafe fn set_blend_mode(&mut self, blend: BlendMode) {
        gl::BlendFunc(gl::ONE, gl::ONE);
        gl::BlendEquationSeparate(blend as u32, gl::FUNC_ADD);
    }

    unsafe fn reset_blend_mode(&mut self) {
        gl::BlendFuncSeparate(
            gl::SRC_ALPHA,
            gl::ONE_MINUS_SRC_ALPHA,
            gl::ONE,
            gl::ONE_MINUS_SRC_ALPHA,
        );
        gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD);
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
            // Create strings for all of the shader attributes
            let position_string = CString::new("position").expect("No interior null bytes in shader").into_raw();
            let tex_coord_string = CString::new("tex_coord").expect("No interior null bytes in shader").into_raw();
            let color_string = CString::new("color").expect("No interior null bytes in shader").into_raw();
            let tex_string = CString::new("tex").expect("No interior null bytes in shader").into_raw();
            let use_texture_string = CString::new("uses_texture").expect("No interior null bytes in shader").into_raw();
            // Create the vertex array
            gl::BufferData(gl::ARRAY_BUFFER, self.vertex_length as isize, nullptr(), gl::STREAM_DRAW);
            let stride_distance = (VERTEX_SIZE * size_of::<f32>()) as i32;
            // Set up the vertex attributes
            let pos_attrib = gl::GetAttribLocation(self.shader, position_string as *const i8) as u32;
            gl::EnableVertexAttribArray(pos_attrib);
            gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, nullptr());
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
        // Upload all of the vertex data
        let vertex_data = self.vertices.as_ptr() as *const c_void;
        gl::BufferSubData(gl::ARRAY_BUFFER, 0, vertex_length as isize, vertex_data);
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
        Ok(())
    }

    unsafe fn flush(&mut self) {
        if self.indices.len() != 0 {
            // Check if the index buffer is big enough and upload the data
            let index_length = size_of::<u32>() * self.indices.len();
            let index_data = self.indices.as_ptr() as *const c_void;
            if index_length > self.index_length {
                self.index_length = index_length * 2;
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, self.index_length as isize, nullptr(), gl::STREAM_DRAW);
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
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, nullptr());
            self.indices.clear();
            self.texture = self.null.get_id();
        }
    }

    unsafe fn create_texture(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<ImageData> where Self: Sized {
        let data = if data.len() == 0 { nullptr() } else { data.as_ptr() as *const c_void };
        let format = format_gl(format);
        let id = {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture as *mut u32);
            texture
        };
        gl::BindTexture(gl::TEXTURE_2D, id);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, 
                        height as i32, 0, format, gl::UNSIGNED_BYTE, data);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        Ok(ImageData { id, width, height })
    }

    unsafe fn destroy_texture(data: &mut ImageData) where Self: Sized {
        gl::DeleteTextures(1, &data.id as *const u32);
    }

    unsafe fn create_surface(image: &Image) -> Result<SurfaceData> where Self: Sized {
        let surface = SurfaceData {
            framebuffer: {
                let mut buffer = 0;
                gl::GenFramebuffers(1, &mut buffer as *mut u32);
                buffer
            }
        };
        gl::BindFramebuffer(gl::FRAMEBUFFER, surface.framebuffer);
        gl::FramebufferTexture(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, image.get_id(), 0);
        gl::DrawBuffers(1, &gl::COLOR_ATTACHMENT0 as *const u32);
        Ok(surface)
    }

    unsafe fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized {
        let mut viewport = [0, 0, 0, 0];
        gl::GetIntegerv(gl::VIEWPORT, (&mut viewport).as_mut_ptr());
        gl::BindFramebuffer(gl::FRAMEBUFFER, surface.data.framebuffer);
        gl::Viewport(0, 0, surface.image.source_width() as i32, surface.image.source_height() as i32);
        viewport
    }

    unsafe fn unbind_surface(_surface: &Surface, viewport: &[i32]) where Self: Sized {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0); 
        gl::Viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
    }

    unsafe fn destroy_surface(surface: &SurfaceData) where Self: Sized {
        gl::DeleteFramebuffers(1, &surface.framebuffer as *const u32);
    }

    unsafe fn viewport(&mut self, area: Rectangle) where Self: Sized {
        let size: LogicalSize = area.size().into();
        let dpi = self.context.get_hidpi_factor();
        self.context.resize(size.to_physical(dpi));
        let dpi = dpi as f32;
        gl::Viewport(
            (area.x() * dpi) as i32, 
            (area.y() * dpi) as i32, 
            (area.width() * dpi) as i32, 
            (area.height() * dpi) as i32
        );
    }

    unsafe fn get_region(&self, region: Rectangle, format: PixelFormat) -> Vec<u8> where Self: Sized {
        let bytes_per_pixel = match format {
            PixelFormat::RGBA => 4,
            PixelFormat::RGB => 3
        };
        let format = format_gl(format);
        let x = region.x() as i32;
        let y = region.y() as i32;
        let width = region.width() as i32;
        let height = region.height() as i32;
        let length = (width * height * bytes_per_pixel) as usize;
        let mut buffer = Vec::with_capacity(length);
        let pointer = buffer.as_mut_ptr() as *mut c_void;
        gl::ReadPixels(x, y, width, height, format, gl::UNSIGNED_BYTE, pointer);
        buffer.set_len(length);
        buffer
    }

    fn show_cursor(&mut self, show_cursor: bool) {
        self.context.hide_cursor(!show_cursor);
    }

    fn set_title(&mut self, title: &str) {
        self.context.set_title(title);
    }

    fn present(&self) -> Result<()> {
        Ok(self.context.swap_buffers()?)
    }

    fn set_fullscreen(&mut self, fullscreen: bool) -> Option<Vector> {
        self.context.set_fullscreen(if fullscreen {
            Some(self.context.get_primary_monitor())
        } else {
            None
        });
        None
    }

    fn resize(&mut self, size: Vector) {
        self.context.set_inner_size(size.into());
    }
}

impl Drop for GL3Backend {
    fn drop(&mut self) { 
        unsafe {
            gl::DeleteProgram(self.shader);
            gl::DeleteShader(self.fragment);
            gl::DeleteShader(self.vertex);
            gl::DeleteBuffers(2, &[self.vbo, self.ebo] as *const u32);
            gl::DeleteVertexArrays(1, &self.vao as *const u32);
        }
    } 
}
