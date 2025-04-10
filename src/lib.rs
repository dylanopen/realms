//! Welcome to Realms!  
//! Realms is a lightweight, simple and powerful library which provides
//! graphics and game functionality.  
//! 
//! Homepage: <https://github.com/dylanopen/realms>  
//! Documentation: <https://docs.rs/realms>  


// ## LINTS
//
// Realms intentionally uses many clippy lints in order to ensure code stays
// as readable and maintainable as possible.
// While it is always preferred to actually *fix* the lints that Clippy
// detects, if a lint makes adding / updating a feature very difficult, you
// are welcome to sprinkle `#[expect(...)]` throughout your code if you
// absolutely need to.
//
// If you believe a lint should be removed from Realms, please make an issue
// or PR explaining why it should be removed.

#![warn(missing_docs)]
#![warn(clippy::absolute_paths)]
#![warn(clippy::allow_attributes)]
#![warn(clippy::allow_attributes_without_reason)]


pub mod window;
pub mod input;
pub mod shader;
pub mod data;
pub mod vertex;
pub mod texture;

pub use gl;
pub use glfw;



