use super::uni_map_window::UniMapWindow;
use crate::model::star_system::StarSystem;
use crate::model::vectors::Vector2DF;
use raylib::consts::KeyboardKey::*;
use raylib::consts::MouseButton::*;
use raylib::RaylibHandle;
use std::collections::HashMap;
use std::ops::Not;

pub fn handle_uni_map_input(rl: &RaylibHandle, uni_map_window: &mut UniMapWindow) -> f32 {
    uni_map_window.sec_size = handle_zoom_unimap(rl, uni_map_window.sec_size);
    handle_key_press_unimap(
        rl,
        &mut uni_map_window.global_pos,
        (128. * rl.get_frame_time()) / (uni_map_window.sec_size / 16.),
    );
    uni_map_window.uni_map_debug_info =
        handle_debug_info_window_key(uni_map_window.uni_map_debug_info, rl);
    0.0
}

fn handle_zoom_unimap(rl: &RaylibHandle, sec_size: f32) -> f32 {
    let zoom_sen = sec_size * rl.get_frame_time();
    if rl.is_key_down(KEY_E) {
        return sec_size + zoom_sen;
    }
    if rl.is_key_down(KEY_Q) {
        return match sec_size > 6. {
            true => sec_size - zoom_sen,
            false => sec_size,
        };
    }
    sec_size
}

fn handle_key_press_unimap(rl: &RaylibHandle, global_pos: &mut Vector2DF, sensitivity: f32) {
    if rl.is_key_down(KEY_W) {
        global_pos.y -= sensitivity;
    }
    if rl.is_key_down(KEY_D) {
        global_pos.x += sensitivity;
    }
    if rl.is_key_down(KEY_S) {
        global_pos.y += sensitivity;
    }
    if rl.is_key_down(KEY_A) {
        global_pos.x -= sensitivity;
    }
}

fn handle_debug_info_window_key(debug_show_flag: bool, rl: &RaylibHandle) -> bool {
    if rl.is_key_pressed(KEY_TAB) {
        return debug_show_flag.not();
    }
    debug_show_flag
}

pub fn handle_select_star_unimap<'a>(
    rl: &RaylibHandle,
    star_map: &'a HashMap<u64, StarSystem>,
    uni_map_window: &UniMapWindow,
) -> Option<&'a StarSystem> {
    if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
        let mouse_x = rl.get_mouse_x() / uni_map_window.sec_size as i32;
        let mouse_y = rl.get_mouse_y() / uni_map_window.sec_size as i32;
        let hash = jonk_utils::cantor_hash(
            uni_map_window.global_pos.x as i32 + mouse_x,
            uni_map_window.global_pos.y as i32 + mouse_y,
        );
        return star_map.get(&hash);
    }
    None
}
