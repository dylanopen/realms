//! The `shape` module contains structs and functions for drawing simple 2D
//! shapes to the screen.
//! Each shape instance will create a new `VertexBuffer` which need to
//! individually be sent to the GPU. For that reason, this module should only
//! be used for prototyping and applications where performance isn't important.

use crate::shader::{Shader, ShaderProgram, ShaderType};

/// This module contains structs for drawing simple shapes to the screen. For
/// this reason, it's useful to have a set of default shaders that are
/// guaranteed to work out-of-the-box when drawing simple 2D shapes.
///
/// This function simply compiles and returns a `ShaderProgram` which will
/// be compatible with all shapes in this module. You are advised to call and
/// store the result of this function at the beginning of your main function,
/// so you don't ever have to recompile this shader.
///
/// ## Example usage
///
/// ``` rust
/// let shape2d_program = shape2d_shader();
/// let triangle = Triangle::new(...);
/// while w.is_running() {
///     ...
///     triangle.draw(&shape2d_program);
/// }
/// ```
///
/// ## Panics
///
/// As the shader source is embedded directly into Realms with the
/// `include_str!` macro, it's unlikely that linking this shader program will
/// fail.
/// However, it's possible that either the vertex shader, fragment shader or
/// shader program will fail to link or compile, and then this function will
/// PANIC.
///
/// Please report any panics to <https://github.com/dylanopen/realms/issues>
#[expect(clippy::unwrap_used)]
#[inline]
pub fn shape2d_shader() -> ShaderProgram {
    ShaderProgram::new(vec![
        Shader::load_str(ShaderType::Vertex, include_str!("shader/shape2d.vert.glsl")).unwrap(),
        Shader::load_str(ShaderType::Fragment, include_str!("shader/shape2d.frag.glsl")).unwrap(),
    ]).unwrap()
}
