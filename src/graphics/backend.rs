extern crate gl;

use gl::types::*;
use graphics::{Color, Vertex};
use std::vec::Vec;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;
use std::str::from_utf8;

pub trait Backend: Send {
     fn clear(&mut self, color: Color);
     fn flush(&mut self);
     fn flip(&mut self);
     fn add(&mut self, texture: GLuint, vertices: &[Vertex], indices: &[GLuint]);
     fn add_vertex(&mut self, vertex: &Vertex);
     fn add_index(&mut self, index: GLuint);
     fn num_vertices(&self) -> usize;
}

pub struct GLBackend {
    texture: GLuint,
    vertices: Vec<f32>,
    indices: Vec<GLuint>,
    shader: GLuint,
    fragment: GLuint,
    vertex: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    vao: GLuint,
    texture_location: GLint,
}

const VERTEX_SIZE: usize = 9; // the number of floats in a vertex

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

impl GLBackend {
    pub fn new() -> GLBackend {
        unsafe {
            let mut vao: u32 = 0;
            gl::GenVertexArrays(1, &mut vao as *mut GLuint);
            gl::BindVertexArray(vao);
            let mut vbo: u32 = 0;
            let mut ebo: u32 = 0;
            gl::GenBuffers(1, &mut vbo as *mut GLuint);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::GenBuffers(1, &mut ebo as *mut GLuint);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            let mut backend = GLBackend {
                texture: 0,
                vertices: Vec::with_capacity(1024),
                indices: Vec::with_capacity(1024),
                shader: 0,
                fragment: 0,
                vertex: 0,
                vbo: vbo,
                ebo: ebo,
                vao: vao,
                texture_location: 0,
            };
            backend.set_shader(DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER);
            backend
        }
    }

    pub fn set_shader(&mut self, vertex_shader: &str, fragment_shader: &str) {
        unsafe {
            if self.shader !=0 { gl::DeleteProgram(self.shader); }
            if self.vertex != 0 { gl::DeleteShader(self.vertex); }
            if self.fragment != 0 { gl::DeleteShader(self.fragment); }
            self.vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let vertex_text = CString::new(vertex_shader).unwrap().into_raw();
            gl::ShaderSource(self.vertex, 1, 
                             &(vertex_text as *const GLchar) as *const *const GLchar, null());
            gl::CompileShader(self.vertex);
            let mut status : GLint = 0;
            gl::GetShaderiv(self.vertex, gl::COMPILE_STATUS, &mut status as *mut GLint);
            if status as u8 != gl::TRUE {
                println!("Vertex shader compilation failed.");
                let buffer: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetShaderInfoLog(self.vertex, 512, &mut length as *mut GLsizei, 
                                     buffer.as_ptr() as *mut GLchar);
                println!("Error: {}", from_utf8(&buffer).unwrap());
            }
            self.fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            let fragment_text = CString::new(fragment_shader).unwrap().into_raw();
            gl::ShaderSource(self.fragment, 1, 
                             &(fragment_text as *const GLchar) as *const *const GLchar, null());
            gl::CompileShader(self.fragment);
            gl::GetShaderiv(self.fragment, gl::COMPILE_STATUS, &mut status as *mut GLint);
            if status as u8 != gl::TRUE {
                println!("Fragment shader compilation failed.");
                let buffer: [u8; 512] = [0; 512];
                let mut length = 0;
                gl::GetShaderInfoLog(self.fragment, 512, &mut length as *mut GLsizei, buffer.as_ptr() as *mut GLchar);
                println!("Error: {}", from_utf8(&buffer).unwrap());
            }
            self.shader = gl::CreateProgram();
            gl::AttachShader(self.shader, self.vertex);
            gl::AttachShader(self.shader, self.fragment);
            let raw = CString::new("out_color").unwrap().into_raw();
            gl::BindFragDataLocation(self.shader, 0, raw as *mut GLchar);
            gl::LinkProgram(self.shader);
            gl::UseProgram(self.shader);
            CString::from_raw(vertex_text);
            CString::from_raw(fragment_text);
            CString::from_raw(raw);
        }
    }

