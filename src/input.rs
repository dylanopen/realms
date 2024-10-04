use minifb::{Key, MouseButton};

use crate::Window;

pub fn key_down(window: &Window, keycode: Key) -> bool {
    window.mini_window.is_key_down(keycode)
}

pub fn key_just_pressed(window: &Window, keycode: Key) -> bool {
    window.mini_window.is_key_pressed(keycode, minifb::KeyRepeat::No)
}

pub fn key_just_released(window: &Window, keycode: Key) -> bool {
    window.mini_window.is_key_released(keycode)
}

pub fn mouse_pos(window: &Window) -> (i32, i32) {
    let (x, y) = window.mini_window.get_mouse_pos(minifb::MouseMode::Clamp)
        .expect("Realms: failed to get mouse position");
    (x as i32, y as i32)
}

pub fn mouse_down(window: &Window, mouse_button: MouseButton) -> bool {
    window.mini_window.get_mouse_down(mouse_button)
}

pub fn mouse_down_left(window: &Window) -> bool {
    mouse_down(window, MouseButton::Left)
}

pub fn mouse_down_right(window: &Window) -> bool {
    mouse_down(window, MouseButton::Right)
}

pub fn mouse_down_middle(window: &Window) -> bool {
    mouse_down(window, MouseButton::Middle)
}

