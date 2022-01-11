use crate::{config::*, sprite::Sprite, util::*};
use macroquad::prelude::*;

pub struct Player {
    x: f32,
    y: f32,
    sprites: Sprites,
    attacking: bool,
}

impl Player {
    pub fn new(sprites: Sprites) -> Self {
        Self {
            x: screen_width() / 2.,
            y: screen_height() / 2.,
            sprites,
            attacking: false,
        }
    }

    pub fn input_handler(&mut self, last_move_key_pressed: &mut KeyCode) {
        if is_key_down(KeyCode::W) {
            self.move_up();
            *last_move_key_pressed = KeyCode::W;
        }
        if is_key_down(KeyCode::A) {
            self.move_left();
            *last_move_key_pressed = KeyCode::A;
        }
        if is_key_down(KeyCode::S) {
            self.move_down();
            *last_move_key_pressed = KeyCode::S;
        }
        if is_key_down(KeyCode::D) {
            self.move_right();
            *last_move_key_pressed = KeyCode::D;
        }
    }

    fn move_up(&mut self) {
        if !is_out_of_bounds_top(self.y) {
            self.y -= MOVE_SPEED;
        }
    }

    fn move_left(&mut self) {
        if !is_out_of_bounds_left(self.x) {
            self.x -= MOVE_SPEED;
        }
    }

    fn move_down(&mut self) {
        if !is_out_of_bounds_bottom(self.y) {
            self.y += MOVE_SPEED;
        }
    }

    fn move_right(&mut self) {
        if !is_out_of_bounds_right(self.x) {
            self.x += MOVE_SPEED;
        }
    }

    pub fn draw(&mut self, last_move_key_pressed: KeyCode) {
        let flip_x = if let KeyCode::A = last_move_key_pressed {
            true
        } else {
            false
        };

        if self.attacking {
            if self.sprites.attack.draw(self.x, self.y, flip_x, false) {
                self.attacking = false;
            }
        } else {
            self.sprites.idle.draw(self.x, self.y, flip_x, false);
        }
    }

    pub fn start_attacking(&mut self) {
        self.attacking = true;
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

pub struct Sprites {
    pub idle: Sprite,
    pub attack: Sprite,
}
