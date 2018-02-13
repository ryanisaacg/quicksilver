use geom::Vector;
use graphics::{Color, Image, PixelFormat};
use ffi::gl;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;
use std::str::from_utf8;

#[derive(Clone, Copy)]
pub(crate) struct Vertex {
    pub pos: Vector,
    pub tex_pos: Vector,
    pub col: Color,
    pub use_texture: bool,
}

#[repr(u32)]
#[derive(Clone, Copy)]
/// The way the colors are blended when drawing on top of other color
///
/// Blend modes only apply to RGB values
pub enum BlendMode {
    /// Add the color being drawn onto and the color being drawn
    ///
    /// Adding red and blue will produce purple for example
    Additive = gl::FUNC_ADD,
    /// Subtract the color being drawn onto and the color being drawn
    ///
    /// Subtracting red from purple will produce blue for example
    Subtractive = gl::FUNC_REVERSE_SUBTRACT,
    /// Take the minimum of each component of the color
    ///
    /// Purple and red will produce red, blue and red will produce black
    Minimum = gl::MIN,
    /// Take the maximum of each component of the color
    ///
    /// Purple and red will produce purple, blue and red will produce purple
    Maximum = gl::MAX
}

pub(crate) struct Backend {
    texture: u32,
    pub(crate) vertices: Vec<f32>,
    pub(crate) indices: Vec<u32>, 
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
}

#[cfg(not(target_arch="wasm32"))]
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

#[cfg(not(target_arch="wasm32"))]
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

#[cfg(target_arch="wasm32")]
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

#[cfg(target_arch="wasm32")]
const DEFAULT_FRAGMENT_SHADER: &str = r#"varying highp vec4 Color;
varying highp vec2 Tex_coord;
varying lowp float Uses_texture;
uniform sampler2D tex;
void main() {
    highp vec4 tex_color = (int(Uses_texture) != 0) ? texture2D(tex, Tex_coord) : vec4(1, 1, 1, 1);
    gl_FragColor = Color * tex_color;
}"#;

pub(crate) const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

