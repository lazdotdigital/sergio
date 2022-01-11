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
    ready_fire: bool,
    game_over_timer: f32,
    game_over: bool,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            player: Player::new(player_sprites().await),
            targets: vec![Target::new()],
            bullet: None,
            last_target_time: get_time(),
            last_move_key_pressed: KeyCode::D,
            ready_fire: true,
            game_over_timer: 5.,
            game_over: false,
        }
    }

    pub fn input_handler(&mut self) {
        self.player.input_handler(&mut self.last_move_key_pressed);
        self.space_down_handler();
        self.space_up_handler();
    }

    fn space_down_handler(&mut self) {
        if is_key_down(KeyCode::Space) {
            if self.ready_fire {
                let y = self.player.y() + 12.5;
                self.bullet = Some(Bullet::new(self.player.x(), y, self.last_move_key_pressed));
                self.player.start_attacking();
                self.ready_fire = false;
            }
        }
    }

    fn space_up_handler(&mut self) {
        if is_key_released(KeyCode::Space) {
            self.ready_fire = true;
        }
    }

    pub fn draw(&mut self) -> bool {
        if self.game_over {
            self.draw_game_over();
        } else {
            self.draw_game();
        }
        self.should_reset_game()
    }

    fn draw_game_over(&mut self) {
        draw_text(
            "Game over! Press space to restart.",
            30.,
            screen_height() / 2.,
            24.,
            WHITE,
        );
    }

    fn draw_game(&mut self) {
        if self.should_add_target() {
            self.add_target();
        }

        self.draw_bullet();
        self.draw_targets();
        self.player.draw(self.last_move_key_pressed);
        self.handle_hit_target();
        self.draw_game_over_timer();
        self.decrease_game_over_timer();
        self.handle_game_over();
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
                self.game_over_timer += 1.;
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

    fn draw_game_over_timer(&self) {
        let text = format!("Game over in: {}", self.game_over_timer);
        draw_text(&text, 10., 25., 25., WHITE);
    }

    fn decrease_game_over_timer(&mut self) {
        self.game_over_timer -= get_frame_time();
    }

    fn handle_game_over(&mut self) {
        if self.game_over_timer <= 0. {
            self.game_over = true;
        }
    }

    fn should_reset_game(&self) -> bool {
        self.game_over && is_key_down(KeyCode::Space)
    }
}

async fn player_sprites() -> player::Sprites {
    player::Sprites {
        idle: Sprite::new("assets/aku.png", 59., 30., 6, 1).await,
        attack: Sprite::new("assets/aku.png", 59., 30., 2, 10).await,
    }
}
