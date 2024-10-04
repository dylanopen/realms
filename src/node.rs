use crate::{Color, Window};

pub trait NodePosition {
    fn get_pos(&self) -> (f32, f32);
    fn set_pos(&mut self, new_pos: (f32, f32));
    fn change_pos(&mut self, delta_pos: (f32, f32));
}

pub trait NodeSize {
    fn get_size(&self) -> (f32, f32);
    fn set_size(&mut self, new_size: (f32, f32));
    fn change_size(&mut self, delta_size: (f32, f32));
}

pub trait NodeColor {
    fn get_color(&self) -> &Color;
    fn get_color_mut(&mut self) -> &mut Color;
    fn set_color(&mut self, new_color: Color);
}

pub trait NodeDraw {
    fn draw(&self, window: &mut Window);
}

