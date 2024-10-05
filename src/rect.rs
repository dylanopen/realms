//! This module contains the code for the rectangle node
//! (`Rect`).

use crate::{Color, NodeDraw, Vec2f, Window};


pub struct Rect {
    pub pos: Vec2f,
    pub size: Vec2f,
    pub color: Color,
}

impl Rect {

    /// Create a new rectangle with the specified `pos`,
    /// `size` and `color`.
    ///
    /// ## Example usage
    ///
    /// A `cyan` rectangle at position `(100, 100)` and
    /// size `(50, 50)`.  
    /// Note: draw call is omitted.
    ///
    /// ``` rust
    /// let rect = Rect::new(
    ///     Vec2f::new(100.0, 100.0),
    ///     Vec2f::new(50.0, 50.0),
    ///     Color::rgb(0, 255, 255)
    /// );
    /// ```
    pub fn new(pos: Vec2f, size: Vec2f, color: Color) -> Rect {
        Rect {
            pos, size, color
        }
    }

    /// Create a rectangle with the specified `color` that
    /// fills the entire screen.  
    /// This means the position is (0.0, 0.0) and the size
    /// is the window size.
    ///
    /// ## Example usage
    ///
    /// Fill the screen `orange`.  
    /// Note: draw call is omitted.
    ///
    /// ``` rust
    /// let fill = Rect::fill(
    ///     &w, // need to pass in window for size field
    ///     Color::rgb(255, 127, 0)
    /// );
    /// ```
    pub fn fill(window: &Window, color: Color) -> Rect {
        let size = Vec2f::new(window.get_width() as f32, window.get_height() as f32);
        Rect {
            pos: Vec2f::new(0.0, 0.0),
            size,
            color,
        }
    }
}

impl NodeDraw for Rect {
    fn draw(&self, window: &mut Window) {
        for y in 0..self.size.y as i32 {
            for x in 0..self.size.x as i32 {
                let x = x as f32;
                let y = y as f32;
                window.set_pixel(self.pos.x + x, self.pos.y + y, self.color.clone());
            }
        }
    }
}


