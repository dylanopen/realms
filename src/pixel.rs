//! This module contains the code for the pixel node
//! (`Pixel`).

use crate::{Color, NodeColor, NodeDraw, NodePosition, NodeSize, Window};

/// The `Pixel` struct stores information about a single
/// pixel.
/// This node has a:
///   - Position
///   - Size (constant)
///   - Color
pub struct Pixel {
    pos: (f32, f32),
    color: Color,
}

impl Pixel {
    /// Create a new rectangle with the specified `pos`,
    /// and `color`.
    /// The size of a pixel is always (1.0, 1.0).
    pub fn new(pos: (f32, f32), color: Color) -> Pixel {
        Pixel {
            pos,
            color
        }
    }
}

impl NodePosition for Pixel {
    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    fn set_pos(&mut self, new_pos: (f32, f32)) {
        self.pos = new_pos
    }

    fn change_pos(&mut self, delta_pos: (f32, f32)) {
       self.pos.0 += delta_pos.0;
       self.pos.1 += delta_pos.1;
    }
}

impl NodeSize for Pixel {

    /// The size of any pixel is (1.0, 1.0).
    fn get_size(&self) -> (f32, f32) {
        (1.0, 1.0)
    }
    
    /// PANIC! The size of a pixel is always (1.0, 1.0).
    fn set_size(&mut self, _new_size: (f32, f32)) {
        panic!("Realms: cannot change size of Pixel - Pixels always have a size of (1.0, 1.0)");
    }

    /// PANIC! The size of a pixel is always (1.0, 1.0).
    fn change_size(&mut self, _delta_size: (f32, f32)) {
        panic!("Realms: cannot change size of Pixel - Pixels always have a size of (1.0, 1.0)");
    }
}

impl NodeColor for Pixel {
    fn get_color(&self) -> &Color {
        &self.color
    }
    
    fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    fn set_color(&mut self, new_color: Color) {
        self.color = new_color
    }
}

impl NodeDraw for Pixel {
    fn draw(&self, window: &mut Window) {
        window.set_pixel(self.pos.0, self.pos.1, self.color.clone());
    }
}

