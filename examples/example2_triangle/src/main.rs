use realms::vertex::VertexBuffer;
use realms::window::Window; // don't accidentally import realms::glfw::Window!
use realms::data::Color;
use realms::shader::{Shader, ShaderProgram, ShaderType};

fn main() {
    // create the window and unwrap the result:
    let mut window = Window::new(800, 600, "Hello Triangle!")
        .expect("Failed to create window");

    // create the shader program from the shaders `vertex.glsl` and `fragment.glsl`:
    let shader_program = ShaderProgram::new(vec![
        Shader::load_str(ShaderType::Vertex, include_str!("vertex.glsl").to_string()).unwrap(),
        Shader::load_str(ShaderType::Fragment, include_str!("fragment.glsl").to_string()).unwrap(),
    ]).unwrap();
    // NOTE: you need to code the `vertex.glsl` and `fragment.glsl` files. Some
    // default shaders are provided for you, in the same directory as this file
    // in the files `vertex.glsl` and `fragment.glsl`. You can copy and paste
    // these into the SAME DIRECTORY as your main.rs file.

    // create an [f32] slice of vertex data:
    let vertices: [f32; 6] = [ // specify type `f32` with 6 elements.
    //   X    Y
         0.0,  0.5,  // top of triangle
        -0.5, -0.5,  // bottom left of triangle
         0.5, -0.5,  // bottom right of triangle
    ];

    // Create a [u32] slice listing the indices of the `vertices` array to draw.
    // Although in this case we only have one triangle, in scenes with many
    // triangles this drastically reduces the size of data sent to the GPU.
    // Read more at https://learnopengl.com/Getting-started/Hello-Triangle,
    // scroll to the section on Element Buffer Objects.
    let elements: [u32; 3] = [
        0, 1, 2,
    ];

    // create a VertexBuffer using references to the `vertices` and `elements`:
    let vb = VertexBuffer::new(&vertices, &elements);
    vb.add_attrib( // the attributes tell Realms how to interpret our vertices.
        0, // layout is 0 as it's the first attrib. second would be 1, etc.
        2, // the position component is made up of 2 floats here: x and y
        2, // stride is 2 as each vertex is made up of 2 floats: x and y
        0, // offset is 0 as this is the first attrib: no previous attribs.
    );

    // loop until the user closes the window or the 'Q' key is pressed
    while window.is_running() {
        // swap the buffers (draw to the screen) and bind our shader program:
        window.new_frame(&shader_program);

        window.fill(Color::rgb(20, 34, 40)); // fill the screen dark blue
        window.events(); // we don't handle any events, but we need to poll for them anyway.
        
        vb.draw(); // draw the data in our vertex buffer
    }
}
