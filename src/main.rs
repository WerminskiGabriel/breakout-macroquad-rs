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

    let total_block_size =
        settings::blocks::SIZE + vec2(settings::blocks::PADDING, settings::blocks::PADDING);

    let mut game = crate::game::Game::new();

    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * settings::blocks::COLUMNS as f32)) * 0.5f32,
        50f32,
    );

    for i in 0..settings::blocks::COLUMNS * settings::blocks::ROWS {
        let block_x = (i % settings::blocks::COLUMNS) as f32 * total_block_size.x;
        let block_y = (i / settings::blocks::COLUMNS) as f32 * total_block_size.y;
        game.blocks
            .push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    game.balls.push(Ball::new(vec2(
        screen_width() * 0.5f32,
        screen_height() * 0.5f32,
    )));

    loop {
        game.update(&font);
        game.draw(&font);
        next_frame().await
    }
}
