//! This module contains the traits implemented by nodes.
//! They define the structure and methods of nodes.  
//! This module does not *implement* any functionality, but
//! instead simply defines the methods of each node struct.

use crate::Window;

/// This trait is used for all nodes that can be displayed
/// (any nodes that have a draw() function).  
/// It provides the `draw()` method.
pub trait NodeDraw {
    
    /// Writes the node to the frame buffer (displays it on
    /// the screen).
    ///
    /// This method should be called each frame in order
    /// for it to be visible.
    ///
    /// ## Example usage
    ///
    /// ```rust
    /// // in game loop
    /// node.draw(&mut w);
    /// ```
    fn draw(&self, window: &mut Window);
}

