mod config;
mod game;
mod sprite;
mod util;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("Segio")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        game.input_handler();
        if game.draw() {
            game = Game::new().await;
        }

        next_frame().await
    }
}
