use std::ffi::CString;

use crate::{model::vectors::Vector2DF, FONT_SIZE};
use raylib::{drawing::RaylibDrawHandle, ffi::Rectangle, rgui::RaylibDrawGui, text::measure_text};

const MARGIN_SIZE: f32 = 16.;

pub fn draw_lines(draw: &mut RaylibDrawHandle, lines: Vec<String>, start_pos: Vector2DF) {
    let joined = lines.join("\n");
    let width = measure_text(&joined, FONT_SIZE);

    let bounds = Rectangle {
        x: start_pos.x,
        y: start_pos.y,
        width: width as f32,
        height: lines.len() as f32 * FONT_SIZE as f32 * 2.,
    };

    draw.gui_panel(bounds);

    draw.gui_label(
        Rectangle {
            x: start_pos.x + MARGIN_SIZE,
            y: start_pos.y + MARGIN_SIZE * 2.,
            width: 1.,
            height: 1.,
        },
        Some(&CString::new(joined).expect("")),
    );
}
