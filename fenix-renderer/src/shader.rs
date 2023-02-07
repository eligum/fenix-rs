//! TODO: Add module documentation when this project grows.

use gl;
use glam::{Mat3, Mat4, Vec2, Vec3, Vec4};
use std::{collections::HashMap, fs, ffi::CString};

/// The types of shader.
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
}

/// A handle to a [Shader Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Shader_objects).
pub struct Shader {
    id: u32,
}

impl Shader {
    /// Creates a new shader.
    ///
    /// Prefer the [`Shader::from_source`](Shader::from_source) method.
    ///
    /// You probably want to skip the direct creation of the shader object and
    /// use [`ShaderProgram::from_source`](ShaderProgram::from_source) or
    /// [`ShaderProgram::from_file`](ShaderProgram::from_file) instead.
    pub fn new(st: ShaderType) -> Option<Self> {
        let shader_id = unsafe { gl::CreateShader(st as u32) };
        if shader_id != 0 {
            Some(Self { id: shader_id })
        } else {
            None
        }
    }

    /// Assigns a source string to the shader.
    ///
    /// Replaces any previously assigned source.
    pub fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.id,
                1,
                &(src.as_bytes().as_ptr() as *const i8),
                &(src.len() as i32),
            );
        }
    }

    /// Compiles the shader based on the assigned source.
    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.id) };
    }

    /// Checks if the last compile was successful or not.
    pub fn is_successfully_compiled(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compiled) };
        compiled == gl::TRUE as i32
    }

    /// Gets the info log for the shader.
    pub fn get_info_log(&self) -> String {
        let mut buffer_len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut buffer_len) };
        let mut info_log: Vec<u8> = Vec::with_capacity(buffer_len as usize);
        let mut written_len: i32 = 0;
        unsafe {
            gl::GetShaderInfoLog(
                self.id,
                info_log.capacity() as i32,
                &mut written_len,
                info_log.as_mut_ptr() as *mut i8,
            );
            info_log.set_len(written_len as usize);
        }
        String::from_utf8_lossy(&info_log).into_owned()
    }

    /// Marks a shader for deletion.
    ///
    /// Note: This **does not** immediately delete the shader. It only marks it
    /// for deletion. If the shader has been previously attached to a
    /// program then the shader will stay allocated until it's unattached
    /// from that program.
    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.id) };
    }

    /// Creates and compiles a shader of the given type from a source string.
    pub fn from_source(st: ShaderType, source: &str) -> Result<Self, String> {
        let shader = Self::new(st).ok_or_else(|| String::from("Couldn't allocate new shader"))?;
        shader.set_source(source);
        shader.compile();
        if shader.is_successfully_compiled() {
            Ok(shader)
        } else {
            let log = shader.get_info_log();
            shader.delete();
            let st: &str = match st {
                ShaderType::Vertex => "VERTEX",
                ShaderType::Fragment => "FRAGMENT",
                ShaderType::Geometry => "GEOMETRY",
            };
            Err(format!("Failed to compile {} shader:\n{}", st, log))
        }
    }
}

/// A handle to a [Program Object](https://www.khronos.org/opengl/wiki/GLSL_Object#Program_objects).
pub struct ShaderProgram {
    id: u32,
    locations: HashMap<String, i32>,
}

impl ShaderProgram {
    /// Allocates a new program object.
    ///
    /// Prefer [`ShaderProgram::from_file`](ShaderProgram::from_file),
    /// it makes a complete program from GLSL source files.
    pub fn new() -> Option<Self> {
        let program_id = unsafe { gl::CreateProgram() };
        if program_id != 0 {
            Some(Self {
                id: program_id,
                locations: HashMap::new(),
            })
        } else {
            None
        }
    }

