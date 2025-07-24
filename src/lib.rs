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

#![warn(
     clippy::all,
     clippy::restriction,
     clippy::pedantic,
     clippy::nursery,
     clippy::cargo,
     clippy::correctness,
     clippy::suspicious,
     clippy::style,
     clippy::complexity,
     clippy::perf,
     missing_docs,
 )]

 #![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::must_use_candidate,
    clippy::allow_attributes_without_reason,
    clippy::needless_return,
    clippy::question_mark_used,
    clippy::use_self,
    clippy::implicit_return,
    clippy::arbitrary_source_item_ordering,
    clippy::doc_comment_double_space_linebreaks,
    clippy::semicolon_inside_block,
    clippy::undocumented_unsafe_blocks,
    clippy::min_ident_chars,
    clippy::float_arithmetic,
    clippy::separated_literal_suffix,
    clippy::unnecessary_semicolon,
    clippy::single_call_fn,
    clippy::shadow_same,
    clippy::shadow_reuse,
    clippy::too_long_first_doc_paragraph,
    clippy::module_name_repetitions,
    clippy::pub_use,
 )]


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

