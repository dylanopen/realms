//! The `shader` module stores structs and functions used to manage opengl
//! shaders used to draw vertices.
//! It has two primary structs: `Shader` and `ShaderProgram`.

extern crate alloc;
use alloc::ffi::CString;
use core::ptr;

use gl::types::{GLchar, GLint};

/// Enum storing the type of shader and its associated opengl integer.
/// You should pass a variant of `ShaderType` when creating a `Shader`.
#[repr(u32)]
#[non_exhaustive]
pub enum ShaderType {

    /// `Vertex` shaders alter the *position* of vertices.
    /// For example, a vertex shader could double the size of a shape by
    /// doubling each coordinate, or be used to animate objects (e.g. trees
    /// swaying in the wind).
    ///
    /// The code for a basic vertex shader which simply outputs the 2D
    /// coordinates stored the `VertexBuffer` (but as a 3D vertex, a `vec4`)
    /// can be found in example 3, at
    /// <https://github.com/dylanopen/realms/tree/main/examples/example3_colorful_triangle/src/vertex.glsl>
    /// 
    /// Read more: <https://www.khronos.org/opengl/wiki/Vertex_Shader>
    Vertex = gl::VERTEX_SHADER,

    /// `Vertex` shaders determine the *color of each pixel* on the screen.
    /// This allows you to set the color of shapes or apply a texture to some
    /// vertices.
    ///
    /// The code for a basic fragment shader which applies the color stored in
    /// the `VertexBuffer` can be found in example 3, at
    /// <https://github.com/dylanopen/realms/tree/main/examples/example3_colorful_triangle/src/fragment.glsl>
    ///
    /// Read more: <https://www.khronos.org/opengl/wiki/Fragment_Shader>
    Fragment = gl::FRAGMENT_SHADER,
}

/// A wrapper around an opengl shader.
/// This struct currently only stores the opengl reference to the shader, but
/// is provided as a convenient wrapper for creating shaders from their source.
/// 
/// NOTE: `Shader`s do nothing unless added to a `ShaderProgram`.
///
/// NOTE: When the shader is added to a `ShaderProgram`, the shader is deleted.
/// This isn't necessary, but it helps free up a little memory as the shader
/// itself is useless without an attached program.
/// If you want to use the same shader in a different program, you MUST create
/// fully a new `Shader`. *Do not* just copy the `gl_id` into a new `Shader`,
/// as the shader is DELETED. This will either throw an error or just not
/// render anything at all.
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
#[non_exhaustive]
pub struct Shader {

    /// The opengl reference id to the shader (as a standard C integer).
    /// This is used to link the vertex, fragment, etc. shaders together using
    /// an opengl function by a `ShaderProgram` class.
    ///
    /// You usually don't need to access this. However, it is made public so
    /// that libraries extending the low-level functionality of Realms can
    /// interoperate with the Realms `Shader` class.
    pub gl_id: u32,
}

