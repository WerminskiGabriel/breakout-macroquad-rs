mod balls;
mod blocks;
mod game;
mod logic;
mod player;
mod settings;

use balls::*;
use blocks::*;
use game::*;
use logic::*;
use player::*;
use settings::*;

use macroquad::prelude::*;

pub enum GameState {
    Menu,
    Playing,
    Completed,
    Dead,
}

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("media/Minecraft.ttf").await.unwrap();
    let mut game = crate::game::Game::new();

    loop {
        clear_background(settings::ui::BACKGROUND_COLOR);
        game.update(&font);
        game.draw(&font);

        next_frame().await
    }
}
