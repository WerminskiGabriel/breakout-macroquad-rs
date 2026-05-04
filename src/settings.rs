use macroquad::math::Vec2;

pub mod blocks {
    use super::*;
    pub const SIZE: Vec2 = Vec2::new(100f32, 40f32);
    pub const COLUMNS: usize = 6;
    pub const ROWS: usize = 6;
    pub const PADDING: f32 = 10f32;
}

pub mod player {
    use super::*;
    pub const SIZE: Vec2 = Vec2::new(150f32, 40f32);
    pub const SPEED: f32 = 500f32;
    pub const INITIAL_LIVES: i32 = 10i32;
}
pub mod balls {
    use super::*;
    pub const SIZE: Vec2 = Vec2::new(50f32, 50f32);
    pub const SPEED: f32 = 400f32;
}

pub mod ui {
    pub const FONT_SIZE: u16 = 30u16;
}
