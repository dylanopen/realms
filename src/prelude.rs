//! Re-exports commonly used structs and functions.
//! Add `use realms::prelude::*` to your code to quickly get started with Realms.

#![expect(clippy::pub_use, reason = "This is a prelude module, we should be ")]

// This actually just re-exports *everything* at the moment, because everything currently in
// Realms will probably be used when you build something.
// When Realms has way more features, many of those new features won't be in the prelude.

pub use crate::data::Color;

pub use crate::input::Event;
pub use crate::input::Key;
pub use crate::input::MouseButton;

pub use crate::shape::Rectangle;
pub use crate::shape::Triangle;
pub use crate::shape::shader_2d;

pub use crate::vertex::VertexBuffer;

pub use crate::texture::Texture;

pub use crate::shader::Shader;
pub use crate::shader::ShaderProgram;
pub use crate::shader::ShaderType;

pub use crate::window::Window;
