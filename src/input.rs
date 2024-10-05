//! This module contains functions for checking user input.  
//! It can be used to check pressed keys, the mouse
//! position, pressed mouse buttons, and other input
//! functionality.

use minifb::{Key, MouseButton};

use crate::Window;

/// Returns `true` if the specified `key` is down,
/// otherwise `false`.
pub fn key_down(window: &Window, key: Key) -> bool {
    window.mini_window.is_key_down(key)
}

/// Returns `true` if the specified `key` was pressed this
/// frame, otherwise `false`.
pub fn key_just_pressed(window: &Window, key: Key) -> bool {
    window.mini_window.is_key_pressed(key, minifb::KeyRepeat::No)
}

/// Returns `true` if the specified `key` was released this
/// frame, otherwise `false`.
pub fn key_just_released(window: &Window, key: Key) -> bool {
    window.mini_window.is_key_released(key)
}

/// Returns the x and y coordinates of the mouse cursor,
/// relative to the top-left of the window, as an
/// `(i32, i32)` tuple.
///
/// > Note: the result is 'clamped' to the window - the
/// > values returned are guaranteed to be positive and
/// > within the window border.
pub fn mouse_pos(window: &Window) -> (i32, i32) {
    let (x, y) = window.mini_window.get_mouse_pos(minifb::MouseMode::Clamp)
        .expect("Realms: failed to get mouse position");
    (x as i32, y as i32)
}

/// Returns `true` if the specified mouse button is held
/// down, `false` otherwise.
pub fn mouse_down(window: &Window, button: MouseButton) -> bool {
    window.mini_window.get_mouse_down(button)
}

/// Returns `true` if the left mouse button is (held)
/// down, `false` otherwise.
pub fn mouse_down_left(window: &Window) -> bool {
    mouse_down(window, MouseButton::Left)
}

/// Returns `true` if the right mouse button is (held)
/// down, `false` otherwise.
pub fn mouse_down_right(window: &Window) -> bool {
    mouse_down(window, MouseButton::Right)
}

/// Returns `true` if the middle mouse button is (held)
/// down, `false` otherwise.
pub fn mouse_down_middle(window: &Window) -> bool {
    mouse_down(window, MouseButton::Middle)
}

