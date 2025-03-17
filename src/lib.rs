//! Welcome to Realms!  
//! Realms is a lightweight, simple and powerful library which provides graphics and game functionality.  
//! 
//! Homepage: <https://github.com/dylanopen/realms>  
//! Documentation: <https://docs.rs/realms>  

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod window;
pub mod input;
pub mod shader;
pub mod data;
pub mod vertex;
pub mod texture;

pub use gl;
pub use glfw;

