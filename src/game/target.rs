use crate::config::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Target {
    x: f32,
    y: f32,
}

impl Target {
    pub fn new() -> Self {
        Self {
            x: rand::gen_range(TARGET_PADDING, screen_width() - TARGET_PADDING),
            y: rand::gen_range(TARGET_PADDING, screen_height() - TARGET_PADDING),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, TARGET_WIDTH, TARGET_HEIGHT, WHITE);
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
