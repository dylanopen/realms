use crate::Color;

pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Vec2f {
        Vec2f {x, y}
    }

    pub fn add(&mut self, other: &Vec2f) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sub(&mut self, other: &Vec2f) {
        self.x -= other.x;
        self.y -= other.y;
    }

    pub fn mul(&mut self, other: &Vec2f) {
        self.x *= other.x;
        self.y *= other.y;
    }

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

