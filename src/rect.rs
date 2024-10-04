use crate::{Color, NodeColor, NodeDraw, NodePosition, NodeSize, Window};

pub struct Rect {
    pos: (f32, f32),
    size: (f32, f32),
    color: Color
}

impl NodePosition for Rect {
    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    fn set_pos(&mut self, new_pos: (f32, f32)) {
        self.pos = new_pos;
    }

    fn change_pos(&mut self, delta_pos: (f32, f32)) {
        self.pos.0 += delta_pos.0;
        self.pos.1 += delta_pos.1;
    }
}

impl NodeSize for Rect {
    fn get_size(&self) -> (f32, f32) {
        self.size
    }

    fn set_size(&mut self, new_size: (f32, f32)) {
        self.size = new_size;
    }

    fn change_size(&mut self, delta_size: (f32, f32)) {
        self.size.0 += delta_size.0;
        self.size.1 += delta_size.1;
    }
}

impl NodeColor for Rect {
    fn get_color(&self) -> &Color {
        &self.color
    }
    
    fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}

impl NodeDraw for Rect {
    fn draw(&self, window: &mut Window) {
        let window_width = window.get_width() as i32;
        let window_height = window.get_height() as i32;
        for y in 0..self.size.1 as i32 {
            for x in 0..self.size.0 as i32 {
                if x < 0 || y < 0 || x >= window_width as i32 || y >= window_height {
                    continue;
                }
                let x = x as f32;
                let y = y as f32;
                window.set_pixel(self.pos.0 + x, self.pos.1 + y, self.color.clone());
            }
        }
    }
}

impl Rect {
    pub fn new(pos: (f32, f32), size: (f32, f32), color: Color) -> Rect {
        Rect {
            pos, size, color
        }
    }

    pub fn fill(window: &Window, color: Color) -> Rect {
        let size = (window.get_width() as f32, window.get_height() as f32);
        Rect {
            pos: (0.0, 0.0),
            size,
            color,
        }
    }
}