impl Backend {
    pub fn new() -> Backend { 
        let (vao, vbo, ebo) = unsafe {
            let vao = gl::GenVertexArray();
            gl::BindVertexArray(vao);
            let vbo = gl::GenBuffer();
            let ebo = gl::GenBuffer();
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable( gl::BLEND );
            (vao, vbo, ebo)
        };
        let null = Image::new_null(1, 1, PixelFormat::RGBA);
        let texture = null.get_id();
        let mut backend = Backend {
            texture,
            vertices: Vec::with_capacity(1024),
            indices: Vec::with_capacity(1024), 
            null,
            vertex_length: 0, 
            index_length: 0, 
            shader: 0, 
            fragment: 0, 
            vertex: 0, 
            vbo, 
            ebo, 
            vao, 
            texture_location: 0,
        };
        backend.set_shader(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER);
        backend
    }

    
    fn set_shader(&mut self, vertex_shader: &str, fragment_shader: &str) {
        unsafe {
            if self.shader != 0 {
                gl::DeleteProgram(self.shader);
            }
            if self.vertex != 0 {
                gl::DeleteShader(self.vertex);
            }
            if self.fragment != 0 {
                gl::DeleteShader(self.fragment);
            }
            self.vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_text = CString::new(vertex_shader).unwrap().into_raw();
            gl::ShaderSource(self.vertex, vertex_text);
            gl::CompileShader(self.vertex);
            let mut status: i32 = 0;
            gl::GetShaderiv(self.vertex, gl::COMPILE_STATUS, &mut status as *mut i32);
            if status as u8 != gl::TRUE {
                println!("Vertex shader compilation failed.");
                let buffer: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetShaderInfoLog(self.vertex, 512, &mut length as *mut i32, buffer.as_ptr() as *mut i8);
                println!("Error: {}", from_utf8(&buffer).unwrap());
            }
            self.fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_text = CString::new(fragment_shader).unwrap().into_raw();
            gl::ShaderSource(self.fragment, fragment_text);
            gl::CompileShader(self.fragment);
            gl::GetShaderiv(self.fragment, gl::COMPILE_STATUS, &mut status as *mut i32);
            if status as u8 != gl::TRUE {
                println!("Fragment shader compilation failed.");
                let buffer: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetShaderInfoLog(self.fragment, 512, &mut length as *mut i32, buffer.as_ptr() as *mut i8);
                println!("Error: {}", from_utf8(&buffer).unwrap());
            }
            self.shader = gl::CreateProgram();
            gl::AttachShader(self.shader, self.vertex);
            gl::AttachShader(self.shader, self.fragment);
            #[cfg(not(target_arch="wasm32"))] {
                let raw = CString::new("out_color").unwrap().into_raw();
                gl::BindFragDataLocation(self.shader, 0, raw as *mut i8);
                CString::from_raw(raw);
            }
            gl::LinkProgram(self.shader);
            gl::UseProgram(self.shader);
            #[cfg(not(target_arch="wasm32"))] {
                CString::from_raw(vertex_text);
                CString::from_raw(fragment_text);
            }
        }
    }

    
    unsafe fn create_buffers(&mut self, vbo_size: isize, ebo_size: isize) {
        //Create strings for all of the shader attributes
        let position_string = CString::new("position").unwrap().into_raw();
        let tex_coord_string = CString::new("tex_coord").unwrap().into_raw();
        let color_string = CString::new("color").unwrap().into_raw();
        let tex_string = CString::new("tex").unwrap().into_raw();
        let use_texture_string = CString::new("uses_texture").unwrap().into_raw();
        gl::BufferData(gl::ARRAY_BUFFER,vbo_size * size_of::<f32>() as isize, null(), gl::STREAM_DRAW);
        //Bind the index data
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, ebo_size * size_of::<u32>() as isize, null(), gl::STREAM_DRAW);
        let stride_distance = (VERTEX_SIZE * size_of::<f32>()) as i32;
        //Set up the vertex attributes
        let pos_attrib = gl::GetAttribLocation(self.shader, position_string as *const i8) as
            u32;
        gl::EnableVertexAttribArray(pos_attrib);
        gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, null());
        let tex_attrib = gl::GetAttribLocation(self.shader, tex_coord_string as *const i8) as
            u32;
        gl::EnableVertexAttribArray(tex_attrib);
        gl::VertexAttribPointer(tex_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, (2 * size_of::<f32>()) as *const c_void);
        let col_attrib = gl::GetAttribLocation(self.shader, color_string as *const i8) as u32;
        gl::EnableVertexAttribArray(col_attrib);
        gl::VertexAttribPointer(col_attrib, 4, gl::FLOAT, gl::FALSE, stride_distance, (4 * size_of::<f32>()) as *const c_void);
        let use_texture_attrib = gl::GetAttribLocation(self.shader, use_texture_string as *const i8) as u32;
        gl::EnableVertexAttribArray(use_texture_attrib);
        gl::VertexAttribPointer(use_texture_attrib, 1, gl::FLOAT, gl::FALSE, stride_distance, (8 * size_of::<f32>()) as *const c_void);
        self.texture_location = gl::GetUniformLocation(self.shader, tex_string as *const i8);
        self.vertex_length = vbo_size as usize;
        self.index_length = ebo_size as usize;
        //Make sure to deallocate the attribute strings
        #[cfg(not(target_arch="wasm32"))] {
            CString::from_raw(position_string);
            CString::from_raw(tex_coord_string);
            CString::from_raw(color_string);
            CString::from_raw(tex_string);
            CString::from_raw(use_texture_string);
        }
    }

    fn switch_texture(&mut self, texture: u32) {
        if self.texture != self.null.get_id() && self.texture != texture {
            self.flush();
        }
        self.texture = texture;
    }

    pub fn set_blend_mode(&mut self, blend: BlendMode) {
        self.flush();
        unsafe { gl::BlendFunc(gl::ONE, gl::ONE) };
        unsafe { gl::BlendEquationSeparate(blend as u32, gl::FUNC_ADD) };
    }

    pub fn reset_blend_mode(&mut self) {
        self.flush();
        unsafe { gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA) };
        unsafe { gl::BlendEquationSeparate(gl::FUNC_ADD, gl::FUNC_ADD) };
    }


    pub fn flush(&mut self) { 
        if self.indices.len() != 0 {
            unsafe {
                //Bind the vertex data
                let vertex_length = size_of::<f32>() * self.vertices.len();
                let vertex_data = self.vertices.as_ptr() as *const c_void;
                let index_length = size_of::<u32>() * self.indices.len();
                let index_data = self.indices.as_ptr() as *const c_void;
                if vertex_length > self.vertex_length || index_length > self.index_length {
                    self.create_buffers(vertex_length as isize * 2, index_length as isize * 2);
                }
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, vertex_length as isize, vertex_data);
                gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, index_length as isize, index_data);
                //Upload the texture to the GPU
                gl::ActiveTexture(gl::TEXTURE0);
                if self.texture != 0 {
                    gl::BindTexture(gl::TEXTURE_2D, self.texture);
                }
                gl::Uniform1i(self.texture_location, 0);
                //Draw the triangles
                gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, null());
            }
            self.vertices.clear();
            self.indices.clear();
            self.texture = self.null.get_id();
        }
    }
    
    
    pub fn clear(&mut self, col: Color) {
        unsafe {
            gl::ClearColor(col.r, col.g, col.b, col.a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len() / VERTEX_SIZE
    }

    pub fn add_vertex(&mut self, vertex: &Vertex) {
        self.vertices.push(vertex.pos.x);
        self.vertices.push(vertex.pos.y);
        self.vertices.push(vertex.tex_pos.x);
        self.vertices.push(vertex.tex_pos.y);
        self.vertices.push(vertex.col.r);
        self.vertices.push(vertex.col.g);
        self.vertices.push(vertex.col.b);
        self.vertices.push(vertex.col.a);
        self.vertices.push(
            if vertex.use_texture { 1f32 } else { 0f32 },
        );
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }
    
    pub fn add(&mut self, texture: u32, vertices: &[Vertex], indices: &[u32]) {
        self.switch_texture(texture);
        let offset = self.num_vertices();
        for vertex in vertices {
            self.add_vertex(vertex);
        }
        for index in indices {
            self.add_index(index + offset as u32);
        }
    }
}

impl Drop for Backend {
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

