# Welcome to Realms - the lightweight Rust game library

Realms is a library that allows you to easily make games, animations and any 2D
or 3D graphics using Rust.

As of version `1.1.1`, Realms now uses OpenGL as a backend. This is a huge API
change, and most things have changed since `0.2.1`. Get up to date by
[reading the examples](https://github.com/dylanopen/realms/tree/main/examples).

## Documentation

For extensive documentation on specific fields, structs and functions, see the
documentation on [docs.rs](https://docs.rs/realms).

## Examples

You can find a list of examples for how to use Realms at
[github.com/dylanopen/realms/tree/main/examples](https://github.com/dylanopen/realms/tree/main/examples)

Here's the code needed to create a window:

``` rust
use realms::window::Window; // don't accidentally import realms::glfw::Window!
use realms::data::Color;
use realms::input::{Event, Key}; // again, don't import from realms::glfw
use realms::shader::ShaderProgram;

fn main() {
    // create the window and unwrap the result:
    let mut window = Window::new(800, 600, "Hello Window!")
        .expect("Failed to create window");

    // loop until the user closes the window or the 'Q' key is pressed
    while window.is_running() {
        window.new_frame(); // swap the buffers (draw to the screen)
        window.fill(Color::rgb(20, 34, 40)); // fill the screen dark blue

        for event in window.events() { // loop over all window events
            match event {
                Event::KeyDown(Key::Q) => window.close(), // if Q pressed, exit loop
                _ => {}, // otherwise do nothing
            }
        }
    }
}
```

## Useful resources

Realms is based on OpenGL. For this reason, it is very helpful to have at
least some knowledge of the basics of OpenGL; in particular with vertex
buffers (VAOs, VBOs and EBOs/IBOs) and shaders in the GLSL programming language.

Below is a list of useful resources for learning OpenGL, the Realms library
APIs and game development in general:

- [Realms examples](https://github.com/dylanopen/realms/tree/main/examples)
- [Realms documentation (docs.rs)](https://docs.rs/realms)
- [LearnOpenGL](https://learnopengl.com)
- [The Book of Shaders](https://thebookofshaders.com/)

Thanks for choosing Realms to build your next great game!
