//! This example will draw an image inside a square on the screen.

// This example assumes you have also read example 3: triangle.
// Many things are not explained here as they were explained in that example.

use realms::vertex::VertexBuffer;
use realms::window::Window;
use realms::data::Color;
use realms::shader::{Shader, ShaderProgram, ShaderType};
use realms::texture::Texture;

fn main() {
    let mut window = Window::new(600, 600, "Rendering Pictures?!")
        .expect("Failed to create window");

    let shader_program = ShaderProgram::new(vec![
        Shader::load_str(ShaderType::Vertex, include_str!("shaders/vertex5.glsl")).unwrap(),
        Shader::load_str(ShaderType::Fragment, include_str!("shaders/fragment5.glsl")).unwrap(),
    ]).unwrap();

    let vertices: [f32; 16] = [ // specify type `f32` with 16 elements: 4 vertices * 4 floats each.
//        X     Y    texX texY
        -0.5,  0.5,  0.0, 1.0,  // top left of triangle, top left of texture
        -0.5, -0.5,  0.0, 0.0,  // bottom left of triangle, bottom left of texture
         0.5, -0.5,  1.0, 0.0, // bottom right of triangle, bottom right of texture
         0.5,  0.5,  1.0, 1.0,  // top right of triangle, top right of texture
    ];

    let elements: [u32; 6] = [
        0, 1, 2,
        2, 3, 0,
    ];

    let vb = VertexBuffer::new(&vertices, &elements);

    vb.set_layout(&[
        2, // each vertex has a position, made up of TWO float components (x, y)
        2, // each vertex has a texture position, made up of TWO float components (x, y)
    ]);

    let texture = Texture::load_file("examples/images/parrot.png").unwrap();
    texture.bind();

    while window.is_running() {
        window.new_frame();
        window.events();

        window.fill(Color::rgb(39, 85, 163));
        vb.draw(&shader_program);
    }
}
