use macroquad::color::Color;
use macroquad::math::Vec2;

const COLOR_RED: Color = Color {
    r: 219f32 / 255.0,
    g: 61f32 / 255.0,
    b: 62f32 / 255.0,
    a: 1f32,
};
const COLOR_BLACK: Color = Color {
    r: 63f32 / 255.0,
    g: 63f32 / 255.0,
    b: 63f32 / 255.0,
    a: 1f32,
};
const COLOR_BEIGE: Color = Color {
    r: 230f32 / 255.0,
    g: 223f32 / 255.0,
    b: 194f32 / 255.0,
    a: 1f32,
};

pub mod blocks {
    use super::*;
    pub const SIZE: Vec2 = Vec2::new(100f32, 40f32);
    pub const COLUMNS: usize = 6;
    pub const ROWS: usize = 6;
    pub const PADDING: f32 = 10f32;
    pub const COLOR: Color = COLOR_BLACK;
    pub const INITIAL_LIVES: i32 = 4;
}

pub mod player {
    use super::*;
    pub const SIZE: Vec2 = Vec2::new(150f32, 40f32);
    pub const SPEED: f32 = 500f32;
    pub const INITIAL_LIVES: i32 = 3;
    pub const COLOR: Color = COLOR_BLACK;
}
pub mod balls {
    use super::*;
    pub const SIZE: Vec2 = Vec2::new(50f32, 50f32);
    pub const SPEED: f32 = 400f32;
    pub const COLOR: Color = COLOR_RED;
}

pub mod ui {
    use super::*;

    pub const FONT_SIZE: u16 = 30u16;
    pub const TEXT_COLOR: Color = COLOR_RED;
    pub const BACKGROUND_COLOR: Color = COLOR_BEIGE;
}
