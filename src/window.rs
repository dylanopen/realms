//! The `window` module contains structs and functions for interacting with
//! a `glfw::PWindow`.
//!
//! In short, it is used as a wrapper around a GLFW window; it provides
//! many abstractions which make it easier to create windows, bind opengl
//! instances and handle window events.
//!
//! The main struct is `Window`.

use glfw::Context as _;

use super::data::Color;
use super::input::Event;
use super::shader::ShaderProgram;

/// The main struct for creating and interacting with Realms windows.
///
/// The backend for window handling was moved from `minifb` to `glfw` in
/// version 1.0.0!
/// The Window stores the GLFW instance, the `glfw::PWindow` used, glfw events
/// as well as a Vec of the Events for this frame.
/// To get the events for the current frame, use `window.events`.
///
/// Important note: throughout the documentation, instances of `Window` are
/// almost unanimously shortened to `w`. Remember: `w` is the instance of the
/// `Window` struct, created with `Window::new(...)`.
pub struct Window {

    /// The instance of glfw. Used for things such as window hints and event
    /// polling each frame.
    glfw: glfw::Glfw,

    /// The glfw `PWindow` handler. The main interface for managing the glfw
    /// window.
    glfw_window: glfw::PWindow,

    /// The `GlfwReceiver` for getting window events using glfw. You don't ever
    /// need to read this field: to get events, use the public `events` field.
    glfw_events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

}

impl Window {
    /// Create a new instance of the `Window` struct with the specified
    /// `width`, `height` and `title`.
    /// This method creates a glfw `PWindow` with some sane defaults. 
    ///
    /// ## Errors
    ///
    /// On systems without glfw installed, this function may fail. Therefore,
    /// a Result<Window, String> is returned. It is recommended that you match
    /// the result of `Window::new` to provide fallback behaviour, but you should
    /// at least use `.expect` to provide a meaningful error message.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// let w = Window::new(800, 600, "Hello Realms!");
    /// ```
    #[inline]
    pub fn new(width: u32, height: u32, title: &str) -> Result<Window, String> {
        use glfw::fail_on_errors;
        #[expect(clippy::question_mark_used, reason = "? makes this code more concise and does exactly the same as a match statement would")]
        let mut glfw = glfw::init(fail_on_errors!())
            .map_err(|err| format!("Realms: failed to initialise glfw: {err}"))?;

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        #[expect(clippy::question_mark_used, reason = "? makes this code more concise and does exactly the same as a match statement would")]
        let (mut glfw_window, glfw_events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .ok_or("Realms: failed to create glfw window")?;

        glfw_window.make_current();
        glfw_window.set_all_polling(true);

        gl::load_with(|symbol| glfw_window.get_proc_address(symbol).cast());

        Ok(Window {
            glfw,
            glfw_window,
            glfw_events,
        })
    }

    /// Returns a `bool` depending on whether the window has been closed by the
    /// user or not.
    /// If the user has closed the program **or** you have explicitly called
    /// `window.close()`, this function returns `false`.
    /// Otherwise, it returns `true`.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// let w = Window::new(800, 600, "Game Loop");
    /// while w.is_running() {
    ///     w.new_frame();
    ///     ...
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn is_running(&self) -> bool {
        !self.glfw_window.should_close()
    }

    /// Manually close the window.
    /// After calling `close`, the `is_running` function returns `false`.
    /// If you follow the recommended approach of using a
    /// `while window.is_running() {}` game loop, calling this function will
    /// break from the loop at the end of the frame.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     for event in w.events() {
    ///         match event {
    ///             Event::KeyDown(Key::Q) => w.close();
    ///             _ => {}
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn close(&mut self) {
        self.glfw_window.set_should_close(true);
    }

    /// This function should be called at the START of each loop iteration.
    /// It swaps the buffers (blits the image to the screen) and calls the
    /// `new_frame` function on the passed shader program, using it.
    ///
    /// > Note: This function no longer polls for events. You must manually
    /// > poll for events by looping over the `Vec<Event>` returned by
    /// > `window.events()`.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     for event in w.events() {...}
    /// }
    /// ```
    #[inline]
    pub fn new_frame(&mut self, shader_program: &ShaderProgram) {
        self.glfw_window.swap_buffers();
        shader_program.new_frame();
    }

    /// Fills the screen with the specified `Color`.
    ///
    /// Note that as Realms automatically adjusts the OpenGL viewport, the
    /// window can be resized and the full screen will be filled.
    ///
    /// > Technical note: The `Color` is converted to a 4-float opengl color
    /// > using the `color.gl()` function.
    #[inline]
    #[expect(clippy::min_ident_chars, reason = "r, g, b, a is short for red, green, blue, alpha")]
    pub fn fill(&mut self, color: Color) {
        let (r, g, b, a) = color.gl();
        unsafe { gl::ClearColor(r, g, b, a) };
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
    }

    /// Returns a `Vec` of `Event`s gathered this frame. You should loop over
    /// the iterator returned **every frame** and match each event.
    ///
    /// Note: It is recommended to loop over events immediately after you call
    /// `w.new_frame()`.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     for event in window.events() {
    ///         match event {
    ///             Event::KeyDown(Key::Q) => w.close();
    ///             _ => {}
    ///         }
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn events(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        self.glfw.poll_events();
        for (_, glfw_event) in glfw::flush_messages(&self.glfw_events) {
            let event = Event::from_glfw(glfw_event);
            events.push(event);
        }
        for event in &events {
            self.handle_event(event);
        }
        events
    }
    
    /// A private function used internally.
    /// This function is called by `w.events()` and it handles certain events
    /// so the library user doesn't have to.
    ///
    /// - Its only current job is to resize the OpenGL viewport if the window is
    ///   resized.
    #[inline]
    #[expect(clippy::unused_self, reason = "will be used in the future")]
    #[expect(clippy::needless_pass_by_ref_mut, reason = "will need to be mutable in the future")]
    fn handle_event(&mut self, event: &Event) {
        #[expect(clippy::single_match, reason = "more events will be handled in the future")]
            #[expect(clippy::wildcard_enum_match_arm, reason = "future added variants should be ignored anyway")]
        match *event {
            Event::ResizeWindow(width, height)
                => unsafe { gl::Viewport(0, 0, width, height) },
            _ => {},
        };
    }
}