    /// Attaches a shader object to this program.
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.id, shader.id) };
    }

    /// Links the various attached, compiled shader objects into a usable
    /// program.
    pub fn link(&self) {
        unsafe { gl::LinkProgram(self.id) };
    }

    /// Checks if the last linking operation was successful.
    pub fn is_linked_successfully(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success) };
        success == gl::TRUE as i32
    }

    /// Gets the log data for this program.
    ///
    /// This is usually used to check the message when a program failed to link.
    pub fn get_info_log(&self) -> String {
        let mut buffer_len = 0;
        unsafe { gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut buffer_len) };
        let mut info_log: Vec<u8> = Vec::with_capacity(buffer_len as usize);
        let mut written_len = 0_i32;
        unsafe {
            gl::GetProgramInfoLog(
                self.id,
                info_log.capacity() as i32,
                &mut written_len,
                info_log.as_mut_ptr() as *mut i8,
            );
            info_log.set_len(written_len as usize);
        }
        String::from_utf8_lossy(&info_log).into_owned()
    }

    /// Sets this program as the program to use when drawing.
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    /// Unbinds this program.
    pub fn unbind(&self) {
        unsafe { gl::UseProgram(0) };
    }

    /// Marks the program for deletion.
    ///
    /// Note: This **does not** immediately delete the program. If the program
    /// is currently in use it won't be deleted until it's not the active
    /// program. When a program is finally deleted and attached shaders are
    /// unattached.
    pub fn delete(self) {
        unsafe { gl::DeleteProgram(self.id) };
    }

    /// Takes a vertex shader source string and a fragment shader source string
    /// and either gets you a working program object or returns an error.
    ///
    /// This is the preferred way to create a simple shader program in the
    /// common case. It's just less error prone than doing all the steps
    /// yourself.
    pub fn from_source(
        vert_src: &str,
        frag_src: &str,
        geom_src: Option<&str>,
    ) -> Result<Self, String> {
        let program = Self::new().ok_or_else(|| String::from("Couldn't allocate a program"))?;
        let vert_shader = Shader::from_source(ShaderType::Vertex, vert_src)?;
        let frag_shader = Shader::from_source(ShaderType::Fragment, frag_src)?;
        program.attach_shader(&vert_shader);
        program.attach_shader(&frag_shader);
        if let Some(source) = geom_src {
            let geom_shader = Shader::from_source(ShaderType::Geometry, source)?;
            program.attach_shader(&geom_shader);
        }
        program.link();
        if program.is_linked_successfully() {
            Ok(program)
        } else {
            let log = program.get_info_log();
            program.delete();
            Err(format!("Failed to link PROGRAM:\n{}", log))
        }
    }

    /// Takes two file paths and possibly a third containing GLSL code and
    /// compiles them into a ShaderProgram. If a problem occurs during this
    /// process an error is returned with a string that has information
    /// about the error.
    ///
    /// This is the preferred way to create a shader program in most cases. It's
    /// just less error prone than doing all the steps yourself.
    pub fn from_file(
        vert_path: &str,
        frag_path: &str,
        geom_path: Option<&str>,
    ) -> Result<Self, String> {
        let vert_src = extract_source(vert_path)?;
        let frag_src = extract_source(frag_path)?;

        if let Some(path) = geom_path {
            let geom_src = extract_source(path)?;
            Self::from_source(&vert_src, &frag_src, Some(&geom_src))
        } else {
            Self::from_source(&vert_src, &frag_src, None)
        }
    }

    /// Getter.
    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        let name_cstr = CString::new(name).unwrap();
        match self.locations.get(name) {
            Some(&location) => location,
            None => {
                let location = unsafe {
                    gl::GetUniformLocation(self.id, name_cstr.as_ptr())
                };
                if location < 0 {
                    panic!("Failed to get location of uniform '{}'", name);
                } else {
                    self.locations.insert(name.to_string(), location);
                }
                location
            },
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_1i(&mut self, name: &str, value: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), value);
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_1i_arr(&mut self, name: &str, values: &[i32]) {
        unsafe {
            gl::Uniform1iv(
                self.get_uniform_location(name),
                values.len() as i32,
                values.as_ptr() as *const i32,
            );
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_1f(&mut self, name: &str, value: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), value);
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_2f(&mut self, name: &str, v: Vec2) {
        unsafe {
            gl::Uniform2f(self.get_uniform_location(name), v.x, v.y);
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_3f(&mut self, name: &str, v: Vec3) {
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), v.x, v.y, v.z);
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_4f(&mut self, name: &str, v: Vec4) {
        unsafe {
            gl::Uniform4f(self.get_uniform_location(name), v.x, v.y, v.z, v.w);
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_1f_arr(&mut self, name: &str, values: &[f32]) {
        unsafe {
            gl::Uniform1fv(
                self.get_uniform_location(name),
                values.len() as i32,
                values.as_ptr() as *const f32,
            );
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_mat3(&mut self, name: &str, matrix: Mat3) {
        let values = matrix.to_cols_array();
        unsafe {
            gl::UniformMatrix3fv(
                self.get_uniform_location(name),
                1,
                0,
                values.as_ptr() as *const f32,
            );
        }
    }

    /// Uploads a uniform to the current ShaderProgram
    pub fn set_uniform_mat4(&mut self, name: &str, matrix: Mat4) {
        let values = matrix.to_cols_array();
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                0,
                values.as_ptr() as *const f32,
            );
        }
    }
}

fn extract_source(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|_| format!("Failed to read file {}", path))
}
