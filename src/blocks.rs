use crate::settings;
use crate::settings::blocks;
use getset::{Getters, MutGetters};
use macroquad::prelude::*;
#[derive(Getters, MutGetters)]
pub struct Block {
    #[getset(get = "pub", get_mut = "pub")]
    rect: Rect,

    #[getset(get = "pub", get_mut = "pub")]
    lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, blocks::SIZE.x, blocks::SIZE.y),
            lives: settings::blocks::INITIAL_LIVES,
        }
    }
    pub fn draw(&self) {
        let color = settings::blocks::COLOR;
        let life_alpha : f32 = self.lives as f32 / settings::blocks::INITIAL_LIVES as f32 ;
        let color: Color = Color {
            r: color.r,
            g: color.g,
            b: color.b,
            a: life_alpha as f32,
        };

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color)
    }
}
