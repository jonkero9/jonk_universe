use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::game_color::COLORS;

pub fn draw_lines(
    draw: &mut RaylibDrawHandle,
    lines: Vec<(String, Color)>,
    f_size: i32,
    s_x: i32,
    s_y: i32,
) {
    let mut start_y = s_y;
    draw.draw_rectangle(s_x, start_y, 540, f_size * lines.len() as i32, COLORS.bg);
    for s in lines {
        draw.draw_text(&s.0, s_x, start_y, f_size, s.1);
        start_y += f_size;
    }
}
