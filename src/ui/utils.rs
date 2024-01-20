use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::{game_color::COLORS, model::vectors::Vector2DI};

pub fn draw_lines(
    draw: &mut RaylibDrawHandle,
    lines: Vec<(String, Color)>,
    f_size: i32,
    start_pos: Vector2DI,
) {
    draw.draw_rectangle(
        start_pos.x,
        start_pos.y,
        540,
        32 + f_size * lines.len() as i32,
        COLORS.bg,
    );

    let mut start_y = start_pos.y + 16;
    let start_x = start_pos.x + 16;
    for s in lines {
        draw.draw_text(&s.0, start_x, start_y, f_size, s.1);
        start_y += f_size;
    }
}
