//! The `vertex` module stores structs and methods to interact with
//! `VertexBuffer`s: a way of interacting with opengl's VAOs, VBOs and
//! EBOs but in a simpler and safer way.
//!
//! This module does *not* contain struct for the individual vertex data,
//! such as storing positions and colors. It is left up to the user of
//! the library to specify an array of vertex data in whichever form
//! works best for your program: Realms does not thrust a certain method
//! of vertex data storage upon users.

use core::ffi::c_void;
use core::convert;
use core::mem;
use core::ptr;

use gl::types::{GLfloat, GLsizei};

use crate::shader::ShaderProgram;

/// A `VertexBuffer` is a wrapper around an opengl VAO, VBO and EBO.
/// It is essentially a list of vertices (positions, colors, textures, etc.)
/// and elements (which tell opengl which order to draw/join the vertices in).
///
/// You still need to create a `[f32]` slice of vertex data, which can store
/// any information you would like to be passed to the shader, and a `[u32]`
/// slice telling opengl the order to draw and connect the vertices in.
///
/// It shortens the 10+ opengl calls to create the buffers into a single call
/// of `new`, as well as many `add_component` calls as you have components.
///
/// In short, this struct allows you to manage the shapes that are drawn to
/// the screen.
pub struct VertexBuffer {

    /// The opengl pointer to the Vertex Array Object.  
    /// The VAO allows for quickly switching between different VBOs.
    /// > See <https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Array_Object>
    vao_id: u32,

    /// The opengl pointer to the Vertex Buffer Object.  
    /// The VBO stores the vertex data (information about the triangles we want
    /// to draw to the screen, including their position, color and whatever
    /// other data we pass it).
    /// > See <https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object>
    vbo_id: u32,

    /// The opengl pointer to the Vertex Buffer Object.  
    /// The VBO stores the vertex data (information about the triangles we want
    /// to draw to the screen, including their position, color and whatever
    /// other data we pass it).
    /// > See <https://www.opengl-tutorial.org/intermediate-tutorials/tutorial-9-vbo-indexing>
    ebo_id: u32,

    /// Stores the number of elements in the VBO. This is equal to the length of
    /// the `elements` slice passed to the `new` method, or the length of the
    /// `vertices` slice divided by the stride side (number of components per
    /// vertex).
    element_count: i32,
}

