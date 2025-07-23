// This example assumes you have also read example 3: colorful triangle.
// Many things are not explained here as they were explained in the last
// example. This example builds upon example 3.
// <https://github.com/dylanopen/realms/tree/main/examples/example3_colorful_triangle>

use realms::input::{Event, Key};
use realms::vertex::VertexBuffer;
use realms::window::Window;
use realms::data::Color;
use realms::shader::{Shader, ShaderProgram, ShaderType};

fn main() {
    let mut window = Window::new(800, 600, "Hello Triangle!")
        .expect("Failed to create window");

    let shader_program = ShaderProgram::new(vec![
        Shader::load_str(ShaderType::Vertex, include_str!("vertex.glsl")).unwrap(),
        Shader::load_str(ShaderType::Fragment, include_str!("fragment.glsl")).unwrap(),
    ]).unwrap();
    // NOTE: the vertex shader has  changed since example 3. Please update
    // `vertex.glsl` using the new versions of it in this directory.
    // The fragment shader has stayed the same.

    // all vertices are the same as in the last example
    let vertices: [f32; 15] = [
    //   X     Y     red green blue
         0.0,  0.5,  0.0, 1.0, 0.0,  // top of triangle, green
        -0.5, -0.5,  1.0, 0.0, 0.0,  // bottom left of triangle, red
         0.5, -0.5,  0.0, 0.0, 1.0,  // bottom right of triangle, blue
    ];

    let elements: [u32; 3] = [
        0, 1, 2,
    ];

    let vb = VertexBuffer::new(&vertices, &elements);

    // same attribute layout as in the last example:
    vb.set_layout(&[2, 3]);

    let (mut camera_x, mut camera_y) = (0.0, 0.0); // --NEW-- //

    while window.is_running() {
        window.new_frame(&shader_program);
        window.fill(Color::rgb(20, 34, 40));

// --- NEW --- //
        for event in window.events() {
            match event {
                // here, we don't store the current state of the key, so we
                // need to move the camera by repeatedly pressing WASD.
                Event::KeyDown(Key::W) => camera_y += 0.1, // move camera up
                Event::KeyDown(Key::S) => camera_y -= 0.1, // move camera down
                Event::KeyDown(Key::A) => camera_x -= 0.1, // move camera left
                Event::KeyDown(Key::D) => camera_x += 0.1, // move camera right
                _ => {}
            }
        }
        // upload the camera position to the vertex shader using a *uniform*:
        // learn more: https://thebookofshaders.com/03/
        shader_program.uniform_2f("cameraPos", (camera_x, camera_y));
// --- END NEW --- //
        
        vb.draw(); // draw the data in our vertex buffer
    }
}

// NOTE: as we are moving the camera, the triangle will be moved opposite to
// what we would expect (e.g. pressing 'left' moves the triangle right).
// This is because we are moving the camera, which direction opposes that of
// the shapes it views.

