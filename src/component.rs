use crate::Color;

/// `Vec2f` is a struct for storing an `x` and `y`
/// component, both stored as a 32-bit float (`f32`).
///
/// It is commonly used for coordinates (positions) and
/// dimensions (sizes).
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    /// Create a new `Vec2f` with the specified `x` and `y`
    /// fields.
    ///
    /// ## Example usage
    ///
    /// ```rust
    /// let rect_position = Vec2f::new(100.0, 80.0);
    /// let rect_size = Vec2f::new(200.0, 160.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f {x, y}
    }

    /// Increments `self.x` by `other.x`, as well as
    /// incrementing `self.y` by `other.y`.
    ///
    /// ## Example usage
    ///
    /// ```rust
    /// let mut a = Vec2f::new(20.0, 10.0);
    /// let b = Vec2f::new(5.0, 4.0);
    /// a.add(b);
    /// // `a` is now (25.0, 14.0)
    /// ```
    pub fn add(&mut self, other: &Vec2f) {
        self.x += other.x;
        self.y += other.y;
    }

    /// Decrements `self.x` by `other.x`, as well as
    /// decrementing `self.y` by `other.y`.
    ///
    /// ## Example usage
    ///
    /// ```rust
    /// let mut a = Vec2f::new(20.0, 10.0);
    /// let b = Vec2f::new(5.0, 4.0);
    /// a.sub(b);
    /// // `a` is now (15.0, 6.0)
    /// ```
    pub fn sub(&mut self, other: &Vec2f) {
        self.x -= other.x;
        self.y -= other.y;
    }

    /// Multiplies `self.x` by `other.x`, as well as
    /// multiplying `self.y` by `other.y`.
    ///
    /// ## Example usage
    ///
    /// ```rust
    /// let mut a = Vec2f::new(20.0, 10.0);
    /// let b = Vec2f::new(5.0, 4.0);
    /// a.mul(b);
    /// // `a` is now (100.0, 40.0)
    /// ```
    pub fn mul(&mut self, other: &Vec2f) {
        self.x *= other.x;
        self.y *= other.y;
    }

    /// Divides `self.x` by `other.x`, as well as
    /// dividing `self.y` by `other.y`.
    ///
    /// ## Example usage
    ///
    /// ```rust
    /// let mut a = Vec2f::new(20.0, 10.0);
    /// let b = Vec2f::new(5.0, 4.0);
    /// a.div(b);
    /// // `a` is now (4.0, 2.5)
    /// ```
    pub fn div(&mut self, other: &Vec2f) {
        self.x /= other.x;
        self.y /= other.y;
    }
}


pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>
}

impl Texture {
    pub fn load(path: &str, width: usize, height: usize) -> Texture {
        let mut ril_image = ril::Image::<ril::Rgba>::open(path)
            .expect(&format!("Failed to load image {}", path));
        ril_image.resize(
            width as u32, height as u32,
            ril::ResizeAlgorithm::Nearest
        );

        let mut pixels: Vec<Color> = Vec::with_capacity(width * height);

        for y in 0..height {
            for x in 0..width {
                let ril_pixel = ril_image
                    .get_pixel(x as u32, y as u32)
                    .expect(&format!("Failed to get pixel of ril::Image {}", path));
                let pixel = Color::rgba(ril_pixel.r, ril_pixel.g, ril_pixel.b, ril_pixel.a);
                pixels.push(pixel);
            }
        }

        Texture {
            width,
            height,
            pixels,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Color {
        self.pixels.get(y * self.width + x).expect(&format!(
                "Could not get pixel x={}, y={} of texture (likely out of bounds)",
                x, y
        ))
    }
}

