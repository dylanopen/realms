//! The `data` module stores structs and functions that are too small to be
//! their own module, such as the `Color` and `GameTime` struct.

/// The `Color` struct is used to represent an RGBA color.
/// It stores the red, green, blue and alpha components as `u8`s.
#[derive(Clone, Copy)]
#[expect(clippy::exhaustive_structs, reason = "Colors will only ever consist of red, green blue and alpha")]
pub struct Color {
    /// The `red` component
    pub r: u8,

    /// The `green` component
    pub g: u8,

    /// The `blue` component
    pub b: u8,

    /// The `alpha` (opacity) component
    pub a: u8,
}

impl Color {
    
    /// A const for representing the color black: 
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };

    /// Construct a `Color` object from the specified red, green, blue and alpha
    /// (opacity) components.  
    /// These components should be specified as a u8 value (between 0-255).
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let transparent = Color::rgba(0, 0, 0, 0);
    /// let white = Color::rgba(255, 255, 255, 255);
    /// let translucent_blue = Color::rgba(31, 127, 255, 100);
    /// ```
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    /// Construct a `Color` object from the specified red, green and blue
    /// components.  
    /// These components should be specified as a u8 value (between 0-255).
    ///
    /// Alpha (`a`) is assumed to be `255` (fully opaque).
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let red = Color::rgb(255, 0, 0);
    /// let green = Color::rgb(0, 255, 0);
    /// let blue = Color::rgb(0, 0, 255);
    /// let yellow = Color::rgb(255, 255, 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    /// Construct a new Color object with the color value `black`.
    ///
    /// Shorthand for `Color::rgb(0, 0, 0)`.
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let black = Color::new();
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Color {
        Color::rgba(0, 0, 0, 255)
    }

    /// Convert a Realms `Color` into a tuple of 4 f32s, for use by opengl
    /// and (optionally) custom shaders and `VertexBuffer`s.
    #[inline]
    #[must_use]
    pub fn gl(&self) -> (f32, f32, f32, f32) {
        ( f32::from(self.r) / 255.0,
          f32::from(self.g) / 255.0,
          f32::from(self.b) / 255.0,
          f32::from(self.a) / 255.0, )
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Color::rgba(0, 0, 0, 255)
    }
}

