//! TODO: Add module documentation when this project grows.

use std::ops::Drop;
use std::{ffi::c_void, mem, ptr};

/// Struct representing a [Buffer Object](https://www.khronos.org/opengl/wiki/Buffer_Object)
/// that stores vertex data.
pub struct VertexBuffer {
    id: u32,
}

impl VertexBuffer {
    /// Creates a new vertex buffer and fills it with the given data.
    pub fn from(data: &[f32]) -> Self {
        let mut id = 0;
        unsafe {
            // NOTE(Miguel): We use DSA https://www.khronos.org/opengl/wiki/Direct_State_Access
            // to create and fill the buffer.
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                mem::size_of_val(data) as isize,
                data.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        Self { id }
    }

    /// Creates a new vertex buffer of the specified size (in bytes).
    pub fn with_size(size: u32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );
        }
        Self { id }
    }

    /// Fills the buffer with vertex data.
    pub fn set_data() {
        todo!();
    }

    /// Binds this buffer to the target `ARRAY_BUFFER`.
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) };
    }

    /// Unbinds any buffer bound to the `ELEMENT_ARRAY_BUFFER` target.
    pub fn unbind() {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}

/// Struct representing a [Buffer Object](https://www.khronos.org/opengl/wiki/Buffer_Object)
/// that stores vertex indices.
pub struct IndexBuffer {
    id: u32,
    count: usize,
}

impl IndexBuffer {
    /// Creates a new index buffer and fills it with the given data.
    pub fn from(indices: &[u32]) -> Self {
        let mut id = 0;
        unsafe {
            gl::CreateBuffers(1, &mut id);
            gl::NamedBufferData(
                id,
                mem::size_of_val(indices) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        Self { id, count: indices.len() }
    }

    /// Returns the number (count) of indices in the buffer.
    pub fn get_count(&self) -> usize {
        self.count
    }

    /// Binds this buffer to the target `ELEMENT_ARRAY_BUFFER`.
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) };
    }

    /// Unbinds any buffer bound to the `ELEMENT_ARRAY_BUFFER` target.
    pub fn unbind() {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) };
    }
}