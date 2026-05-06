use crate::balls::Ball;
use crate::settings;
use macroquad::prelude::*;

pub fn resolve_collision(ball_a: &mut Ball, b: &Rect) -> bool {
    let intersection = match ball_a.rect().intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };

    ball_a.vel_mut().y *= -1f32;
    let a_center = ball_a.rect().center();
    let b_center = b.center();
    let to = b_center - a_center;
    let to_signum = to.signum();

    match intersection.w > intersection.h {
        true => {
            ball_a.rect_mut().y -= to_signum.y * intersection.h;
            match to_signum.y > 0f32 {
                true => ball_a.vel_mut().y = -ball_a.vel().y.abs(),
                false => ball_a.vel_mut().y = ball_a.vel().y.abs(),
            }
        }
        false => {
            ball_a.rect_mut().x -= to_signum.x * intersection.w;
            match to_signum.x < 0f32 {
                true => ball_a.vel_mut().x = ball_a.vel().x.abs(),
                false => ball_a.vel_mut().x = -ball_a.vel().x.abs(),
            }
        }
    }
    true
}

pub fn draw_title_text(text: &str, font: &Font) {
    let font_size = 40u16;
    let dims = measure_text(&text, Some(&font), font_size, 1f32);

    draw_text_ex(
        &text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32,
        TextParams {
            font: Some(&font),
            font_size,
            color: settings::ui::TEXT_COLOR,
            ..Default::default()
        },
    );
}
