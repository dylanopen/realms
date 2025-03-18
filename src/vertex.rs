use std::ffi::c_void;
use std::ptr;

use gl::types::{GLfloat, GLsizei, GLuint};

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
    vao_id: u32,
    vbo_id: u32,
    ebo_id: u32,
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
    /// > Please read https://learnopengl.com/Getting-started/Hello-Triangle
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
    pub fn new(vertices: &[f32], elements: &[u32]) -> VertexBuffer {
        let (mut vbo_id, mut vao_id, mut ebo_id) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao_id);
            gl::GenBuffers(1, &mut vbo_id);
            gl::GenBuffers(1, &mut ebo_id);

            gl::BindVertexArray(vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER,
                std::mem::size_of_val(vertices).try_into().unwrap(),
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(elements).try_into().unwrap(),
                &elements[0] as *const GLuint /*or u32*/ as *const c_void,
                gl::STATIC_DRAW);
        };
        VertexBuffer {
            vao_id, vbo_id, ebo_id,
            element_count: elements.len() as i32,
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
    pub fn add_attrib(&self, layout: u32, component_count: i32, stride: i32, offset: usize) {
        unsafe {
            let offset_ptr = (offset * std::mem::size_of::<GLfloat>()) as *const c_void;
            gl::VertexAttribPointer(
                layout, component_count, gl::FLOAT, gl::FALSE,
                stride * std::mem::size_of::<GLfloat>() as GLsizei,
                offset_ptr
            );
            gl::EnableVertexAttribArray(layout);
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
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo_id);
            gl::EnableVertexAttribArray(self.vao_id); // TODO: try replacing vao_id with 0??
            gl::DrawElements(gl::TRIANGLES, self.element_count, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

