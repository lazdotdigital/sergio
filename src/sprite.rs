use macroquad::prelude::*;

pub struct Sprite {
    texture: Texture2D,
    frame_width: f32,
    frame_height: f32,
    frame_number: usize,
    frame_count: usize,
    frame_gap: f64,
    prev_frame_change_time: f64,
}

impl Sprite {
    pub async fn new(
        path: &str,
        frame_width: f32,
        frame_height: f32,
        frame_gap: usize,
        frame_count: usize,
    ) -> Self {
        let texture = load_texture(path).await.unwrap();
        Self {
            texture,
            frame_width,
            frame_height,
            frame_number: 1,
            frame_count,
            frame_gap: get_frame_time() as f64 * frame_gap as f64,
            prev_frame_change_time: get_time(),
        }
    }
}

impl Sprite {
    pub fn draw(&mut self, x: f32, y: f32, flip_x: bool, flip_y: bool) -> bool {
        if self.can_increment_frame_number() {
            self.increment_frame_number();
        }

        draw_texture_ex(self.texture, x, y, WHITE, self.draw_params(flip_x, flip_y));

        self.frame_number == self.frame_count - 1
    }

    fn can_increment_frame_number(&self) -> bool {
        get_time() - self.prev_frame_change_time >= self.frame_gap
    }

    fn increment_frame_number(&mut self) {
        if self.frame_number < self.frame_count - 1 {
            self.frame_number += 1;
        } else {
            self.frame_number = 0;
        }
        self.reset_prev_frame_change_time();
    }

    fn reset_prev_frame_change_time(&mut self) {
        self.prev_frame_change_time = get_time();
    }

    fn draw_params(&self, flip_x: bool, flip_y: bool) -> DrawTextureParams {
        let x = self.frame_width * self.frame_number as f32;
        DrawTextureParams {
            dest_size: None,
            source: Some(Rect::new(x, 0., self.frame_width, self.frame_height)),
            rotation: 0.,
            flip_x,
            flip_y,
            pivot: None,
        }
    }
}
