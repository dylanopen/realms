use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgba(r, g, b, 255)
    }

    pub fn new() -> Color {
        Color::rgb(0, 0, 0)
    }

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
}

pub struct GameTime {
    delta: Duration,
    startup: Instant,
    frame_start: Instant,
}

impl GameTime {
    pub fn new() -> GameTime {
        GameTime {
            delta: Duration::new(0, 0),
            startup: Instant::now(),
            frame_start: Instant::now()
        }
    }

    pub fn new_frame(&mut self) {
        let now = Instant::now();
        self.delta = now - self.frame_start;
        self.frame_start = now;
    }
    
    pub fn delta(&self) -> Duration {
        self.delta
    }

    pub fn fps(&self) -> f32 {
        1.0 / self.delta.as_secs_f32()
    }

    pub fn runtime(&self) -> Duration {
        Instant::now() - self.startup
    }
}

