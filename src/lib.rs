//! Welcome to Realms: the lightweight, simple and powerful library which
//! provides graphics and game functionality.  
//! 
//! Homepage: <https://dylancode.dev/realms>.
//! Repository: <https://github.com/dylanopen/realms>.
//! Documentation: <https://docs.rs/realms>.


// ## Internal: Lints
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

// Realms modules //
pub mod window;
pub mod input;
pub mod shader;
pub mod data;
pub mod vertex;
pub mod texture;
pub mod shape;

// External crates //
pub extern crate gl;
pub extern crate glfw;

