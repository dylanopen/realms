#[derive(Clone)]
pub struct Color {

    // The `red` component
    pub r: u8,

    // The `green` component
    pub g: u8,

    // The `blue` component
    pub b: u8,

    // The `alpha` (opacity) component
    pub a: u8,
}

impl Color {
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
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
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
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
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
    pub fn new() -> Color {
        Color::rgba(0, 0, 0, 255)
    }

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
    ///
    /// TODO: Make this documentation more understandable!
    pub fn add_layer(&mut self, other: Color) {
        if other.a == 255 {
            *self = other.clone();
        }
        let other_alpha = other.a as f64 / 255.0;
        let old_alpha = 1.0 - other_alpha;
        self.r = (self.r as f64 * old_alpha + other.r as f64 * other_alpha) as u8;
        self.g = (self.g as f64 * old_alpha + other.g as f64 * other_alpha) as u8;
        self.b = (self.b as f64 * old_alpha + other.b as f64 * other_alpha) as u8;
    }

    pub fn gl(&self) -> (f32, f32, f32, f32) {
        ( self.r as f32 / 255.0,
          self.g as f32 / 255.0,
          self.b as f32 / 255.0,
          self.a as f32 / 255.0, )
    }
}

