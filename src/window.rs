//! The `window` module contains only the `Window` struct.
//! The module holds functionality for creating, managing
//! and handling input from windows.

use crate::{Color, GameTime};

/// `Window` is the main struct for creating a graphical
/// application with Realms.
/// Almost all Realms functions will reference the `Window`
/// struct.
///
/// ## Example usage
///
/// ``` rust
/// use realms::*;
/// fn main() {
///     let w = Window::new("Hello Realms", 800, 600);
///     while w.is_running() {
///         w.new_frame();
///     }
/// }
/// ```
pub struct Window {
    pub running: bool,
    pub time: GameTime,
    pub mini_window: minifb::Window,
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

impl Window {
    /// Create a window with the specified `title`,
    /// `width` and `height`.  
    /// Also creates a frame buffer with the
    /// dimensions specified.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let w = Window::new("title", 800, 600);
    /// ```
    pub fn new(title: &str, width: usize, height: usize) -> Window {
        let buffer = vec![Color::BLACK; width*height];

        let opts = minifb::WindowOptions::default();
        let mut mini_window = minifb::Window::new(
            title, width, height, opts
        ).expect("Realms: failed to create window");
        mini_window.set_target_fps(60);

        Window {
            running: true,
            time: GameTime::new(),
            mini_window,
            width, height,
            buffer,
        }
    }

    /// Returns a bool representing whether the window is
    /// open (`true`) or whether it should close (`false`).
    ///
    /// The result is calculated based on two factors:
    ///   - Whether you have manually closed the window by
    ///     setting the 'running' field to false.
    ///   - Whether the user closed the window.
    ///
    /// If the window was closed in any of these ways, this
    /// function will return `false`, otherwise `true`.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     /* other update and draw logic */
    /// }
    /// ```
    pub fn is_running(&self) -> bool {
        self.mini_window.is_open() && self.running
    }

    /// Modify the frame buffer by setting a pixel to the
    /// specified color.
    ///
    /// If the specified color *is not* opaque (i.e. its
    /// alpha field is less than 255), it's color will be
    /// 'added' to the previous color using the
    /// `Color::add_layer` method. This allows for a simple
    /// way of handling transparency.
    ///
    /// If the specified color *is* opaque, it simply sets
    /// the pixel in the frame buffer to the specified
    /// color.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// // in game loop
    /// w.set_pixel(400, 300, Color::rgb(31, 255, 31));
    /// ```
    pub fn set_pixel(&mut self, x: f32, y: f32, color: Color) {
        let x = x as i32;
        let y = y as i32;
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        self.buffer[(y * self.width as i32 + x) as usize].add_layer(color);
    }
    
    /// This function should be called at the **START** of
    /// each iteration of the game loop.
    ///
    /// It currently does the following:
    ///   1. Sends the frame buffer to minifb to display.
    ///   2. Updates the `time` field.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// while w.is_running() {
    ///     w.new_frame();
    ///     /* update and draw logic here */
    /// }
    /// ```
    pub fn new_frame(&mut self) {
        let mini_buffer = self.buffer_u32();
        self.mini_window.update_with_buffer(&mini_buffer, self.width, self.height)
            .expect("Realms: failed to flip window buffer (cannot update screen)");
        self.time.new_frame();
    }

    /// Set the window's target framerate to the specified
    /// frames per second (FPS).
    ///
    /// Note that this is simply a target and is not
    /// guaranteed to always be reached.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let w = Window::new("fps", 800, 600);
    /// w.set_fps(120);
    /// ```
    pub fn set_fps(&mut self, target_fps: usize) {
        self.mini_window.set_target_fps(target_fps);
    }

    /// Manually close the window.  
    /// This sets the `running` field to `false`.  
    /// `is_running()` will return `false` after this method
    /// is called.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// // in game loop
    /// if key_down(&w, Key::Escape) {
    ///     w.close();
    /// }
    /// ```
    pub fn close(&mut self) {
        self.running = false;
    }

    /// Get the width of the window as a `usize`.
    ///
    /// This function simply returns `self.width`.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let w = Window::new("example", 800, 600);
    /// println!("The window width is {}", w.get_width());
    /// ```
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the height of the window as a `usize`.
    ///
    /// This function simply returns `self.height`.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let w = Window::new("example", 800, 600);
    /// println!("The window height is {}", w.get_height());
    /// ```
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Helper function to convert a Vec of `Color` objects
    /// to a Vec of `u32`s.
    ///
    /// The input is `self.buffer`.
    ///
    /// It is used because `minifb` requires the pixel color
    /// data to be specified as a `u32` - a hexadecimal of
    /// the red, green and blue components of the color.
    ///
    /// > Note: This method is not designed for use outside
    /// of the Window struct, and is therefore private.
    fn buffer_u32(&self) -> Vec<u32> {
        let mut buf: Vec<u32> = Vec::new();
        for pixel in &self.buffer {
            buf.push(pixel.r as u32 * 65536 + pixel.g as u32 * 256 + pixel.b as u32);
        }
        buf
    }
}

