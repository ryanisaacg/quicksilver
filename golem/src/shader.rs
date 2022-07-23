use crate::*;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use core::mem::size_of;
use core::ops::Range;

/// The parameters to create a [`ShaderProgram`]
pub struct ShaderDescription<'a> {
    /// The inputs to the vertex shader stage, which are also the inputs to the whole shader
    pub vertex_input: &'a [Attribute],
    /// The inputs to the fragment shader stage, which are also the outputs from the vertex shader
    pub fragment_input: &'a [Attribute],
    /// The uniform values available to all shader stages, across all vertices of a draw call
    ///
    /// Uniforms can be bound with [`ShaderProgram::set_uniform`]
    pub uniforms: &'a [Uniform],
    /// The text of the vertex shader stage
    ///
    /// Do not include the vertex inputs, outputs, or uniforms, use the [`vertex_input`],
    /// [`fragment_input`], and [`uniforms`] fields instead. Just provide the 'main' function, as
    /// well as any helpers. The shader inputs, outputs, and uniforms will be generated for you.
    ///
    /// The inputs to this stage are defined as the [`vertex_input`] and the ouptuts are the
    /// [`fragment_input`] as well as `gl_Position`, a vec4 that represents the vertex's position.
    ///
    /// [`vertex_input`]: ShaderDescription::vertex_input
    /// [`fragment_input`]: ShaderDescription::fragment_input
    /// [`uniforms`]: ShaderDescription::uniforms
    pub vertex_shader: &'a str,
    /// The text of the fragment shader stage
    ///
    /// See the documentation of the [`vertex_shader`]. The inputs to this stage are
    /// defined as the [`fragment_input`] and the ouptut is `gl_FragColor`, a vec4 that represents
    /// the RGBA color of the fragment. Use the function `texture` to read values from GLSL
    /// textures; it will be converted to `texture2D` on the web backend.
    ///
    /// [`vertex_shader`]: ShaderDescription::vertex_shader
    /// [`fragment_input`]: ShaderDescription::fragment_input
    pub fragment_shader: &'a str,
}

/// A GPU program that draws data to the screen
pub struct ShaderProgram {
    ctx: crate::Context,
    id: GlProgram,
    vertex: GlShader,
    fragment: GlShader,
    input: Vec<Attribute>,
}

fn generate_shader_text(
    is_vertex: bool,
    body: &str,
    inputs: &[Attribute],
    outputs: &[Attribute],
    uniforms: &[Uniform],
) -> String {
    let mut shader = String::new();

    #[cfg(not(target_arch = "wasm32"))]
    shader.push_str("#version 150\n");

    shader.push_str("precision mediump float;\n");
    for attr in inputs.iter() {
        attr.as_glsl(is_vertex, Position::Input, &mut shader);
    }
    for attr in outputs.iter() {
        attr.as_glsl(is_vertex, Position::Output, &mut shader);
    }
    for uniform in uniforms.iter() {
        uniform.as_glsl(&mut shader);
    }
    shader.push_str(body);

    shader
}

