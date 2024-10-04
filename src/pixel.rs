use crate::{Color, NodeColor, NodeDraw, NodePosition, NodeSize, Window};


pub struct Pixel {
    pos: (f32, f32),
    color: Color,
}

impl Pixel {
    pub fn new(pos: (f32, f32), color: Color) -> Pixel {
        Pixel {
            pos,
            color
        }
    }
}

impl NodePosition for Pixel {
    fn get_pos(&self) -> (f32, f32) {
        self.pos
    }

    fn set_pos(&mut self, new_pos: (f32, f32)) {
        self.pos = new_pos
    }

    fn change_pos(&mut self, delta_pos: (f32, f32)) {
       self.pos.0 += delta_pos.0;
       self.pos.1 += delta_pos.1;
    }
}

impl NodeSize for Pixel {
    fn get_size(&self) -> (f32, f32) {
        (1.0, 1.0)
    }

    fn set_size(&mut self, _new_size: (f32, f32)) {
        panic!("Realms: cannot change size of Pixel - Pixels always have a size of (1.0, 1.0)");
    }

    fn change_size(&mut self, _delta_size: (f32, f32)) {
        panic!("Realms: cannot change size of Pixel - Pixels always have a size of (1.0, 1.0)");
    }
}

impl NodeColor for Pixel {
    fn get_color(&self) -> &Color {
        &self.color
    }
    
    fn get_color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    fn set_color(&mut self, new_color: Color) {
        self.color = new_color
    }
}

impl NodeDraw for Pixel {
    fn draw(&self, window: &mut Window) {
        window.set_pixel(self.pos.0, self.pos.1, self.color.clone());
    }
}

