//! The `shape` module contains structs and functions for drawing simple 2D
//! shapes to the screen.
//! Each shape instance will create a new `VertexBuffer` which need to
//! individually be sent to the GPU. For that reason, this module should only
//! be used for prototyping and applications where performance isn't important.

use crate::data::Color;
use crate::shader::{Shader, ShaderProgram, ShaderType};
use crate::vertex::VertexBuffer;

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

/// The `TriangleShape` struct represents any 3 points in the 2d plane. Each
/// point (vertex) is made up of:
/// - 2 position components (x, y)
/// - 3 color components (r, g, b)
///
/// There are many different `new` methods for a `TriangleShape`, each providing
/// an easy way to create a certain type of triangle.
/// For examples, please see the different functions that `TriangleShape`
/// implements.
#[non_exhaustive]
pub struct TriangleShape {
    /// Stores the `VertexBuffer` that represents the triangle.
    /// See the documentation for `VertexBuffer` for more info.
    /// The `TriangleShape::draw` method will call `vertex_buffer.draw`.
    vertex_buffer: VertexBuffer,
}

impl TriangleShape {
    /// Create a new `TriangleShape`from a list of 15 `f32`s. The list is a set
    /// of 3 vertices, each containing two components:
    /// - a *position* made up of `2` `f32`s (x, y)
    /// - a *color* made up of `3` `f32`s (r, g, b)
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let shader = shape2d_shader();
    /// let triangle = TriangleShape::new(&[
    ///     -0.5, -0.5, 1.0, 0.0, 0.0,
    ///      0.5, -0.5, 0.0, 1.0, 0.0,
    ///     -0.5,  0.5, 0.0, 0.0, 1.0,
    /// ]);
    /// while w.is_running() {
    ///     ...
    ///     triangle.draw(&shader);
    /// }
    /// ```
    #[inline]
    pub fn new(vertices: &[f32; 15]) -> TriangleShape {
        let vertex_buffer = VertexBuffer::new(vertices, &[0, 1, 2]);
        vertex_buffer.set_layout(&[2_i32, 3_i32]);
        TriangleShape { vertex_buffer }
    }

    /// Create a new `TriangleShape` from a list of 6 `f32`s and a color for the
    /// entire triangle. The list is a set of 3 vertices, each containing one
    /// component:
    /// - a *position* made up of `2` `f32`s (x, y)
    ///
    /// As the name implies, this function will create a new triangle with a
    /// single, solid color for the entire triangle. If you need to interpolate
    /// (blend) between different colors and have different colors for each
    /// vertex, use the `TriangleShape::new` function.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let shader = shape2d_shader();
    /// let triangle = TriangleShape::new_solid(&[
    ///     -0.5, -0.5,
    ///      0.5, -0.5,
    ///     -0.5,  0.5,
    /// ], &Color::rgb(255, 127, 31));
    /// while w.is_running() {
    ///     ...
    ///     triangle.draw(&shader);
    /// }
    /// ```
    #[inline]
    pub fn new_solid(vertices: &[f32; 6], color: &Color) -> TriangleShape {
        let (r, g, b, _) = color.gl();
        TriangleShape::new(&[
            vertices[0], vertices[1], r, g, b,
            vertices[2], vertices[3], r, g, b,
            vertices[4], vertices[5], r, g, b,
        ])
    }

    /// Create a new `TriangleShape` as an isosceles triangle with a flat base.
    /// Isosceles triangles have two sides the same.
    /// The coordinates for the isosceles triangle are calculated like this:
    /// 
    /// 1. (`x`, `y`)
    /// 2. (`x+base`, `y`)
    /// 3. (`x+base*0.5`, `y+height`)
    ///
    /// ## Parameters
    ///
    /// - `x: f32` - the mininum (furthest left) X coordinate on the triangle
    /// - `y: f32` - the minimum (furhest down) Y coordinate on the triangle
    /// - `width: f32` - how far the base of the triangle extends (the
    ///   difference in X position between the furthest right point and the
    ///   furthest left point
    /// - `height: f32` - how far upwards the triangle extends (the difference
    ///   in Y position between the highest point and the lowest point)
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let shader = shape2d_shader();
    /// let triangle = TriangleShape::new_flat_isosceles(
    ///     -0.5, -0.5, 1.0, 1.0,
    ///     Color::new(63, 191, 91)
    /// );
    /// while w.is_running() {
    ///     ...
    ///     triangle.draw(&shader);
    /// }
    /// ```
    #[inline]
    pub fn new_flat_isosceles(x: f32, y: f32, width: f32, height: f32, color: &Color) -> TriangleShape {
        TriangleShape::new_solid(&[
            x, y,
            x + width, y,
            width.mul_add(0.5, x), y + height,
        ], color)
    }

    /// Draw the triangle to the screen.
    /// You **must** pass in a reference to the shader program returned by the
    /// `shape2d_shader()` function, or a compatible shader program, or else
    /// this method will fail silently.
    ///
    /// This method currently simply calls the `draw` method of the
    /// `vertex_buffer` field.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let shader = shape2d_shader();
    /// let triangle = TriangleShape::new_solid(&[
    ///     -0.5, -0.5,
    ///      0.5, -0.5,
    ///     -0.5,  0.5,
    /// ], &Color::rgb(255, 127, 31));
    /// while w.is_running() {
    ///     ...
    ///     triangle.draw(&shader);
    /// }
    /// ```
    #[inline]
    pub fn draw(&self, shader_program: &ShaderProgram) {
        self.vertex_buffer.draw(shader_program);
    }
}

