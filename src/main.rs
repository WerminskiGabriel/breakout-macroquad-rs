mod balls;
mod blocks;
mod game;
mod logic;
mod perlinNoise;
mod perlinTexture;
mod player;
mod settings;

use balls::*;
use blocks::*;
use game::*;
use logic::*;
use perlinTexture::*;
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
        let background_texture = perlin_texture(get_time() as f32);
        draw_texture(&background_texture, 0.0, 0.0, WHITE);
        game.update(&font);
        game.draw(&font);

        next_frame().await
    }
}
