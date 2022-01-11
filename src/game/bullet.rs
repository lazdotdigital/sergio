use crate::config::*;
use crate::util::*;
use macroquad::prelude::*;

pub struct Bullet {
    y: f32,
    x: f32,
    captured_key: KeyCode,
}

impl Bullet {
    pub fn new(x: f32, y: f32, last_move_key_pressed: KeyCode) -> Self {
        Self {
            x,
            y,
            captured_key: last_move_key_pressed,
        }
    }

    pub fn tick_fire(&mut self) -> bool {
        match self.captured_key {
            KeyCode::A => {
                if is_out_of_bounds_left(self.x) {
                    return true;
                }
                self.x -= BULLET_SPEED;
                false
            }
            _ => {
                if is_out_of_bounds_right(self.x) {
                    return true;
                }
                self.x += BULLET_SPEED;
                false
            }
        }
    }

    pub fn draw(&mut self) {
        draw_line(self.x, self.y, self.x + BULLET_SIZE, self.y, 1., WHITE)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}
