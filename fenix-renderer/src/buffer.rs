//! TODO: Add module documentation when this project grows.

use gl;

/// Basic wrapper for a [Vertex Array Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray(pub u32);

impl VertexArray {
    /// Creates a new vertex array object. If the operation fails `None` is
    /// returned.
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        if vao != 0 {
            Some(Self(vao))
        } else {
            None
        }
    }

    /// Binds this vertex array as the current vertex array object.
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.0) };
    }

    /// Unbinds the current vertex array object.
    pub fn unbind() {
        unsafe { gl::BindVertexArray(0) };
    }
}

/// The types of buffer object that you can have.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BufferType {
    /// Vertex Buffer holds arrays of vertex data for drawing.
    Vertex = gl::ARRAY_BUFFER,
    /// Index Buffer holds indices of what vertices to use for drawing.
    Index = gl::ELEMENT_ARRAY_BUFFER,
}

/// Basic wrapper for a [Buffer Object](https://www.khronos.org/opengl/wiki/Buffer_Object).
pub struct Buffer(pub u32);

impl Buffer {
    /// Creates a new vertex buffer.
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self(vbo))
        } else {
            None
        }
    }

    /// Binds this buffer as the current buffer for the given type.
    pub fn bind(&self, bt: BufferType) {
        unsafe { gl::BindBuffer(bt as u32, self.0) };
    }

    /// Unbinds the current buffer for the given type.
    pub fn unbind(bt: BufferType) {
        unsafe { gl::BindBuffer(bt as u32, 0) };
    }
}
