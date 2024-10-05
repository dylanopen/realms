//! This module contains the code for the pixel node
//! (`Pixel`).

use crate::{Color, NodeDraw, Vec2f, Window};

/// The `Pixel` struct stores information about a single
/// pixel.
///
/// This node has a:
///   - Position
///   - Size (constant)
///   - Color
///
/// ## Example usage
///
/// Set the pixel at position `(32, 32)` to `green`.
///
/// ``` rust
/// let mut w = Window::new("pixels", 100, 100);
/// let pixel = Pixel::new(
///     Vec2f::new(32.0, 32.0),
///     Color::rgb(31, 255, 31)
/// );
/// while w.is_running() {
///     w.new_frame();
///     pixel.draw(&mut w);
/// }
/// ```
pub struct Pixel {
    pos: Vec2f,
    color: Color,
}

impl Pixel {
    /// Create a new rectangle with the specified `pos`,
    /// and `color`.  
    /// The size of a pixel is always (1.0, 1.0).
    ///
    /// ## Example usage
    ///
    /// Set the pixel (50, 30) to a semi-transparent blue.  
    /// Note: draw call is omitted.
    ///
    /// ``` rust
    /// let pixel = Pixel::new(
    ///     Vec2f::new(50.0, 30.0),
    ///     Color::rgba(31, 127, 255, 127)
    /// );
    /// ```
    pub fn new(pos: Vec2f, color: Color) -> Pixel {
        Pixel {
            pos,
            color
        }
    }
}

impl NodeDraw for Pixel {
    fn draw(&self, window: &mut Window) {
        window.set_pixel(self.pos.x, self.pos.y, self.color.clone());
    }
}

