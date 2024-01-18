use std::{collections::HashMap, time::Instant};

use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::model::{star_system::StarSystem, vectors::Vector2DI};

use super::uni_map_window::UniMapWindow;
use super::utils;

impl UniMapWindow {
    pub fn draw(
        &self,
        star_map: &HashMap<u64, StarSystem>,
        timer: Instant,
        draw: &mut RaylibDrawHandle,
    ) {
        handle_uni_map_draw(&star_map, draw, &self);
        draw_uni_debug_widget(timer, &self, draw);
        handle_mouse_hover(&star_map, draw, &self);
    }
}

fn handle_uni_map_draw(
    star_map: &HashMap<u64, StarSystem>,
    draw: &mut RaylibDrawHandle,
    uni_map_window: &UniMapWindow,
) {
    for y in 0..uni_map_window.n_sectors.y {
        for x in 0..uni_map_window.n_sectors.x {
            let global_sec = Vector2DI {
                x: uni_map_window.global_pos.x as i32 + x,
                y: uni_map_window.global_pos.y as i32 + y,
            };
            let hash_key = jonk_utils::cantor_hash(global_sec.x, global_sec.y);
            if let Some(star) = star_map.get(&hash_key) {
                let sec_to_screen = Vector2DI {
                    x: x * uni_map_window.sec_size as i32,
                    y: y * uni_map_window.sec_size as i32,
                };
                draw.draw_circle(
                    sec_to_screen.x + (uni_map_window.sec_size / 2.) as i32,
                    sec_to_screen.y + (uni_map_window.sec_size / 2.) as i32,
                    (star.radius / 2000.) * (uni_map_window.sec_size / 2.),
                    Color::from(star.star_color),
                );
            }
        }
    }
}

fn handle_mouse_hover(
    star_map: &HashMap<u64, StarSystem>,
    draw: &mut RaylibDrawHandle,
    uni_map_window: &UniMapWindow,
) {
    if let Some(star) = star_map.get(&jonk_utils::cantor_hash(
        uni_map_window.global_pos.x as i32 + (draw.get_mouse_x() / uni_map_window.sec_size as i32),
        uni_map_window.global_pos.y as i32 + (draw.get_mouse_y() / uni_map_window.sec_size as i32),
    )) {
        draw_debug_star_menu(star, draw);
    }
}

fn draw_debug_star_menu(star: &StarSystem, draw: &mut RaylibDrawHandle) {
    let lines = vec![
        format!("Radius: {:.2}", star.radius),
        format!("Luminosity: {:.2} lums", star.luminosity),
        format!("Temp: {:.2}K", star.surface_temp),
        format!("Mass: {:.2} Solar masses", star.mass),
        format!("Planets: {}", star.planets.len()),
        format!("Color: {:?}", star.star_color),
        format!("location: {}, {}", star.location.x, star.location.y),
    ];
    let planet_lines: Vec<String> = star
        .planets
        .iter()
        .map(|p| format!(" planet : {}", p.mass))
        .collect();
    let collect: Vec<&String> = lines.iter().chain(planet_lines.iter()).collect();
    utils::draw_lines(draw, collect, 32, 12, 160);
}

fn draw_uni_debug_widget(timer: Instant, uni_map_win: &UniMapWindow, draw: &mut RaylibDrawHandle) {
    let elasped = timer.elapsed().as_secs_f64();
    if uni_map_win.uni_map_debug_info {
        utils::draw_lines(
            draw,
            vec![
                &format!(
                    "nsecs: {}, {}",
                    uni_map_win.n_sectors.x, uni_map_win.n_sectors.y
                ),
                &format!("run time seconds: {:.6}", elasped),
                &format!("Zoom: {:.2} ", uni_map_win.sec_size),
                &format!(
                    "Sector: {}, {}",
                    uni_map_win.global_pos.x, uni_map_win.global_pos.y
                ),
            ],
            32,
            12,
            12,
        );
    }
}
