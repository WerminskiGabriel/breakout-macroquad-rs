use crate::{settings, Ball};
use crate::GameState;
use crate::Player;
use crate::blocks::Block;
use crate::settings::player;
use macroquad::color::{BLACK, GRAY};
use macroquad::input::{KeyCode, is_key_pressed};
use macroquad::math::vec2;
use macroquad::prelude::{clear_background, draw_text_ex, get_frame_time, measure_text, screen_height, screen_width, Font, TextParams};
use crate::logic::{draw_title_text, resolve_collision};

pub struct Game {
    pub(crate) state: GameState,
    pub(crate) player: Player,
    pub(crate) blocks: Vec<Block>,
    pub(crate) balls: Vec<Ball>,
    pub(crate) score: i32,
    pub(crate) lives: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Menu,
            player: Player::new(),
            score: 0,
            lives: player::INITIAL_LIVES,
            blocks: Vec::new(),
            balls: Vec::new(),
        }
    }

    pub fn update(&mut self, font : &Font) {
        match self.state {
            GameState::Playing => {

                let balls_len = self.balls.len();
                self.balls.retain(|ball| ball.rect().y < screen_height());
                let removed_balls = balls_len - self.balls.len();
                if removed_balls > 0 {
                    self.lives -= removed_balls as i32;
                }
                self.blocks.retain(|block| *block.lives() > 0);

                for ball in self.balls.iter_mut() {
                    ball.update(get_frame_time());
                }


                if is_key_pressed(KeyCode::Space) {
                    self.balls.push(Ball::new(vec2(
                        screen_width() * 0.5f32,
                        screen_height() * 0.5f32,
                    )));
                }
                self.player.update(get_frame_time());

                clear_background(GRAY);

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
            GameState::Dead => {

            }
            GameState::Completed => {

            }
            GameState::Menu => {

            }
        }
    }

    pub fn draw(&mut self, font : &Font) {

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
                        color: BLACK,
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
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::Dead => {
                draw_title_text(&format!("You are dead! Score: {}", self.score), &font);
            }
            GameState::Completed => {
                draw_title_text(&format!("You won! Score: {}", self.score), &font);
            }
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    self.state = GameState::Playing;
                }
                draw_title_text("CLICK SPACE TO PLAY", &font);
            }

        }
    }
}