impl ShaderProgram {
    /// Create a shader program with the given [`ShaderDescription`]
    pub fn new(ctx: &Context, desc: ShaderDescription) -> Result<ShaderProgram, GolemError> {
        let gl = &ctx.0.gl;
        unsafe {
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml
            // Errors:
            // 1. An error occurred creating the shader (handled by glow's error layer)
            // 2. An invalid value was passed (VERTEX_SHADER is valid)
            let vertex = gl.create_shader(glow::VERTEX_SHADER)?;
            let vertex_source = generate_shader_text(
                true,
                desc.vertex_shader,
                desc.vertex_input,
                desc.fragment_input,
                desc.uniforms,
            );
            log::debug!("Vertex shader source: {}", vertex_source);
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml
            // Errror conditions:
            // 1 & 2. Vertex isn't a GL shader (it always will be)
            // 3. Shader size is handled by glow
            gl.shader_source(vertex, &vertex_source);
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml
            // Errror conditions: Vertex isn't a GL shader (it always will be)
            gl.compile_shader(vertex);
            if !gl.get_shader_compile_status(vertex) {
                let info = gl.get_shader_info_log(vertex);
                log::error!("Failed to compile vertex shader: {}", info);
                return Err(GolemError::ShaderCompilationError(info));
            }
            log::trace!("Compiled vertex shader succesfully");

            // For GL pre/post condition explanations, see vertex shader compilation above
            let fragment = gl.create_shader(glow::FRAGMENT_SHADER)?;
            // Handle creating the output color and giving it a name, but only on desktop gl
            #[cfg(target_arch = "wasm32")]
            let (fragment_output, fragment_body) =
                { (&[], &desc.fragment_shader.replace("texture", "texture2D")) };

            #[cfg(not(target_arch = "wasm32"))]
            let (fragment_output, fragment_body) = {
                (
                    &[Attribute::new(
                        "outputColor",
                        AttributeType::Vector(Dimension::D4),
                    )],
                    &desc.fragment_shader.replace("gl_FragColor", "outputColor"),
                )
            };
            let fragment_source = generate_shader_text(
                false,
                fragment_body,
                desc.fragment_input,
                fragment_output,
                desc.uniforms,
            );
            log::debug!("Fragment shader source: {}", fragment_source);
            gl.shader_source(fragment, &fragment_source);
            gl.compile_shader(fragment);
            if !gl.get_shader_compile_status(fragment) {
                let info = gl.get_shader_info_log(fragment);
                log::error!("Failed to compile vertex shader: {}", info);
                return Err(GolemError::ShaderCompilationError(info));
            }
            log::trace!("Compiled fragment shader succesfully");

            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml
            // Failing to create a program is handled by glow
            let id = gl.create_program()?;

            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml
            // Errors:
            // 1, 2, 3: id, vertex, and fragment are all assigned to once, by the correct GL calls
            // 4: vertex and fragment are generated then immediately attached exactly once
            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);

            // Bind the color output for desktop GL
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindFragDataLocation.xhtml
            // Errors:
            // 1. colorNumber will always be 0, and therefore cannot overrun the bounds
            // 2. 'outputColor' does not started with the reserved 'gl_' prefix
            // 3. 'id' is generated by create_program above
            #[cfg(not(target_arch = "wasm32"))]
            gl.bind_frag_data_location(id, 0, "outputColor");

            for (index, attr) in desc.vertex_input.iter().enumerate() {
                gl.bind_attrib_location(id, index as u32, attr.name());
            }

            gl.link_program(id);
            if !gl.get_program_link_status(id) {
                let info = gl.get_program_info_log(id);
                log::error!("Failed to link the shader program: {}", info);
                return Err(GolemError::ShaderCompilationError(info));
            }
            log::trace!("Linked shader program succesfully");

            Ok(ShaderProgram {
                ctx: Context(ctx.0.clone()),
                id,
                vertex,
                fragment,
                input: desc.vertex_input.to_vec(),
            })
        }
    }

    /// Check if this shader program is currently bound to be operated on
    pub fn is_bound(&self) -> bool {
        match *self.ctx.0.current_program.borrow() {
            Some(program) => self.id == program,
            None => false,
        }
    }

    /// Set a uniform value, assuming the shader is bound by [`ShaderProgram::bind`]
    pub fn set_uniform(&self, name: &str, uniform: UniformValue) -> Result<(), GolemError> {
        if self.is_bound() {
            let gl = &self.ctx.0.gl;
            let location = unsafe { gl.get_uniform_location(self.id, name) };
            if location.is_none() {
                return Err(GolemError::NoSuchUniform(name.to_owned()));
            }
            use UniformValue::*;
            unsafe {
                match uniform {
                    Int(x) => gl.uniform_1_i32(location, x),
                    IVector2([x, y]) => gl.uniform_2_i32(location, x, y),
                    IVector3([x, y, z]) => gl.uniform_3_i32(location, x, y, z),
                    IVector4([x, y, z, w]) => gl.uniform_4_i32(location, x, y, z, w),
                    Float(x) => gl.uniform_1_f32(location, x),
                    Vector2([x, y]) => gl.uniform_2_f32(location, x, y),
                    Vector3([x, y, z]) => gl.uniform_3_f32(location, x, y, z),
                    Vector4([x, y, z, w]) => gl.uniform_4_f32(location, x, y, z, w),
                    Matrix2(mat) => gl.uniform_matrix_2_f32_slice(location, false, &mat),
                    Matrix3(mat) => gl.uniform_matrix_3_f32_slice(location, false, &mat),
                    Matrix4(mat) => gl.uniform_matrix_4_f32_slice(location, false, &mat),
                }
            }

            Ok(())
        } else {
            Err(GolemError::NotCurrentProgram)
        }
    }

    /// Bind this shader to use it, either to [`set a uniform`] or to [`draw`]
    ///
    /// [`set a uniform`]: ShaderProgram::set_uniform
    /// [`draw`]: ShaderProgram::draw
    pub fn bind(&mut self) {
        let gl = &self.ctx.0.gl;
        log::trace!("Binding the shader and buffers");
        unsafe {
            gl.use_program(Some(self.id));
        }
        *self.ctx.0.current_program.borrow_mut() = Some(self.id);
    }

    /// Draw the given elements from the element buffer with this shader
    ///
    /// The range should fall within the elements of the buffer (which is checked for via an
    /// `assert!`.) The GeometryMode determines what the set of indices produces: triangles
    /// consumes 3 vertices into a filled triangle, lines consumes 2 vertices into a thin line,
    /// etc.
    ///
    /// The `ShaderProgram` must be bound first, see [`ShaderProgram::bind`].
    ///
    /// # Safety
    ///
    /// The safety concerns to keep in mind:
    ///
    /// 1. The elements in the [`ElementBuffer`] are not checked against the size of the
    ///    [`VertexBuffer`]. If they are illegal indices, this will result in out-of-bounds reads on
    ///    the GPU and therefore undefined behavior. The caller is responsible for ensuring all
    ///    elements are valid and in-bounds.
    ///
    /// [`Surface::bind`]: crate::Surface::bind
    pub unsafe fn draw(
        &self,
        vb: &VertexBuffer,
        eb: &ElementBuffer,
        range: Range<usize>,
        geometry: GeometryMode,
    ) -> Result<(), GolemError> {
        assert!(
            range.end <= eb.size(),
            "The range exceeded the size of the element buffer"
        );
        // prepare_draw also takes care of ensuring this program is current
        self.prepare_draw(vb, eb)?;
        self.draw_prepared(range, geometry);
        Ok(())
    }

    /// Set up a [`VertexBuffer`] and [`ElementBuffer`] to draw multiple times with the same
    /// buffers.
    ///
    /// The `ShaderProgram` must be bound first, see [`ShaderProgram::bind`].
    ///
    /// See [`ShaderProgram::draw_prepared`] to execute the draw calls. If you're only drawing the
    /// buffers once before replacing their data, see [`ShaderProgram::draw`].
    pub fn prepare_draw(&self, vb: &VertexBuffer, eb: &ElementBuffer) -> Result<(), GolemError> {
        if !self.is_bound() {
            Err(GolemError::NotCurrentProgram)
        } else {
            eb.bind();
            vb.bind();
            let stride: i32 = self.input.iter().map(|attr| attr.size()).sum();
            let stride = stride * size_of::<f32>() as i32;
            let mut offset = 0;
            log::trace!("Binding the attributes to draw");
            let gl = &self.ctx.0.gl;
            for (index, attr) in self.input.iter().enumerate() {
                let size = attr.size();
                unsafe {
                    let pos_attrib = index as u32;
                    gl.enable_vertex_attrib_array(pos_attrib);
                    gl.vertex_attrib_pointer_f32(
                        pos_attrib,
                        size,
                        glow::FLOAT,
                        false,
                        stride,
                        offset,
                    );
                }
                offset += size * size_of::<f32>() as i32;
            }
            // Disable any dangling vertex attributes
            let current_max_attrib = self.input.len() as u32;
            let previous_max_attrib = self.ctx.max_attrib(current_max_attrib);
            for i in current_max_attrib..previous_max_attrib {
                unsafe {
                    gl.disable_vertex_attrib_array(i);
                }
            }

            Ok(())
        }
    }

    /// Draw the given elements from the prepared element buffer with this shader
    ///
    /// This relies on the caller having a valid prepared state: see [`prepare_draw`].
    ///
    /// # Safety
    ///
    /// The safety concerns to keep in mind:
    ///
    /// 1. [`prepare_draw`] *must* be called before this method, and the buffers passed to it
    ///    *must* not have their underlying storage changed. Their values can change, but calls to
    ///    `set_data` may cause them to expand and move to a new memory location on the GPU,
    ///    invalidating the cal to preparation. Some calls to [`set_data`] are optimized to calls
    ///    to [`set_sub_data`]; do not rely on this implementation detail.
    /// 2. No other buffers may be operated on between [`prepare_draw`] and `draw_prepared`. Any
    ///    calls to [`set_data`] or [`set_sub_data`] from a buffer that wasn't passed to
    ///    [`prepare_draw`] will result in the wrong buffer being bound when `draw_prepared` is
    ///    called.
    /// 3. The elements in the prepared buffer must correspond to valid locations within the vertex
    ///    buffer. See [`draw`] for details.
    /// 4. This shader must still be bound (see [`bind`])
    ///
    /// [`prepare_draw`]: ShaderProgram::prepare_draw
    /// [`draw`]: ShaderProgram::draw
    /// [`bind`]: ShaderProgram::bind
    /// [`set_data`]: crate::Buffer::set_data
    /// [`set_sub_data`]: crate::Buffer::set_sub_data
    pub unsafe fn draw_prepared(&self, range: Range<usize>, geometry: GeometryMode) {
        log::trace!("Dispatching draw command");
        let length = range.end - range.start;
        self.ctx.0.gl.draw_elements(
            ShaderProgram::shape_type(geometry),
            length as i32,
            glow::UNSIGNED_INT,
            (range.start * size_of::<u32>()) as i32,
        );
    }

    fn shape_type(geometry: GeometryMode) -> u32 {
        use GeometryMode::*;
        match geometry {
            Points => glow::POINTS,
            Lines => glow::LINES,
            LineStrip => glow::LINE_STRIP,
            LineLoop => glow::LINE_LOOP,
            TriangleStrip => glow::TRIANGLE_STRIP,
            TriangleFan => glow::TRIANGLE_FAN,
            Triangles => glow::TRIANGLES,
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        let gl = &self.ctx.0.gl;
        unsafe {
            gl.delete_program(self.id);
            gl.delete_shader(self.fragment);
            gl.delete_shader(self.vertex);
        }
    }
}
