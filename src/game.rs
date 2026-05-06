use crate::GameState;
use crate::Player;
use crate::blocks::Block;
use crate::logic::{draw_title_text, resolve_collision};
use crate::settings::player;
use crate::{Ball, settings};
use macroquad::prelude::*;
use std::fmt::format;

pub struct Game {
    pub(crate) state: GameState,
    pub(crate) player: Player,
    pub(crate) blocks: Vec<Block>,
    pub(crate) balls: Vec<Ball>,
    pub(crate) score: i32,
    pub(crate) best_score: i32,
    pub(crate) lives: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            player: Player::new(),
            score: 0,
            best_score: 0,
            lives: player::INITIAL_LIVES,
            blocks: Vec::new(),
            balls: Vec::new(),
        }
    }
    pub fn reset(&mut self) {
        if self.best_score < self.score {
            self.best_score = self.score
        }
        self.score = 0;
        self.blocks = Vec::new();
        self.balls = Vec::new();
        self.lives = player::INITIAL_LIVES;

        let total_block_size =
            settings::blocks::SIZE + vec2(settings::blocks::PADDING, settings::blocks::PADDING);
        let board_start_pos = vec2(
            (screen_width() - (total_block_size.x * settings::blocks::COLUMNS as f32)) * 0.5f32,
            50f32,
        );
        for i in 0..settings::blocks::COLUMNS * settings::blocks::ROWS {
            let block_x = (i % settings::blocks::COLUMNS) as f32 * total_block_size.x;
            let block_y = (i / settings::blocks::COLUMNS) as f32 * total_block_size.y;
            self.blocks
                .push(Block::new(board_start_pos + vec2(block_x, block_y)));
        }
        self.balls.push(Ball::new(vec2(
            0.5f32 * screen_width(),
            0.6f32 * screen_height(),
        )));
    }

    pub fn update(&mut self, font: &Font) {
        match self.state {
            GameState::Playing => {
                // remove player lives and balls outside screen
                let balls_len = self.balls.len();
                self.balls.retain(|ball| ball.rect().y < screen_height());
                let removed_balls = balls_len - self.balls.len();
                if removed_balls > 0 {
                    self.balls.push(Ball::new(vec2(
                        screen_width() * 0.5f32,
                        screen_height() * 0.5f32,
                    )));
                    self.lives -= removed_balls as i32;
                    if self.lives == 0 {
                        self.state = GameState::Dead;
                    }
                }
                if self.balls.is_empty() {
                    self.state = GameState::Completed;
                }

                // remove blocks with 0 lives
                self.blocks.retain(|block| *block.lives() > 0);

                // update balls
                for ball in self.balls.iter_mut() {
                    ball.update(get_frame_time());
                }
                self.player.update(get_frame_time());

                // spawning balls
                if is_key_pressed(KeyCode::Space) {
                    self.balls.push(Ball::new(vec2(
                        screen_width() * 0.5f32,
                        screen_height() * 0.5f32,
                    )));
                }

                for ball in self.balls.iter_mut() {
                    resolve_collision(ball, self.player.rect());

                    for block in self.blocks.iter_mut() {
                        if resolve_collision(ball, block.rect()) {
                            *block.lives_mut() -= 1;
                            if *block.lives() <= 0 {
                                self.score += 10
                            } else {
                                self.score += 1
                            }
                        }
                    }
                }
            }
            GameState::Dead => {}
            GameState::Completed => {}
            GameState::Menu => {}
        }
    }

    pub fn draw(&mut self, font: &Font) {
        match self.state {
            GameState::Playing => {
                self.player.draw();

                for ball in self.balls.iter() {
                    ball.draw();
                }

                for block in self.blocks.iter() {
                    block.draw();
                }

                let score_text = &format!("score: {}", self.score);
                let score_text_dim = measure_text(&score_text, Some(&font), 30u16, 1f32);

                draw_text_ex(
                    &score_text,
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams {
                        font: Some(&font),
                        font_size: 30,
                        color: settings::ui::TEXT_COLOR,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    &format!("lives: {}", self.lives),
                    settings::blocks::PADDING,
                    40.0,
                    TextParams {
                        font: Some(&font),
                        font_size: 30,
                        color: settings::ui::TEXT_COLOR,
                        ..Default::default()
                    },
                );
            }
            GameState::Dead => {
                let message = if self.best_score == 0 {
                    &format!("You are dead! Score: {}!", self.score)
                } else {
                    &format!(
                        "You are dead! Score: {}! Best score: {}", self.score,
                        self.best_score.to_string())
                };

                draw_title_text(message,
                                &font,
                );
                if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
                    self.reset();
                }
            }
            GameState::Completed => {
                let message = if self.best_score == 0 {
                    &format!("You are dead! Score: {}!", self.score)
                } else {
                    &format!(
                        "You won! Score: {}! Best score: {}", self.score,
                        self.best_score.to_string())
                };
                if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
                    self.reset();
                }
            }
            GameState::Menu => {
                draw_title_text("CLICK SPACE TO PLAY", &font);
                if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
                    self.reset();
                }
            }
        }
    }
}
