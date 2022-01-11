mod bullet;
mod player;
mod target;

use crate::{config::*, sprite::Sprite};
use bullet::Bullet;
use macroquad::prelude::*;
use player::Player;
use target::Target;

pub struct Game {
    player: Player,
    targets: Vec<Target>,
    bullet: Option<Bullet>,
    last_target_time: f64,
    last_move_key_pressed: KeyCode,
    score: isize,
}

impl Game {
    pub async fn new() -> Self {
        let player_sprites = player::Sprites {
            idle: Sprite::new("assets/aku.png", 59., 30., 6, 1).await,
            attack: Sprite::new("assets/aku.png", 59., 30., 3, 10).await,
        };

        Self {
            player: Player::new(player_sprites),
            targets: Vec::new(),
            bullet: None,
            last_target_time: get_time(),
            last_move_key_pressed: KeyCode::D,
            score: 0,
        }
    }

    pub fn input_handler(&mut self) {
        self.player.input_handler(&mut self.last_move_key_pressed);
        self.space_handler();
    }

    fn space_handler(&mut self) {
        if is_key_down(KeyCode::Space) {
            if let None = self.bullet {
                self.bullet = Some(Bullet::new(
                    self.player.x(),
                    self.player.y(),
                    self.last_move_key_pressed,
                ));
                self.player.start_attacking();
            }
        }
    }

    pub fn draw(&mut self) {
        if self.should_add_target() {
            self.add_target();
        }

        self.player.draw(self.last_move_key_pressed);
        self.draw_bullet();
        self.draw_targets();
        self.handle_hit_target();
        self.draw_score();
    }

    fn should_add_target(&self) -> bool {
        get_time() - self.last_target_time >= TARGET_SPAWN_RATE_SECS
    }

    fn add_target(&mut self) {
        self.last_target_time = get_time();
        self.targets.push(Target::new());
    }

    fn draw_bullet(&mut self) {
        if let Some(bullet) = &mut self.bullet {
            bullet.draw();
            if bullet.tick_fire() {
                self.bullet = None;
            }
        }
    }

    fn draw_targets(&mut self) {
        for target in &self.targets {
            target.draw();
        }
    }

    fn handle_hit_target(&mut self) {
        for (i, target) in self.targets.clone().iter().enumerate() {
            if self.bullet_collided_with_target(target) {
                self.score += 2;
                self.targets.remove(i);
                self.bullet = None;
            }
        }
    }

    fn bullet_collided_with_target(&self, target: &Target) -> bool {
        if let Some(bullet) = &self.bullet {
            let x_check = (bullet.x() - target.x()).abs() <= TARGET_WIDTH;
            let y_check = (bullet.y() - target.y()).abs() <= TARGET_HEIGHT;

            return x_check && y_check;
        }
        false
    }

    fn draw_score(&self) {
        let text = format!("Score: {}", self.score);
        draw_text(&text, 10., 25., 25., WHITE);
    }
}
