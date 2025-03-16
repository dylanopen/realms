// This example assumes you have also read example 2: triangle.
// Many things are not explained here as they were explained in the last
// example.

use realms::vertex::VertexBuffer;
use realms::window::Window;
use realms::data::Color;
use realms::shader::{Shader, ShaderProgram, ShaderType};

fn main() {
    let mut window = Window::new(800, 600, "Hello Triangle!")
        .expect("Failed to create window");

    let shader_program = ShaderProgram::new(vec![
        Shader::load_str(ShaderType::Vertex, include_str!("vertex.glsl").to_string()).unwrap(),
        Shader::load_str(ShaderType::Fragment, include_str!("fragment.glsl").to_string()).unwrap(),
    ]).unwrap();
    // NOTE: the shaders have changed slightly since example 2. Please update
    // `vertex.glsl` and `fragment.glsl` using the new versions of them in
    // this directory.

    let vertices: [f32; 15] = [ // specify type `f32` with 15 elements: 3 vertices * 5 floats each.
    //   X     Y     red green blue
         0.0,  0.5,  0.0, 1.0, 0.0,  // top of triangle, green
        -0.5, -0.5,  1.0, 0.0, 0.0,  // bottom left of triangle, red
         0.5, -0.5,  0.0, 0.0, 1.0,  // bottom right of triangle, blue
    ];

    // See https://learnopengl.com/Getting-started/Hello-Triangle for more info.
    // Scroll to the section on Element Buffer Objects.
    let elements: [u32; 3] = [
        0, 1, 2,
    ];

    let vb = VertexBuffer::new(&vertices, &elements);

    vb.add_attrib( // create an attribute for the POSITION of the vertex.
        0, // first attrib so layout = 0
        2, // the position component is made up of 2 floats here: x and y
        5, // stride is 5 as each vertex is made up of 5 floats: 2 for position + 3 for color
        0, // offset is 0 as this is the first attrib: no previous attribs.
    );

    vb.add_attrib( // create an attribute for the COLOR of the vertex.
        1, // second attrib so layout = 1
        3, // the color component is made up of 3 floats here: r, g and b
        5, // stride is 5 as each vertex is made up of 5 floats: 2 for position + 3 for color
        2, // offset is 2 as the previous attrib(s) used 2 floats
    );

    while window.is_running() {
        window.new_frame(&shader_program);

        window.fill(Color::rgb(20, 34, 40));
        window.events(); // we don't handle any events, but we need to poll for them anyway.
        
        vb.draw(); // draw the data in our vertex buffer
    }
}
