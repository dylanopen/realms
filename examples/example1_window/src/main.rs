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
