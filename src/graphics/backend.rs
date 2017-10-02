extern crate gl;

use gl::types::*;
use geom::Transform;
use graphics::{Color, Vertex};
use std::vec::Vec;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;
use std::str::from_utf8;

pub struct Backend {
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
    transform: Transform
}

const VERTEX_SIZE: usize = 8; // the number of floats in a vertex

const DEFAULT_VERTEX_SHADER: &str = r#"#version 150
in vec2 position;
in vec2 tex_coord;
in vec4 color;
uniform mat3 transform;
out vec4 Color;
out vec2 Tex_coord;
void main() {
	Color = color;
	Tex_coord = tex_coord;
	vec3 transformed = vec3(position, 1.0) * transform;
	transformed.z = 0;
	gl_Position = vec4(transformed, 1.0);
}"#;

const DEFAULT_FRAGMENT_SHADER: &str = r#"#version 150
in vec4 Color;
in vec2 Tex_coord;
out vec4 outColor;
uniform sampler2D tex;
void main() {
	vec4 tex_color = texture(tex, Tex_coord);
	outColor = Color * tex_color;
}"#;

impl Drop for Backend {
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

impl Backend {
    pub fn new() -> Backend {
        unsafe {
            let mut vao: u32 = 0;
            gl::GenVertexArrays(1, &mut vao as *mut GLuint);
            let mut vbo: u32 = 0;
            let mut ebo: u32 = 0;
            gl::GenBuffers(1, &mut vbo as *mut GLuint);
            gl::GenBuffers(1, &mut ebo as *mut GLuint);
            let mut backend = Backend {
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
                transform: Transform::identity()
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

    pub fn clear(&mut self, col: Color) {
        unsafe {
            gl::ClearColor(col.r, col.g, col.b, col.a);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn flush(&mut self) {
        unsafe {
            let transform_string = CString::new("transform").unwrap().into_raw();
            let position_string = CString::new("position").unwrap().into_raw();
            let tex_coord_string = CString::new("tex_coord").unwrap().into_raw();
            let color_string = CString::new("color").unwrap().into_raw();
            let tex_string = CString::new("tex").unwrap().into_raw();
            let transform_attrib = gl::GetUniformLocation(self.shader, transform_string as *const GLchar);
            let transform_ptr = self.transform.get_array().as_ptr() as *const GLfloat;
            gl::UniformMatrix3fv(transform_attrib, 1, gl::FALSE, transform_ptr);
            //Bind the vertex data
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, (self.vertices.len() * size_of::<GLfloat>()) as isize, 
                           self.vertices.as_ptr() as *const c_void, gl::STREAM_DRAW);
            //Bind the index data
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (self.indices.len() * size_of::<GLuint>()) as isize, 
                           self.indices.as_ptr() as *const c_void, gl::STREAM_DRAW);
            //Set up the vertex attributes
            let pos_attrib = gl::GetAttribLocation(self.shader, position_string as *const GLchar) as u32;
            gl::EnableVertexAttribArray(pos_attrib);
            gl::VertexAttribPointer(pos_attrib, 2, gl::FLOAT, gl::FALSE, 8 * size_of::<GLfloat>() as i32, 
                                    null());
            let tex_attrib = gl::GetAttribLocation(self.shader, tex_coord_string as *const GLchar) as u32;
            gl::EnableVertexAttribArray(tex_attrib);
            gl::VertexAttribPointer(tex_attrib, 2, gl::FLOAT, gl::FALSE, 8 * size_of::<GLfloat>() as i32, 
                                    (2 * size_of::<GLfloat>()) as *const c_void);
            let col_attrib = gl::GetAttribLocation(self.shader, color_string as *const GLchar) as u32;
            gl::EnableVertexAttribArray(col_attrib);
            gl::VertexAttribPointer(col_attrib, 4, gl::FLOAT, gl::FALSE, 8 * size_of::<GLfloat>() as i32, 
                                    (4 * size_of::<GLfloat>()) as *const c_void);
            //Upload the texture to the GPU
            self.texture_location = gl::GetUniformLocation(self.shader, tex_string as *const GLchar);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
            gl::Uniform1i(self.texture_location, 0);
            //Draw the triangles
            gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, null());
            CString::from_raw(transform_string);
            CString::from_raw(position_string);
            CString::from_raw(tex_coord_string);
            CString::from_raw(color_string);
            CString::from_raw(tex_string);
        }
		self.vertices.clear();
        self.indices.clear();
        self.texture = 0;
    }

    pub fn flip(&mut self) {
        self.flush();
    }

    fn switch_texture(&mut self, texture: GLuint) {
        if self.texture != 0 && self.texture != texture {
            self.flush();
        }
        self.texture = texture;
    }

    pub fn add(&mut self, texture: GLuint, vertices: &[Vertex], indices: &[GLuint]) {
        self.switch_texture(texture);
        let offset = self.vertices.len() / VERTEX_SIZE;
        for vertex in vertices {
            self.vertices.push(vertex.pos.x);
            self.vertices.push(vertex.pos.y);
            self.vertices.push(vertex.tex_pos.x);
            self.vertices.push(vertex.tex_pos.y);
            self.vertices.push(vertex.col.r);
            self.vertices.push(vertex.col.g);
            self.vertices.push(vertex.col.b);
            self.vertices.push(vertex.col.a);
        }
        for index in indices {
            self.indices.push(index + offset as u32);
        }
    }
}
