use crate::blend::{BlendEquation, BlendFunction, BlendMode};
use crate::depth::DepthTestMode;
use crate::{GlFramebuffer, GlProgram, GlVertexArray, GolemError};
use alloc::rc::Rc;
use core::cell::RefCell;
use glow::HasContext;

/// The context required to interact with the GPU
pub struct Context(pub(crate) Rc<ContextContents>);

pub(crate) struct ContextContents {
    pub(crate) gl: glow::Context,
    pub(crate) current_program: RefCell<Option<GlProgram>>,
    pub(crate) current_surface: RefCell<Option<GlFramebuffer>>,
    vao: GlVertexArray,
    max_vertex_attrib_index: RefCell<u32>,
}

impl Drop for ContextContents {
    fn drop(&mut self) {
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml
        // glow handles passing in the pointer to our value, and GL will silently ignore invalid
        // values
        unsafe {
            self.gl.delete_vertex_array(self.vao);
        }
    }
}

impl Context {
    /// Create an instance from an OpenGL context
    pub fn from_glow(gl: glow::Context) -> Result<Context, GolemError> {
        let vao = unsafe {
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGenVertexArrays.xhtml
            // glow handles passing in '1' and returning the value to us
            let vao = gl.create_vertex_array()?;
            // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml
            // In this case, we know 'vao' must be a valid vao because we just constructed it
            gl.bind_vertex_array(Some(vao));

            vao
        };

        let contents = Context(Rc::new(ContextContents {
            gl,
            current_program: RefCell::new(None),
            current_surface: RefCell::new(None),
            vao,
            max_vertex_attrib_index: RefCell::new(0),
        }));
        contents.set_clear_color(0.0, 0.0, 0.0, 1.0);

        Ok(contents)
    }

    /// Set the section of the framebuffer that will be rendered to
    ///
    /// By default, this is the entire internal area of the window. When switching to a
    /// [`Surface`], it's generally important to set the viewport to its area.
    ///
    /// [`Surface`]: crate::Surface
    pub fn set_viewport(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.0
                .gl
                .viewport(x as i32, y as i32, width as i32, height as i32);
        }
    }

    /// Set the section of the framebuffer that will be affected by rendering operations.
    /// Rendering operations won't have any effect on pixels outside this section. Unlike the
    /// `set_viewport` method, this will *not* affect the scale of the rendered content.
    ///
    /// By default, the scissor is disabled, which means that rendering operations can draw on
    /// the entire viewport. You can use `disable_scissor` to disable it again.
    pub fn set_scissor(&self, x: u32, y: u32, width: u32, height: u32) {
        unsafe {
            self.0.gl.enable(glow::SCISSOR_TEST);
            self.0
                .gl
                .scissor(x as i32, y as i32, width as i32, height as i32);
        }
    }

    /// Disables the scissor (see the `set_scissor` method).
    ///
    /// This method has no effect if the scissor is already disabled.
    pub fn disable_scissor(&self) {
        unsafe {
            self.0.gl.disable(glow::SCISSOR_TEST);
        }
    }

    /// Set the color the render target will be cleared to by [`clear`]
    ///
    /// [`clear`]: Context::clear
    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearColor.xhtml
        // Set the clear color to (r, g, b, a)
        unsafe {
            self.0.gl.clear_color(r, g, b, a);
        }
    }

    /// Clear the current render target to the render color (see [`set_clear_color`])
    ///
    /// [`set_clear_color`]: Context::set_clear_color
    pub fn clear(&self) {
        let gl = &self.0.gl;
        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    /// Set the blend mode, with `None` disabling blending
    ///
    /// By default, this is `None`
    ///
    /// See the documentation for [`BlendMode`] for the various blending options
    pub fn set_blend_mode(&self, blend_state: Option<BlendMode>) {
        let gl = &self.0.gl;
        match blend_state {
            Some(BlendMode {
                equation,
                function,
                global_color: [r, g, b, a],
            }) => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                // gl::BLEND is on the whitelist
                gl.enable(glow::BLEND);

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquation.xhtml
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendEquationSeparate.xhtml
                // The to_gl() function only produces valid values
                match equation {
                    BlendEquation::Same(eq) => gl.blend_equation(eq.to_gl()),
                    BlendEquation::Separate { color, alpha } => {
                        gl.blend_equation_separate(color.to_gl(), alpha.to_gl());
                    }
                }

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFunc.xhtml
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendFuncSeparate.xhtml
                // The to_gl() function only produces valid values
                match function {
                    BlendFunction::Same {
                        source,
                        destination,
                    } => {
                        gl.blend_func(source.to_gl(), destination.to_gl());
                    }
                    BlendFunction::Separate {
                        source_color,
                        source_alpha,
                        destination_alpha,
                        destination_color,
                    } => {
                        gl.blend_func_separate(
                            source_color.to_gl(),
                            source_alpha.to_gl(),
                            destination_alpha.to_gl(),
                            destination_color.to_gl(),
                        );
                    }
                }

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBlendColor.xhtml
                gl.blend_color(r, g, b, a);
            },
            None => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                // gl::BLEND is on the whitelist
                gl.disable(glow::BLEND);
            },
        }
    }

    /// Set the depth test mode, with `None` disabling depth testing
    ///
    /// By default, this is `None`
    ///
    /// See the documentation for [`DepthTestMode`](depth/struct.DepthTestMode.html)
    /// for the various depth testing options
    pub fn set_depth_test_mode(&self, depth_test_state: Option<DepthTestMode>) {
        let gl = &self.0.gl;
        match depth_test_state {
            Some(DepthTestMode {
                function,
                range_near,
                range_far,
                depth_mask,
            }) => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                gl.enable(glow::DEPTH_TEST);
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthFunc.xhtml
                // The to_gl() function only produces valid values
                gl.depth_func(function.to_gl());

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthRange.xhtml
                #[cfg(not(target_arch = "wasm32"))]
                gl.depth_range_f64(range_near as f64, range_far as f64);

                // https://www.khronos.org/registry/OpenGL-Refpages/es3.0/html/glDepthRangef.xhtml
                #[cfg(target_arch = "wasm32")]
                gl.depth_range_f32(range_near, range_far);

                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDepthMask.xhtml
                gl.depth_mask(depth_mask);
            },
            None => unsafe {
                // https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glEnable.xhtml
                gl.disable(glow::DEPTH_TEST);
            },
        }
    }

    /// Set the new max attribute, clear the old one
    pub(crate) fn max_attrib(&self, index: u32) -> u32 {
        let mut attrib_ptr = self.0.max_vertex_attrib_index.borrow_mut();
        let attrib = *attrib_ptr;
        *attrib_ptr = index;

        attrib
    }
}
