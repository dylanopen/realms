//! The `texture` module stores structs for interacting with image files
//! and loading them as opengl textures.
//!
//! The main struct is the `Texture` struct.

use std::ffi::c_void;
use std::path::Path;

use image::GenericImage;

/// This struct stores the opengl id for a given 2D texture.
/// It is a wrapper around creating and binding an opengl `TEXTURE_2D`.
/// You still need to define attributes in the `VertexBuffer` and as inputs to
/// your vertex and fragment shader for the texture's coordinates.
pub struct Texture {
    gl_id: u32,
}

impl Texture {
    /// Create a 2D `Texture` from the given filepath.
    /// The texture is by default set to repeat when wrapping.
    ///
    /// If the file does not exist or cannot be opened, an `Err` variant
    /// containing a string of what went wrong is returned.
    /// Otherwise, a `Texture` is returned. You can bind (enable) this
    /// `Texture` each frame using its `bind()` method.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// let tex = Texture::new("res/texturemap.png");
    /// while w.is_running() {
    ///     tex.bind();
    ///     vertex_buffer.draw();
    /// }
    /// ```
    pub fn load_file(path: &str) -> Result<Texture, String> {
        let mut gl_id = 0;
        unsafe {
            gl::GenTextures(1, &mut gl_id);
            gl::BindTexture(gl::TEXTURE_2D, gl_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NONE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NONE as i32);
            let img = image::open(Path::new(path))
                .map_err(|err| format!("Realms: could not open image file {}: {}", path, err))?;
            let img = img.flipv();
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D,
                0,
                gl::RGB/*A*/ as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        
        Ok(Texture {
            gl_id
        })
    }

    /// Bind the texture. This makes it the 'in-use' texture by the GPU.
    /// While you can swap between many textures if you so wish, this is
    /// expensive. It is recommended to just bind one texture before the loop
    /// and use this texture as an *atlas* of many textures, accessed via
    /// coordinates *from 0.0 to 1.0*.
    ///
    /// ## Example usage:
    ///
    /// ``` rust
    /// let tex = Texture::new("res/texturemap.png");
    /// tex.bind();
    /// while w.is_running() {
    ///     w.new_frame();
    ///     // {...} //
    ///     vertex_buffer.draw();
    /// }
    /// ```
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
        }
    }
}

