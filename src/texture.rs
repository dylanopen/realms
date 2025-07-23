//! The `texture` module stores structs for interacting with image files
//! and loading them as opengl textures.
//!
//! The main struct is the `Texture` struct.

use core::ffi::c_void;
use std::path::Path;

use image::GenericImage as _;

/// This struct stores the opengl id for a given 2D texture.
/// It is a wrapper around creating and binding an opengl `TEXTURE_2D`.
/// You still need to define attributes in the `VertexBuffer` and as inputs to
/// your vertex and fragment shader for the texture's coordinates.
pub struct Texture {
    /// Stores the opengl id for the 2d texture.  
    /// This is used internally by the `Texture` struct. You can't use this
    /// directly, instead use the `bind` method to bind the texture to opengl's
    /// texture buffer.
    gl_id: u32,
}

impl Texture {
    /// Create a 2D `Texture` from the given filepath.
    /// The texture is by default set to repeat when wrapping.
    ///
    /// ## Errors 
    /// 
    /// If the file does not exist or cannot be opened, an `Err` variant
    /// containing a string of what went wrong is returned.
    /// Otherwise, an `Ok` variant containing a `Texture` is returned. You can
    /// bind (enable) this `Texture` each frame using its `bind()` method.
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
    #[inline]
    pub fn load_file(path: &str) -> Result<Texture, String> {
        let mut gl_id = 0;
        
        unsafe { gl::GenTextures(1, &raw mut gl_id) };
        unsafe { gl::BindTexture(gl::TEXTURE_2D, gl_id) };
        #[expect(clippy::as_conversions, clippy::cast_possible_wrap)]
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32) };
        #[expect(clippy::as_conversions, clippy::cast_possible_wrap)]
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32) };
        #[expect(clippy::as_conversions, clippy::cast_possible_wrap)]
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NONE as i32) };
        #[expect(clippy::as_conversions, clippy::cast_possible_wrap)]
        unsafe { gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NONE as i32) };

        let img = image::open(Path::new(path))
            .map_err(|err| format!("Realms: could not open image file {path}: {err}"))?;
        let img = img.flipv();

        let data = img.raw_pixels();
        unsafe {
            #[expect(clippy::borrow_as_ptr, clippy::indexing_slicing, clippy::as_conversions, reason = "can't find other way to specify pixels field")]
            gl::TexImage2D(gl::TEXTURE_2D,
                0,
                gl::RGB.try_into()
                    .map_err(|_e| "gl::RGB returned garbage data which overflowed ")?,
                img.width().try_into()
                    .map_err(|_e| "image width too large, led to overflow")?,
                img.width().try_into()
                    .map_err(|_e| "image width too large, led to overflow")?,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                (&data[0] as *const u8).cast::<c_void>()
            );
        };
        unsafe { gl::GenerateMipmap(gl::TEXTURE_2D); };

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
    #[inline]
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.gl_id) };
    }
}