impl Shader {
    /// Load and compile an opengl shader from the given source string and
    /// shader type.
    ///
    /// ## Errors
    ///
    /// If the shader compilation failed (i.e. your shader has a syntax error),
    /// the error information is provided in the `Err` variant as a string.
    /// If you `.unwrap()` or `.expect(...)` the return value, it will print
    /// the GLSL error.
    ///
    /// ## Panics
    ///
    /// In the rare but possible case that opengl returns a malformed response
    /// upon request for the compile error of the shader, this function will
    /// PANIC.
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
    #[inline]
    #[expect(clippy::needless_pass_by_value, reason = "in next major release, instead take in a reference")]
    #[expect(clippy::unwrap_in_result, reason = "this crash is likely extremely rare if not impossible")]
    #[expect(clippy::uninit_vec, reason = "I can't find a way to fix this lint, PR/issue if you know the solution")]
    pub fn load_str(shader_type: ShaderType, source: String) -> Result<Shader, String> {
        #[expect(clippy::as_conversions, reason = "no other way to get integral value of enum variant")]
        let gl_id = unsafe {gl::CreateShader(shader_type as u32)};
        #[expect(clippy::question_mark_used, reason = "? here simplifies code and does the same thing as a match would")]
        let c_source = CString::new(source.as_bytes())
            .map_err(|err| format!("Realms: failed to create CString from shader source: {err}"))?;
        unsafe {gl::ShaderSource(gl_id, 1, &c_source.as_ptr(), ptr::null())};
        unsafe {gl::CompileShader(gl_id)};

        let mut success = GLint::from(gl::FALSE);
        let mut info_log = Vec::with_capacity(1024);
        unsafe {info_log.set_len(1024 - 1)}; // -1 to skip trailing \0
        unsafe {gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &raw mut success)};
        if success != GLint::from(gl::TRUE) {
            #[expect(clippy::as_conversions, reason = "seemingly no other way to cast to a GLchar")]
            #[expect(clippy::ptr_as_ptr, reason = "clippy's suggestion doesn't seem to work")]
            unsafe {gl::GetShaderInfoLog(
                gl_id, 1024, ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar
            );};
            let gl_error = String::from_utf8_lossy(&info_log);
            #[expect(clippy::expect_used, reason = "this crash is likely extremely rare if not impossible")]
            #[expect(clippy::shadow_reuse, reason = "we need to create a temp variable then shadow it, as otherwise the ref doesn't last long enough")]
            let gl_error = gl_error.split_once('\0')
                .expect("Realms: received malformed shader compile error info from opengl").0;

            return Err(format!("Realms: failed to compile shader: {gl_error}"));
        }

        Ok(Shader { gl_id })

    }
}

/// A wrapper around an opengl shader *program*.
///
/// This struct currently only stores the opengl reference to the shader
/// program.
/// It is provided as a convenient wrapper for creating a shader program from
/// a vector of `Shader` objects.
#[non_exhaustive]
pub struct ShaderProgram {

    /// The opengl reference id to the shader program (as a standard C integer).
    /// This is used to `use` the shader program on each frame with the
    /// `gl::UseProgram` function called by the `new_frame` function.
    ///
    /// You usually don't need to access this. However, it is made public so
    /// that libraries extending the low-level functionality of Realms can
    /// interoperate with the Realms `Shader` class.
    pub gl_id: u32,
}

impl ShaderProgram {

    /// A default `ShaderProgram` for when you want to *unbind* the current
    /// shader.
    /// Its uses are to unbind a shader (if you don't want to use any shader)
    /// or if you haven't yet written a shader and want to use realms without
    /// one.
    ///
    /// For an example of using the `NONE` shader, see *example 1: window*:
    /// <https://github.com/dylanopen/realms/tree/main/examples/example1_window>
    pub const NONE: ShaderProgram = ShaderProgram { gl_id: 0 }; 