impl VertexBuffer {
    /// Create a `realms::VertexBuffer` with the specified `vertices` and
    /// `elements`.
    /// Once created, you need to call the `add_component` method for each
    /// component of the vertex array to tell the shader where to find each
    /// section of the vertex data (i.e. which parts are positions, colors,
    /// texture coords, etc.).
    ///
    /// If you don't know what opengl vertex buffers are, they are essentially
    /// arrays storing information about the triangles to draw to the screen.
    /// > Please read <https://learnopengl.com/Getting-started/Hello-Triangle>
    /// > (specifically the start of the "Vertex input" section) for more info
    /// > on opengl indices and buffers.
    ///
    /// ## Example usage:
    ///
    /// #### In your rust source file:
    ///
    /// ``` rust
    /// let vertices: [f32; 24] = [
    /// //  POSITION:      COLOR:
    ///     0.5,  -0.5,    1.0, 0.0, 0.0, 1.0,  // bottom right
    ///     -0.5, -0.5,    0.0, 1.0, 0.0, 1.0,  // bottom left
    ///     -0.5, 0.5,     0.0, 0.0, 1.0, 1.0,  // top left
    ///     0.5,  0.5,     1.0, 1.0, 1.0, 1.0,  // top right
    /// ];
    /// let elements: [u32; 6] = [
    ///    0, 1, 2,
    ///    0, 3, 2,
    /// ];
    /// let vb: VertexBuffer = VertexBuffer::new(&vertices, &elements);
    /// vb.add_attrib(0, 2, 6, 0);
    /// vb.add_attrib(1, 4, 6, 2);
    /// ```
    ///
    /// #### Then, remember to specify the layouts in the vertex shader:
    ///
    /// ```glsl
    /// layout (location = 0) in vec2 aPos;  
    /// layout (location = 1) in vec4 aColor;
    /// ```
    ///
    /// ## Panics
    ///
    /// Although rare, it is technically possible for this function to PANIC if
    /// the size of the vertices passed in was too large to be converted from a
    /// `u32` to an `i32`. You probably don't need to worry about this, unless
    /// you have over 2.1 billion vertex components (in which case you've got
    /// bigger problems to deal with, such as your GPU being on fire).
    #[expect(clippy::similar_names, reason = "yes clippy, the vao and vbo *should* have a similar name...")]
    #[inline]
    pub fn new(vertices: &[f32], elements: &[u32]) -> VertexBuffer {
        let (mut vbo_id, mut vao_id, mut ebo_id) = (0, 0, 0);
            unsafe { gl::GenVertexArrays(1, &raw mut vao_id) };
            unsafe { gl::GenBuffers(1, &raw mut vbo_id) };
            unsafe { gl::GenBuffers(1, &raw mut ebo_id) };

            unsafe { gl::BindVertexArray(vao_id) };
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id) };
            #[expect(clippy::expect_used, clippy::indexing_slicing)]
            unsafe { gl::BufferData(gl::ARRAY_BUFFER,
                mem::size_of_val(vertices).try_into()
                    .expect("way too much vertex data, overflowed when casting u32 to i32"),
                (&raw const vertices[0]).cast::<c_void>(),
                gl::STATIC_DRAW); };
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id) };
            #[expect(clippy::expect_used, clippy::indexing_slicing)]
            unsafe { gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                mem::size_of_val(elements).try_into()
                    .expect("way too many elements, overflowed when casting u32 to i32"),
                (&raw const elements[0]).cast::<c_void>(),
                gl::STATIC_DRAW); };
        #[expect(clippy::expect_used)]
        VertexBuffer {
            vao_id, vbo_id, ebo_id,
            element_count: elements.len().try_into()
                .expect("way too many elements, overflowed when casting u32 to i32"),
        }
    }

    /// As the `vertices` array reference passed to the `new` function was just
    /// a slice of floats, we need to tell opengl how to split it up.
    /// For each 'attribute' of the vertex, call this function. For example, if
    /// you have a `vertices` array with 4 vertices, and each vertex was made
    /// up of a position (using two floats) and a color (using 4 floats), you
    /// would call the function twice (for each attribute):
    ///
    /// ``` rust
    /// let vb = VertexBuffer::new(...);
    /// vb.add_attrib( // position attribute
    ///     0, // first attrib, so layout index is 0
    ///     2, // the position is made up of 2 floats, so 2 position components
    ///     6, // each vertex has 6 floats (2 pos + 4 color) so stride is 6
    ///     0, // first attrib, so no offset
    /// )
    /// vb.add_attrib( // color attribute
    ///     0, // second attrib, so layout index is 1
    ///     4, // the color is made up of 4 floats, so 4 position components
    ///     6, // each vertex has 6 floats (2 pos + 4 color) so stride is 6
    ///     2, // second attrib. first had 2 components, so offset is 2
    /// )
    /// ```
    ///
    /// ## Panics
    ///
    /// It's likely impossible to occur, but if the value returned by
    /// `mem::size_of` for the size of a `GLfloat` cannot be converted into a
    /// `GLsizei`, the program will PANIC with an `expect` error.
    #[inline]
    pub fn add_attrib(&self, layout: u32, component_count: i32, stride: i32, offset: usize) {
        #[expect(clippy::as_conversions, clippy::arithmetic_side_effects)]
        let offset_ptr = (offset * mem::size_of::<GLfloat>()) as *const c_void;
        #[expect(clippy::expect_used, clippy::arithmetic_side_effects)]
        unsafe { gl::VertexAttribPointer(
            layout, component_count, gl::FLOAT, gl::FALSE,
            stride * convert::TryInto::<GLsizei>::try_into(mem::size_of::<GLfloat>())
                .expect("Realms: Failed to convert size of GLfloat to GLsizei"),
            offset_ptr
        ); };
        unsafe { gl::EnableVertexAttribArray(layout) };
    }

    /// Adding the vertex attributes through the `add_attrib` method requires a
    /// lot of boilerplate and leads to messy code. Realms can infer all of that
    /// information just from a slice of component counts.
    ///
    /// A component count is just a number reflecting the number of floats a
    /// component is made up of. A component is some information about the
    /// vertex that your vertex shader takes in as a `layout` parameter.
    ///
    /// For example, if each vertex has a `position` and a `color`, your
    /// component counts may be:
    /// - `3` components for position (X, Y and Z)
    /// - `4` components for color (R, G, B and A)
    /// - Therefore your `component_counts` slice would be `[3, 4]`.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let vb = VertexBuffer::new(...);
    /// vb.set_layout(&[
    ///     3, // if our vertex position is made up of 3 floats (x, y, z)
    ///     4, // if our vertex color is made up of 4 floats (r, g, b, a)
    /// ]);
    /// ```
    ///
    /// ## Panics
    ///
    /// If the layout could not be converted from a `usize` into a `u32` (this
    /// could only happen if there are over 4.2 billion components) the program
    /// will PANIC. This will likely never happen.
    ///
    /// If the sum of the components are larger than the max usize value, the
    /// program will PANIC. But you will never need that many components.
    /// Also, on 8-bit or 16-bit systems, the max value for a `usize` is much
    /// lower, so casting each `component` `i32` to a `usize` may fail and cause
    /// a PANIC.
    ///
    /// It's likely impossible to occur, but if the value returned by
    /// `mem::size_of` for the size of a `GLfloat` cannot be converted into a
    /// `GLsizei`, the program will PANIC with an `expect` error. This panic is
    /// raised by the `add_attrib` method.
    #[inline]
    pub fn set_layout(&self, component_counts: &[i32]) {
        let stride: i32 = component_counts.iter().sum();
        let mut offset = 0;
        #[expect(clippy::arithmetic_side_effects, clippy::unwrap_used)]
        for (layout, component_count) in component_counts.iter().enumerate() {
            self.add_attrib(u32::try_from(layout).unwrap(), *component_count, stride, offset);
            offset += usize::try_from(*component_count).unwrap();
        }
    }

    /// Draw the vertex buffer as a series of triangles.
    /// Note that if you change the elements of the array of vertices or
    /// elements, they will not be updated in the `VertexBuffer`. If changing
    /// the elements, you should create a new `VertexBuffer` and call `draw` on
    /// that instead.
    /// 
    /// WARNING: This binds the VAO, VBO and EBO. It *does not* unbind them
    /// afterwards.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     vertex_buffer.draw(&shader_program);
    /// }
    /// ```
    /// ## Migrating from 2.3.4 to 3.3.4
    ///
    /// The `Window::new_frame` method no longer takes in a shader program
    /// reference, but the `VertexBuffer::draw` method now does.
    /// You should bind the shader program when calling this draw method, NOT
    /// when calling `Window::new_frame`.
    ///
    /// In short, instead of doing this (pre-3.3.4):
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame(&shader_program);
    ///     vertex_buffer.draw();
    /// }
    /// ```
    ///
    /// You should do this (3.3.4+):
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     vertex_buffer.draw(&shader_program);
    /// }
    /// ```
    #[inline]
    pub fn draw(&self, shader_program: &ShaderProgram) {
        shader_program.bind();
        unsafe { gl::BindVertexArray(self.vao_id) };
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id) };
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id) };
        unsafe { gl::EnableVertexAttribArray(self.vao_id) };
        unsafe { gl::DrawElements(gl::TRIANGLES, self.element_count, gl::UNSIGNED_INT, ptr::null()) };
    }
}

