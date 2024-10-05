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