    unsafe fn create_buffers(&mut self, vbo_size: isize, ebo_size: isize) {
        //Create strings for all of the shader attributes
        let position_string = CString::new("position").unwrap().into_raw();
        let tex_coord_string = CString::new("tex_coord").unwrap().into_raw();
        let color_string = CString::new("color").unwrap().into_raw();
        let tex_string = CString::new("tex").unwrap().into_raw();
        let use_texture_string = CString::new("uses_texture").unwrap().into_raw();
        gl::BufferData(gl::ARRAY_BUFFER, vbo_size * size_of::<GLfloat>() as isize, null(), gl::STREAM_DRAW);
        //Bind the index data
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, ebo_size * size_of::<GLuint>() as isize, null(), gl::STREAM_DRAW);
        let stride_distance = (VERTEX_SIZE * size_of::<GLfloat>()) as i32;
        //Set up the vertex attributes
        let pos_attrib = gl::GetAttribLocation(self.shader, position_string as *const GLchar) as u32;
        gl::EnableVertexAttribArray(pos_attrib);
        gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, null());
        let tex_attrib = gl::GetAttribLocation(self.shader, tex_coord_string as *const GLchar) as u32;
        gl::EnableVertexAttribArray(tex_attrib);
        gl::VertexAttribPointer(tex_attrib, 2, gl::FLOAT, gl::FALSE, stride_distance, 
                                (2 * size_of::<GLfloat>()) as *const c_void);
        let col_attrib = gl::GetAttribLocation(self.shader, color_string as *const GLchar) as u32;
        gl::EnableVertexAttribArray(col_attrib);
        gl::VertexAttribPointer(col_attrib, 4, gl::FLOAT, gl::FALSE, stride_distance, 
                                (4 * size_of::<GLfloat>()) as *const c_void);
        let use_texture_attrib = gl::GetAttribLocation(self.shader, use_texture_string as *const GLchar) as u32;
        gl::EnableVertexAttribArray(use_texture_attrib);
        gl::VertexAttribPointer(use_texture_attrib, 1, gl::FLOAT, gl::FALSE, stride_distance, 
                                (8 * size_of::<GLfloat>()) as *const c_void);
        self.texture_location = gl::GetUniformLocation(self.shader, tex_string as *const GLchar);
        //Make sure to deallocate the attribute strings
        CString::from_raw(position_string);
        CString::from_raw(tex_coord_string);
        CString::from_raw(color_string);
        CString::from_raw(tex_string);
        CString::from_raw(use_texture_string);
    }

    fn switch_texture(&mut self, texture: GLuint) {
        if self.texture != 0 && self.texture != texture {
            self.flush();
        }
        self.texture = texture;
    }
}

impl Drop for GLBackend {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader);
            gl::DeleteShader(self.fragment);
            gl::DeleteShader(self.vertex);
            gl::DeleteBuffers(1, &self.vbo as *const GLuint);
            gl::DeleteBuffers(1, &self.ebo as *const GLuint);
            gl::DeleteVertexArrays(1, &self.vao as *const GLuint);
        }
    }
}

impl Backend for GLBackend {
     fn clear(&mut self, col: Color) {
        unsafe {
            gl::ClearColor(col.r, col.g, col.b, col.a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.vertices.clear();
        self.indices.clear();
    }

     fn flush(&mut self) {
        unsafe {
            //Check to see if the GL buffers can hold the data
            let mut vbo_size: GLint = 0;
            gl::GetBufferParameteriv(gl::ARRAY_BUFFER, gl::BUFFER_SIZE, &mut vbo_size as *mut GLint);
            let mut ebo_size: GLint = 0;
            gl::GetBufferParameteriv(gl::ELEMENT_ARRAY_BUFFER, gl::BUFFER_SIZE, &mut ebo_size as *mut GLint);
            if self.vertices.len() > (vbo_size as usize / VERTEX_SIZE) as usize 
                || self.indices.len() > ebo_size as usize {
                let vertex_capacity = self.vertices.capacity() as isize * 2;
                let index_capacity = self.indices.capacity() as isize* 2;
                self.create_buffers(vertex_capacity, index_capacity);
            }
            //Bind the vertex data
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (size_of::<GLfloat>() * self.vertices.len()) as isize, self.vertices.as_ptr() as *const c_void);
            gl::BufferSubData(gl::ELEMENT_ARRAY_BUFFER, 0, (size_of::<GLuint>() * self.indices.len()) as isize, self.indices.as_ptr() as *const c_void);
            //Upload the texture to the GPU
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::Uniform1i(self.texture_location, 0);
            //Draw the triangles
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, null());
        }
		self.vertices.clear();
        self.indices.clear();
        self.texture = 0;
    }

     fn flip(&mut self) {
        self.flush();
    }

     fn num_vertices(&self) -> usize {
        self.vertices.len() / VERTEX_SIZE
    }

     fn add_vertex(&mut self, vertex: &Vertex) {
        self.vertices.push(vertex.pos.x);
        self.vertices.push(vertex.pos.y);
        self.vertices.push(vertex.tex_pos.x);
        self.vertices.push(vertex.tex_pos.y);
        self.vertices.push(vertex.col.r);
        self.vertices.push(vertex.col.g);
        self.vertices.push(vertex.col.b);
        self.vertices.push(vertex.col.a);
        self.vertices.push(if vertex.use_texture { 1f32 } else { 0f32 } );
    }

     fn add_index(&mut self, index: GLuint) {
        self.indices.push(index);
    }

     fn add(&mut self, texture: GLuint, vertices: &[Vertex], indices: &[GLuint]) {
        self.switch_texture(texture);
        let offset = self.vertices.len() / VERTEX_SIZE;
        for vertex in vertices {
            self.add_vertex(vertex);
        }
        for index in indices {
            self.add_index(index + offset as u32);
        }
    }
}
