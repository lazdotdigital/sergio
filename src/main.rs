mod config;
mod game;
mod sprite;
mod util;

use game::Game;
use macroquad::prelude::*;

#[macroquad::main("Learning")]
async fn main() {
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        game.input_handler();
        game.draw();

        next_frame().await
    }
}
