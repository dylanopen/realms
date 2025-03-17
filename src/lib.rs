//! Welcome to Realms!  
//! Realms is a lightweight, simple and powerful library which provides graphics and game functionality.  
//! 
//! Homepage: <https://github.com/dylanopen/realms>  
//! Documentation: <https://docs.rs/realms>  

#![warn(missing_docs)]

#[allow(clippy::similar_names)]
#[allow(clippy::pub_use)]
#[allow(clippy::arbitrary_source_item_ordering)]

pub mod window;
pub mod input;
pub mod shader;
pub mod data;
pub mod vertex;
pub mod texture;

pub use gl;
pub use glfw;

