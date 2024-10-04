use crate::{Color, GameTime};

pub struct Window {
    pub running: bool,
    pub time: GameTime,
    pub mini_window: minifb::Window,
    width: usize,
    height: usize,
    buffer: Vec<Color>,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Window {
        let buffer = vec![Color::BLACK; width*height];

        let opts = minifb::WindowOptions::default();
        let mut mini_window = minifb::Window::new(
            title, width, height, opts
        ).expect("Realms: failed to create window");
        mini_window.set_target_fps(60);

        Window {
            running: true,
            time: GameTime::new(),
            mini_window,
            width, height,
            buffer,
        }
    }

    pub fn is_running(&self) -> bool {
        self.mini_window.is_open() && self.running
    }

    pub fn set_pixel(&mut self, x: f32, y: f32, color: Color) {
        let x = x as i32;
        let y = y as i32;
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        self.buffer[(y * self.width as i32 + x) as usize].add_layer(color);
    }

    pub fn new_frame(&mut self) {
        let mini_buffer = self.buffer_u32();
        self.mini_window.update_with_buffer(&mini_buffer, self.width, self.height)
            .expect("Realms: failed to flip window buffer (cannot update screen)");
        self.time.new_frame();
    }

    pub fn set_fps(&mut self, target_fps: usize) {
        self.mini_window.set_target_fps(target_fps);
    }

    pub fn close(&mut self) {
        self.running = false;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn buffer_u32(&self) -> Vec<u32> {
        let mut buf: Vec<u32> = Vec::new();
        for pixel in &self.buffer {
            buf.push(pixel.r as u32 * 65536 + pixel.g as u32 * 256 + pixel.b as u32);
        }
        buf
    }
}

