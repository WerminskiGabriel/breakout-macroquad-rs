use crate::settings;
use crate::settings::balls;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;

#[derive(Getters, MutGetters)]
pub struct Ball {
    #[getset(get = "pub", get_mut = "pub")]
    rect: Rect,

    #[getset(get = "pub", get_mut = "pub")]
    vel: Vec2,
}
impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, balls::SIZE.x, balls::SIZE.y),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * balls::SPEED;
        self.rect.y += self.vel.y * dt * balls::SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32
        } else if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }
    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            settings::balls::COLOR,
        )
    }
}
