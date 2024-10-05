//! This module contains the code for the sprite image node
//! (`Sprite`).

use crate::{NodeDraw, Texture, Vec2f, Window};

/// The `Sprite` struct stores information about a
/// 'textured rectangle' (sprite).
///
/// This node has a:
///   - Position
///   - Texture (ref)
/// 
///
/// ## Example usage
///
/// Load "res/image.png" into a 256x256 texture and display
/// it at (128, 128) using a Sprite.
///
/// ```rust
/// let texture = Texture::load("res/image.png", 256, 256);
/// let sprite = Sprite::new(
///     Vec2f::new(128.0, 128.0),
///     &texture
/// );
/// ```
pub struct Sprite<'a> {
    pub pos: Vec2f,
    pub texture: &'a Texture,
}

impl<'a> Sprite<'a> {
    /// Create a new `Sprite` with the specified `pos` and
    /// `texture` components.
    ///
    /// ## Example usage
    ///
    /// Load "res/image.png" into a 256x256 texture and display
    /// it at (128, 128) using a Sprite.
    ///
    /// ```rust
    /// let texture = Texture::load("res/image.png", 256, 256);
    /// let sprite = Sprite::new(
    ///     Vec2f::new(128.0, 128.0),
    ///     &texture
    /// );
    /// ```
    pub fn new(pos: Vec2f, texture: &'a Texture) -> Sprite<'a> {
        Sprite {
            pos, texture
        }
    }
}

impl<'a> NodeDraw for Sprite<'a> {
    fn draw(&self, window: &mut Window) {
        for x in 0..self.texture.width {
            for y in 0..self.texture.height {
                let screen_x = x as f32 + self.pos.x;
                let screen_y = y as f32 + self.pos.y;
                let color = self.texture.get(x, y);
                window.set_pixel(screen_x, screen_y, color.clone());
            }
        }
    }
}

