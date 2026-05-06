use crate::settings::player;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;
use crate::settings;

#[derive(Getters, MutGetters)]
pub struct Player {
    #[getset( get="pub", get_mut="pub")]
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - player::SIZE.x * 0.5f32,
                screen_height() - 100f32,
                player::SIZE.x,
                player::SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::A), is_key_down(KeyCode::D)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };
        self.rect.x += x_move * dt * player::SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, settings::player::COLOR)
    }
}
