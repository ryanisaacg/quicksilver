use {
    Result, geom::Vector, error::QuicksilverError, 
    graphics::{
        backend::{Backend, BlendMode, ImageData, ImageScaleStrategy, SurfaceData, VERTEX_SIZE},
        Color, GpuTriangle, Image, PixelFormat, Surface, Vertex
    },
    std::mem::size_of,
    stdweb::{
        web::{
            html_element::CanvasElement,
            TypedArray
        },
        unstable::TryInto
    },
    webgl_stdweb::{
        WebGLBuffer,
        WebGLProgram,
        WebGL2RenderingContext as gl,
        WebGLShader,
        WebGLUniformLocation
    }
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
    texture_location: Option<WebGLUniformLocation>,
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

static mut GL_CONTEXT: Option<gl> = None;
static mut TEXTURE_COUNT: u32 = 0;

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

    unsafe fn new(canvas: CanvasElement, texture_mode: ImageScaleStrategy) -> Result<WebGLBackend> {
        let ctx: gl = match canvas.get_context() {
            Ok(ctx) => ctx,
            _ => return Err(QuicksilverError::ContextError("Could not create WebGL2 context".to_owned()))
        };
        let texture_mode = match texture_mode {
            ImageScaleStrategy::Pixelate => gl::NEAREST,
            ImageScaleStrategy::Blur => gl::LINEAR
        };
        let vbo = try_opt(ctx.create_buffer(), "Create vertex buffer")?;
        let ebo = try_opt(ctx.create_buffer(), "Create index buffer")?;
        ctx.bind_buffer(gl::ARRAY_BUFFER, Some(&vbo));
        ctx.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, Some(&ebo));
        ctx.blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        ctx.enable(gl::BLEND);
        let vertex = try_opt(ctx.create_shader(gl::VERTEX_SHADER), "Create vertex shader")?;
        ctx.shader_source(&vertex, DEFAULT_VERTEX_SHADER);
        ctx.compile_shader(&vertex);
        let fragment = try_opt(ctx.create_shader(gl::FRAGMENT_SHADER), "Create fragment shader")?;
        ctx.shader_source(&fragment, DEFAULT_FRAGMENT_SHADER);
        ctx.compile_shader(&fragment);
        let shader = try_opt(ctx.create_program(), "Create shader program")?;
        ctx.attach_shader(&shader, &vertex);
        ctx.attach_shader(&shader, &fragment);
        ctx.link_program(&shader);
        ctx.use_program(Some(&shader));
        GL_CONTEXT = Some(ctx);
        // GL CALLS ARE ONLY SAFE AFTER THIS POINT
        let null = Image::new_null(1, 1, PixelFormat::RGBA)?;
        let texture = null.clone();
        Ok(WebGLBackend {
            texture,
            vertices: Vec::with_capacity(1024),
            indices: Vec::with_capacity(1024), 
            null,
            vertex_length: 0, 
            index_length: 0, 
            shader, fragment, vertex, vbo, ebo, 
            texture_location: None,
            texture_mode
        })
    }
    
    unsafe fn clear(&mut self, col: Color) {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.clear_color(col.r, col.g, col.b, col.a);
            gl_ctx.clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    unsafe fn set_blend_mode(&mut self, blend: BlendMode) {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.blend_func(gl::ONE, gl::ONE);
            gl_ctx.blend_equation_separate(blend as u32, gl::FUNC_ADD);
        }
    }

    unsafe fn reset_blend_mode(&mut self) {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl_ctx.blend_equation_separate(gl::FUNC_ADD, gl::FUNC_ADD);
        }
    }

    unsafe fn draw(&mut self, vertices: &[Vertex], triangles: &[GpuTriangle]) -> Result<()> {
        if let Some(ref gl_ctx) = GL_CONTEXT {
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
                gl_ctx.buffer_data(gl::ARRAY_BUFFER, self.vertex_length as i64, gl::STREAM_DRAW);
                let stride_distance = (VERTEX_SIZE * size_of::<f32>()) as i32;
                // Set up the vertex attributes
                let pos_attrib = gl_ctx.get_attrib_location(&self.shader, "position") as u32;
                gl_ctx.enable_vertex_attrib_array(pos_attrib);
                gl_ctx.vertex_attrib_pointer(pos_attrib, 2, gl::FLOAT, false, stride_distance, 0);
                let tex_attrib = gl_ctx.get_attrib_location(&self.shader, "tex_coord") as u32;
                gl_ctx.enable_vertex_attrib_array(tex_attrib);
                gl_ctx.vertex_attrib_pointer(tex_attrib, 2, gl::FLOAT, false, stride_distance, 2 * size_of::<f32>() as i64);
                let col_attrib = gl_ctx.get_attrib_location(&self.shader, "color") as u32;
                gl_ctx.enable_vertex_attrib_array(col_attrib);
                gl_ctx.vertex_attrib_pointer(col_attrib, 4, gl::FLOAT, false, stride_distance, 4 * size_of::<f32>() as i64);
                let use_texture_attrib = gl_ctx.get_attrib_location(&self.shader, "uses_texture") as u32;
                gl_ctx.enable_vertex_attrib_array(use_texture_attrib);
                gl_ctx.vertex_attrib_pointer(use_texture_attrib, 1, gl::FLOAT, false, stride_distance, 8 * size_of::<f32>() as i64);
                self.texture_location = Some(try_opt(gl_ctx.get_uniform_location(&self.shader, "tex"), "Get texture uniform")?);
            }
            // Upload all of the vertex data
            let array: TypedArray<f32> = self.vertices.as_slice().into();
            gl_ctx.buffer_sub_data(gl::ARRAY_BUFFER, 0, &array.buffer());
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
        Ok(())
    }

    unsafe fn flush(&mut self) {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            if self.indices.len() != 0 {
                // Check if the index buffer is big enough and upload the data
                let index_length = size_of::<u32>() * self.indices.len();
                if index_length > self.index_length {
                    self.index_length = index_length * 2;
                    gl_ctx.buffer_data(gl::ELEMENT_ARRAY_BUFFER, self.index_length as i64, gl::STREAM_DRAW);
                }
                let array: TypedArray<u32> = self.indices.as_slice().into();
                gl_ctx.buffer_sub_data(gl::ELEMENT_ARRAY_BUFFER, 0, &array.buffer());
                // Upload the texture to the GPU
                gl_ctx.active_texture(gl::TEXTURE0);
                if self.texture.get_id() != 0 {
                    gl_ctx.bind_texture(gl::TEXTURE_2D, Some(&self.texture.data().data));
                    gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.texture_mode as i32);
                    gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.texture_mode as i32);
                }
                match self.texture_location {
                    Some(ref location) => gl_ctx.uniform1i(Some(location), 0),
                    None => gl_ctx.uniform1i(None, 0)
                }
                // Draw the triangles
                gl_ctx.draw_elements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, 0);
            }
            self.indices.clear();
            self.texture = self.null.clone();
        } else {
            unreachable!();
        }
    }

    unsafe fn create_texture(data: &[u8], width: u32, height: u32, format: PixelFormat) -> Result<ImageData> where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            let id = TEXTURE_COUNT;
            TEXTURE_COUNT += 1;
            let format = match format {
                PixelFormat::RGB => gl::RGB as i64,
                PixelFormat::RGBA => gl::RGBA as i64
            };

            let texture = try_opt(gl_ctx.create_texture(), "Create GL texture")?;
            gl_ctx.bind_texture(gl::TEXTURE_2D, Some(&texture));
            gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl_ctx.tex_parameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl_ctx.tex_parameteri(gl::TEXTURE_2D,
             gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            if data.len() == 0 {
                gl_ctx.tex_image2_d(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, 
                            height as i32, 0, format as u32, gl::UNSIGNED_BYTE, None);
            } else {
                let width = width as i32;
                let height = height as i32;
                let format = format as u32;
                let array: TypedArray<u8> = data.into();
                gl_ctx.tex_image2_d_3(gl::TEXTURE_2D, 0, gl::RGBA as i32, width, height, 0, format, gl::UNSIGNED_BYTE, array);
            };
            gl_ctx.generate_mipmap(gl::TEXTURE_2D);
            Ok(ImageData { id, data: texture, width, height })
        } else {
            unreachable!();
        }
    }

    unsafe fn destroy_texture(data: &mut ImageData) where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.delete_texture(Some(&data.data));
        }
    }

    unsafe fn create_surface(image: &Image) -> Result<SurfaceData> where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            let surface = SurfaceData {
                framebuffer: try_opt(gl_ctx.create_framebuffer(), "Create GL framebuffer")?
            };
            gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&surface.framebuffer));
            gl_ctx.framebuffer_texture2_d(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, Some(&image.data().data), 0);
            gl_ctx.draw_buffers(&[gl::COLOR_ATTACHMENT0]);
            Ok(surface)
        } else {
            unreachable!();
        }
    }
    
    unsafe fn bind_surface(surface: &Surface) -> [i32; 4] where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            let viewport_data = gl_ctx.get_parameter(gl::VIEWPORT);
            let viewport = [
                js! { @{&viewport_data}[0] }.try_into().expect("Malformed GL viewport attribute"),
                js! { @{&viewport_data}[1] }.try_into().expect("Malformed GL viewport attribute"),
                js! { @{&viewport_data}[2] }.try_into().expect("Malformed GL viewport attribute"),
                js! { @{&viewport_data}[3] }.try_into().expect("Malformed GL viewport attribute"),
            ];
            gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, Some(&surface.data.framebuffer));
            gl_ctx.viewport(0, 0, surface.image.source_width() as i32, surface.image.source_height() as i32);
            viewport
        } else {
            unreachable!();
        }
    }

    unsafe fn unbind_surface(_surface: &Surface, viewport: &[i32]) where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.bind_framebuffer(gl::FRAMEBUFFER, None); 
            gl_ctx.viewport(viewport[0], viewport[1], viewport[2], viewport[3]);
        }
    }

    unsafe fn destroy_surface(surface: &SurfaceData) where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.delete_framebuffer(Some(&surface.framebuffer));
        }
    }

    unsafe fn viewport(x: i32, y: i32, width: i32, height: i32) where Self: Sized {
        if let Some(ref gl_ctx) = GL_CONTEXT {
            gl_ctx.viewport(x, y, width, height);
        }
    }
}

impl Drop for WebGLBackend {
    fn drop(&mut self) { 
        unsafe {
            if let Some(ref gl_ctx) = GL_CONTEXT {
                gl_ctx.delete_program(Some(&self.shader));
                gl_ctx.delete_shader(Some(&self.fragment));
                gl_ctx.delete_shader(Some(&self.vertex));
                gl_ctx.delete_buffer(Some(&self.vbo));
                gl_ctx.delete_buffer(Some(&self.ebo));
            }
        }
    }
}

