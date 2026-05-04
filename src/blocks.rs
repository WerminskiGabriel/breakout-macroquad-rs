use getset::{Getters, MutGetters};
use macroquad::prelude::*;
use crate::settings::blocks;
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
            lives: 2,
        }
    }
    pub fn draw(&self) {
        let color = match self.lives {
            1 => RED,
            _ => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color)
    }
}
