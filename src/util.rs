use macroquad::prelude::{screen_height, screen_width};

pub fn is_out_of_bounds_top(y: f32) -> bool {
    y <= 0.
}

pub fn is_out_of_bounds_left(x: f32) -> bool {
    x <= 0.
}

pub fn is_out_of_bounds_bottom(y: f32) -> bool {
    y >= screen_height()
}

pub fn is_out_of_bounds_right(x: f32) -> bool {
    x >= screen_width()
}
