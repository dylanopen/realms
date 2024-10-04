//! This module contains the traits implemented by nodes.
//! They define the structure and methods of nodes.
//! This module does not *implement* any functionality, but
//! instead simply defines the methods of each node struct.

use crate::{Color, Window};

/// This trait is used for nodes which have a **position**
/// on the window buffer.
pub trait NodePosition {

    /// Returns the position of the node as an `(f32, f32)`
    /// tuple.
    fn get_pos(&self) -> (f32, f32);

    /// Sets the position of the node to the `new_pos`
    /// position.
    fn set_pos(&mut self, new_pos: (f32, f32));

    /// Changes the position of the node by adding the `x`
    /// and `y` components together and storing the result
    /// in `self`.
    fn change_pos(&mut self, delta_pos: (f32, f32));
}

/// This trait is used for nodes with a display size.
/// It specifies the dimensions of a node on the buffer.
pub trait NodeSize {

    /// Returns the display size of the node as an
    /// `(f32, f32)` tuple.
    fn get_size(&self) -> (f32, f32);

    /// Sets the display size of the node to the specified
    /// `new_size`.
    ///
    /// > Note: not all nodes support resizing, so this
    /// > function may panic.
    fn set_size(&mut self, new_size: (f32, f32));

    /// Changes the position of the node by adding the
    /// `width` and `height` components together and
    /// storing the result in `self`.
    ///
    /// > Note: not all nodes support resizing, so this
    /// > function may panic.
    fn change_size(&mut self, delta_size: (f32, f32));
}

pub trait NodeColor {

    /// Returns an immutable reference to the `Color` of
    /// the node.
    ///
    /// If you need a mutable reference (i.e. to modify the
    /// color), use `get_color_mut`.
    /// To set the value of the color, use `set_color`.
    fn get_color(&self) -> &Color;


    fn get_color_mut(&mut self) -> &mut Color;


    fn set_color(&mut self, new_color: Color);
}

pub trait NodeDraw {
    
    /// Writes the node to the frame buffer (displays it on
    /// the screen).
    /// This method should be called each frame in order
    /// for it to be visible.
    fn draw(&self, window: &mut Window);
}

