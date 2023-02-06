/// Basic wrapper for a [Vertex Array Object](https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object).
pub struct VertexArray(pub u32);

impl VertexArray {
    /// Creates a new vertex array object. If the operation fails `None` is
    /// returned.
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut vao);
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