    /// Load and compile an opengl shader **program** from the given `Vec` of
    /// `Shader`s;
    ///
    /// NOTE: When the shader is added to a `ShaderProgram`, the shader is deleted.
    /// This isn't necessary, but it helps free up a little memory as the shader
    /// itself is useless without an attached program.
    /// If you want to use the same shader in a different program, you MUST create
    /// fully a new `Shader`. *Do not* just copy the `gl_id` into a new `Shader`,
    /// as the shader is DELETED. This will either throw an error or just not
    /// render anything at all.
    ///
    /// ## Errors
    ///
    /// If linking the shader program fails, the error will be returned in the
    /// `Err` variant of the result.
    /// If you `.unwrap()` or `.expect(...)` the return value, it will print
    /// the GLSL error.
    ///
    /// ## Panics
    ///
    /// In the rare but possible case that opengl returns a malformed response
    /// upon request for the compile error of the shader, this function will
    /// PANIC.
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
    #[expect(clippy::needless_pass_by_value, reason = "the caller shouldn't be able to mutate the shaders after building the shader program")]
    #[inline]
    pub fn new(shaders: Vec<Shader>) -> Result<ShaderProgram, String> {

        #[expect(clippy::multiple_unsafe_ops_per_block, reason = "will fix when I have time; feel free to open a PR")]
        #[expect(clippy::uninit_vec, reason = "we should add MaybeUninit wrapper to info_log in the future")]
        let gl_id = unsafe {
            let gl_id = gl::CreateProgram();
            for shader in &shaders {
                gl::AttachShader(gl_id, shader.gl_id);
            }
            gl::LinkProgram(gl_id);

            let mut success = GLint::from(gl::FALSE);
            let mut info_log = Vec::with_capacity(1024);
            info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramiv(gl_id, gl::LINK_STATUS, &raw mut success);
            if success != GLint::from(gl::TRUE) {
                #[expect(clippy::as_conversions, reason = "need to cast mut ptr -> *mut GLchar")]
                #[expect(clippy::ptr_as_ptr, reason = "need to cast mut ptr -> *mut GLchar")]
                gl::GetProgramInfoLog(
                    gl_id, 1024, ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar
                );
                #[expect(clippy::absolute_paths, reason = "importing the `str` module would clash with the `str` type")]
                return Err(format!("Realms: failed to link shader program: {}",
                        core::str::from_utf8(&info_log).map_err(|e| e.to_string())?
                ));
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
    #[inline]
    pub fn new_frame(&self) {
        unsafe { gl::UseProgram(self.gl_id) };
    }

    /// Upload a single float to the shader as a uniform.
    /// Use this uniform in the shader using `uniform float <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_1f(&self, uniform_name: &str, data: f32) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform1f(location, data)};
    }

    /// Upload a vec2 of floats to the shader as a uniform.
    /// Use this uniform in the shader using `uniform vec2 <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_2f(&self, uniform_name: &str, data: (f32, f32)) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform2f(location, data.0, data.1)};
    }

    /// Upload a vec3 of floats to the shader as a uniform.
    /// Use this uniform in the shader using `uniform vec3 <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_3f(&self, uniform_name: &str, data: (f32, f32, f32)) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform3f(location, data.0, data.1, data.2)};
    }

    /// Upload a vec4 of floats to the shader as a uniform.
    /// Use this uniform in the shader using `uniform vec4 <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_4f(&self, uniform_name: &str, data: (f32, f32, f32, f32)) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform4f(location, data.0, data.1, data.2, data.3)};
    }

    /// Upload a single integer (i32) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform int <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_1i(&self, uniform_name: &str, data: i32) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform1i(location, data)};
    }

    /// Upload a vec2 of integers (i32s) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform ivec2 <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_2i(&self, uniform_name: &str, data: (i32, i32)) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform2i(location, data.0, data.1)};
    }

    /// Upload a vec3 of integers (i32s) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform ivec3 <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_3i(&self, uniform_name: &str, data: (i32, i32, i32)) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform3i(location, data.0, data.1, data.2)};
    }

    /// Upload a vec4 of integers (i32s) to the shader as a uniform.
    /// Use this uniform in the shader using `uniform ivec4 <name>`.
    ///
    /// ## Panics
    ///
    /// If the `uniform_name` passed could not be converted into a `CString`,
    /// this function will PANIC as it unwraps a `Result`.
    #[inline]
    pub fn uniform_4i(&self, uniform_name: &str, data: (i32, i32, i32, i32)) {
        #[expect(clippy::unwrap_used, reason = "it is very rare that the library user will pass a `&str` that cannot be converted to a `CString`")]
        let location = unsafe { gl::GetUniformLocation(self.gl_id, CString::new(uniform_name).unwrap().as_ptr()) };
        unsafe {gl::Uniform4i(location, data.0, data.1, data.2, data.3)};
    }
}

