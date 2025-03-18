//! The `data` module stores structs and functions that are too small to be
//! their own module, such as the `Color` and `GameTime` struct.

/// The `Color` struct is used to represent an RGBA color.
/// It stores the red, green, blue and alpha components as a 
#[derive(Clone, Copy)]
#[expect(clippy::min_ident_chars, reason = "r, g, b and a are widely used abbreviations for red, green, blue and alpha")]
#[expect(clippy::exhaustive_structs, reason = "colors will only ever be made up of red, green, blue and alpha, so this struct is exhaustive")]
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

    /// Construct a `Color` object from the specified red,
    /// green, blue and alpha (opacity) components.  
    /// These components should be specified as a u8 value
    /// (between 0-255).
    ///
    /// ## Example usage
    ///
    /// ``` rust
    /// let transparent = Color::rgba(0, 0, 0, 0);
    /// let white = Color::rgba(255, 255, 255, 255);
    /// let translucent_blue = Color::rgba(31, 127, 255, 100);
    /// ```
    #[expect(clippy::min_ident_chars, reason = "r, g, b and a are widely used abbreviations")]
    #[inline]
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    /// Construct a `Color` object from the specified red,
   /// green and blue components.  
    /// These components should be specified as a u8 value
    /// (between 0-255).
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
    #[expect(clippy::min_ident_chars, reason = "r,g,b,a is universally abbreviated")]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    /// Construct a new Color object with the color value
    /// `black`.
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

    /// NOTE: While this function was used all the time in the
    /// old framebuffer-based Realms library, it is rarely used
    /// now that Realms uses opengl. You are welcome to use it,
    /// however it will rarely be updated and may be unstable.
    ///
    /// This function mutates `self` to represent the color
    /// result if the `other` color was 'added on top of'
    /// the existing `self` color.
    ///
    /// If `other` is opaque, `self` becomes `other` (in
    /// other words, the object is set to the value of
    /// `other`).
    ///
    /// If `other` is *not* opaque, `self` is set to the
    /// original value of self, but blended with `other`.
    /// The weight of each color (how much it affects the
    /// resulting color) is determined by the `alpha`
    /// channel of `other`. If `other`'s alpha channel is
    /// lower (more transparent), the result will be more
    /// similar to the original `self`. If `other`'s alpha
    /// channel is higher (more opaque), the result will be
    /// more similar to `other`.
    #[deprecated(since = "0.2.1", note = "no longer needed now that Realms uses opengl")]
    #[inline]
    #[expect(clippy::float_arithmetic, reason = "we need float arithmetic for calculating the alpha values")]
    #[expect(clippy::as_conversions, reason = "while `as` can crash, given the inputs, it is unlikely to; `as` is also necessary here")]
    #[expect(clippy::suboptimal_flops, reason = "add_layer function no longer maintained")]
    #[expect(clippy::cast_possible_truncation, reason = "no other feasible way to convert f64->u8 (other than `as`)")]
    #[expect(clippy::cast_sign_loss, reason = "no other feasible way to convert f64->u8 (other than `as`)")]
    pub fn add_layer(&mut self, other: Color) {
        if other.a == 255 {
            *self = other;
        }
        let other_alpha = f64::from(other.a) / 255.0f64;
        let old_alpha = 1.0f64 - other_alpha;
        self.r = (f64::from(self.r) * old_alpha + f64::from(other.r) * other_alpha) as u8;
        self.g = (f64::from(self.g) * old_alpha + f64::from(other.g) * other_alpha) as u8;
        self.b = (f64::from(self.b) * old_alpha + f64::from(other.b) * other_alpha) as u8;
    }

    /// Convert a Realms `Color` into a tuple of 4 float32s, for use by opengl
    /// and (optionally) custom shaders and `VertexBuffer`s.
    #[expect(clippy::float_arithmetic, reason = "we need float arithmetic for calculating the opengl-style color value")]
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

