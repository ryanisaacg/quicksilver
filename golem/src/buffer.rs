use crate::*;

/// A buffer to store the vertices on the GPU
pub type VertexBuffer = Buffer<f32>;

/// A buffer to store the indices that make up the geometry elements
///
/// See the [`GeometryMode`] for how the elements will be interpreted.
///
/// [`GeometryMode`]: crate::GeometryMode
pub type ElementBuffer = Buffer<u32>;

/// A collection of values stored contiguously on the GPU, as either a [`VertexBuffer`] or an
/// [`ElementBuffer`]
///
/// Buffers serve as input to [`draw calls`], by passing the input to the [`ShaderProgram`]
///
/// [`ShaderProgram`]: crate::ShaderProgram
/// [`draw calls`]: crate::ShaderProgram::draw
pub struct Buffer<T> {
    ctx: Context,
    id: GlBuffer,
    length: usize,
    target: u32,
    _p: core::marker::PhantomData<T>,
}

impl Buffer<f32> {
    /// Create a [`VertexBuffer`] to store the vertex values
    pub fn new(ctx: &Context) -> Result<Self, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_buffer() }?;

        Ok(Buffer {
            ctx,
            id,
            length: 0,
            target: glow::ARRAY_BUFFER,
            _p: core::marker::PhantomData,
        })
    }
}

impl Buffer<u32> {
    /// Create a [`ElementBuffer`] to store the index values
    pub fn new(ctx: &Context) -> Result<Self, GolemError> {
        let ctx = Context(ctx.0.clone());
        let id = unsafe { ctx.0.gl.create_buffer() }?;

        Ok(Buffer {
            ctx,
            id,
            length: 0,
            target: glow::ELEMENT_ARRAY_BUFFER,
            _p: core::marker::PhantomData,
        })
    }
}

impl<T: bytemuck::Pod> Buffer<T> {
    pub(crate) fn bind(&self) {
        unsafe {
            self.ctx.0.gl.bind_buffer(self.target, Some(self.id));
        }
    }

    /// The current capacity of the buffer in bytes
    pub fn size(&self) -> usize {
        self.length
    }

    /// Set the data this buffer holds, resizing it if necessary
    ///
    /// The conditions under which the buffer is reallocated are an implementation detail, and it's
    /// best not to rely on them.
    pub fn set_data(&mut self, data: &[T]) {
        let gl = &self.ctx.0.gl;

        let u8_buffer = bytemuck::cast_slice(data);
        let data_length = u8_buffer.len();
        self.bind();
        if data_length >= self.length {
            log::trace!("Resizing buffer to hold new data");
            let new_length = data_length * 2;
            unsafe {
                gl.buffer_data_size(self.target, new_length as i32, glow::STREAM_DRAW);
            }
            self.length = new_length;
        }
        log::trace!("Writing data to OpenGL buffer");
        unsafe {
            gl.buffer_sub_data_u8_slice(self.target, 0, u8_buffer);
        }
    }

    /// Set some range of the buffer, within the existing capacity
    ///
    /// The range (the start to the end of the data) must fall within the existing buffer's
    /// capacity, or this method will panic.
    pub fn set_sub_data(&self, start: usize, data: &[T]) {
        let u8_buffer = bytemuck::cast_slice(data);
        let data_length = u8_buffer.len();
        assert!(
            start + data_length < self.length,
            "The data runs past the end of the buffer"
        );
        log::trace!("Writing data to OpenGL buffer");
        unsafe {
            self.ctx
                .0
                .gl
                .buffer_sub_data_u8_slice(self.target, start as i32, u8_buffer);
        }
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        unsafe {
            self.ctx.0.gl.delete_buffer(self.id);
        }
    }
}
