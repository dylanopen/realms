use std::ffi::CString;
use std::ptr;

use gl::types::{GLchar, GLint};

/// Enum storing the type of shader and its associated opengl integer.
/// You should pass a variant of `ShaderType` when creating a `Shader`.
#[repr(u32)]
pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
}

/// A wrapper around an opengl shader.
/// This struct currently only stores the opengl reference to the shader, but
/// is provided as a convenient wrapper for creating shaders from their source.
pub struct Shader {
    pub gl_id: u32,
}

impl Shader {
    /// Load and compile an opengl shader from the given source string and
    /// shader type.
    ///
    /// If the shader compilation failed (i.e. your shader has a syntax error),
    /// the error information is provided in the `Err` variant as a string.
    /// If you `.unwrap()` or `.expect(...)` the return value, it will print
    /// the GLSL error.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// let v_shader_src = include_str!("default.vert.glsl").to_string();
    /// let f_shader_src = include_str!("default.frag.glsl").to_string();                
    /// let v_shader = Shader::load_str(ShaderType::Vertex, v_shader_src).unwrap();         
    /// let f_shader = Shader::load_str(ShaderType::Fragment, f_shader_src).unwrap();
    /// let program = ShaderProgram::new(vec![v_shader, f_shader]).unwrap();
    /// ```
    pub fn load_str(shader_type: ShaderType, source: String) -> Result<Shader, String> {
        let gl_id = unsafe {
            let gl_id = gl::CreateShader(shader_type as u32);
            let c_source = CString::new(source.as_bytes())
                .map_err(|err| format!("Realms: failed to create CString from shader source: {}", err))?;
            gl::ShaderSource(gl_id, 1, &c_source.as_ptr(), ptr::null());
            gl::CompileShader(gl_id);
            
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(1024);
            info_log.set_len(1024 - 1); // -1 to skip trailing \0
            gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    gl_id, 1024, ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar
                );
                let gl_error = String::from_utf8_lossy(&info_log);
                let gl_error = gl_error.split_once("\0").unwrap().0;
                return Err(format!("Realms: failed to compile shader: {}", gl_error));
            }

            gl_id
        };

        Ok(Shader { gl_id })

    }
}

/// A wrapper around an opengl shader *program*.
/// This struct currently only stores the opengl reference to the shader
/// program.
/// It is provided as a convenient wrapper for creating a shader program from
/// a vector of `Shader` objects.
pub struct ShaderProgram {
    pub gl_id: u32,
}

impl ShaderProgram {
    pub const NONE: ShaderProgram = ShaderProgram { gl_id: 0 }; 

    /// Load and compile an opengl shader **program** from the given `Vec` of
    /// `Shader`s;
    ///
    /// If linking the shader program fails, the error will be returned in the
    /// `Err` variant of the result.
    /// If you `.unwrap()` or `.expect(...)` the return value, it will print
    /// the GLSL error.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// let v_shader_src = include_str!("default.vert.glsl").to_string();
    /// let f_shader_src = include_str!("default.frag.glsl").to_string();                
    /// let v_shader = Shader::load_str(ShaderType::Vertex, v_shader_src).unwrap();         
    /// let f_shader = Shader::load_str(ShaderType::Fragment, f_shader_src).unwrap();
    /// let program = ShaderProgram::new(vec![v_shader, f_shader]).unwrap();
    /// ```
    pub fn new(shaders: Vec<Shader>) -> Result<ShaderProgram, String> {
        let gl_id = unsafe {
            let gl_id = gl::CreateProgram();
            for shader in &shaders {
                gl::AttachShader(gl_id, shader.gl_id);
            }
            gl::LinkProgram(gl_id);

            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(1024);
            info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramiv(gl_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    gl_id, 1024, ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar
                );
                return Err(format!("Realms: failed to link shader program: {}", std::str::from_utf8(&info_log).unwrap()));
            }
            for shader in &shaders {
                gl::DeleteShader(shader.gl_id);
            }
            gl_id
        };
        
        Ok(ShaderProgram { gl_id })
    }

    /// Bind the shader.
    /// If you pass this shader program to the `w.new_frame()` function, this
    /// function is automatically run at the start of each frame so does not
    /// need to be manually called.
    /// If you would like to render multiple `VertexBuffer`s with different
    /// shader programs, however, you can manually call this in the middle of
    /// your loop to switch to a different shader program.
    ///
    /// In short, calling this funtion tells opengl to use this shader program
    /// to draw the vertices.
    pub fn new_frame(&self) {
        unsafe { gl::UseProgram(self.gl_id) };
    }

    /// Upload a single float to the shader as a uniform.
    /// Use this uniform in the shader using `uniform float <name>`.
    pub fn uniform_1f(&self, uniform_name: &str, data: f32) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform1f(location, data);
        }
    }

    /// Upload a vec2 of floats to the shader as a uniform.
    /// Use this uniform in the shader using `uniform vec2 <name>`.
    pub fn uniform_2f(&self, uniform_name: &str, data: (f32, f32)) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform2f(location, data.0, data.1);
        }
    }

    /// Upload a vec3 of floats to the shader as a uniform.
    /// Use this uniform in the shader using `uniform vec3 <name>`.
    pub fn uniform_3f(&self, uniform_name: &str, data: (f32, f32, f32)) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform3f(location, data.0, data.1, data.2);
        }
    }

    /// Upload a vec4 of floats to the shader as a uniform.
    /// Use this uniform in the shader using `uniform vec4 <name>`.
    pub fn uniform_4f(&self, uniform_name: &str, data: (f32, f32, f32, f32)) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform4f(location, data.0, data.1, data.2, data.3);
        }
    }

    /// Upload a single integer (i32) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform int <name>`.
    pub fn uniform_1i(&self, uniform_name: &str, data: i32) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform1i(location, data);
        }
    }

    /// Upload a vec2 of integers (i32s) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform ivec2 <name>`.
    pub fn uniform_2i(&self, uniform_name: &str, data: (i32, i32)) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform2i(location, data.0, data.1);
        }
    }

    /// Upload a vec3 of integers (i32s) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform ivec3 <name>`.
    pub fn uniform_3i(&self, uniform_name: &str, data: (i32, i32, i32)) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform3i(location, data.0, data.1, data.2);
        }
    }

    /// Upload a vec4 of integers (i32s) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform ivec4 <name>`.
    pub fn uniform_4i(&self, uniform_name: &str, data: (i32, i32, i32, i32)) {
        unsafe {
            let location = gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr());
            gl::Uniform4i(location, data.0, data.1, data.2, data.3);
        }
    }
}

