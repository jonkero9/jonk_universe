use game_color::COLORS;
use model::star_system::StarSystem;
use raylib::consts::KeyboardKey::*;

use raylib::prelude::*;
use std::collections::HashMap;
use std::time::Instant;
use u_gen::factory;

pub mod game_color;
pub mod model;
pub mod u_gen;

#[derive(Debug, Clone, Copy)]
pub struct VecI {
    pub x: i32,
    pub y: i32,
}

fn main() {
    let mut sec_size: f32 = 16.;
    let mut global_pos = Vector2 { x: 0., y: 0. };

    let star_map: HashMap<u64, StarSystem> = factory::new_universe(1000);

    let (mut rl, thread) = raylib::init()
        .vsync()
        .resizable()
        .size(960, 640)
        .title("Jspace")
        .build();
    rl.set_target_fps(75);

    while !rl.window_should_close() {
        let timer = Instant::now();
        sec_size = handle_zoom(&rl, sec_size);

        handle_key_press(
            &rl,
            &mut global_pos,
            (128. * rl.get_frame_time()) / (sec_size / 16.),
        );

        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
        let n_sec_x = rl.get_screen_width() / sec_size as i32;
        let n_sec_y = rl.get_screen_height() / sec_size as i32;

        // Begin Draw
        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        for y in 0..n_sec_y {
            for x in 0..n_sec_x {
                let global_sec = VecI {
                    x: global_pos.x as i32 + x,
                    y: global_pos.y as i32 + y,
                };
                let hash_key = jonk_utils::cantor_hash(global_sec.x, global_sec.y);
                if let Some(star) = star_map.get(&hash_key) {
                    let sec_to_screen = VecI {
                        x: x * sec_size as i32,
                        y: y * sec_size as i32,
                    };
                    draw.draw_circle(
                        sec_to_screen.x + (sec_size / 2.) as i32,
                        sec_to_screen.y + (sec_size / 2.) as i32,
                        (star.radius / 2000.) * (sec_size / 2.),
                        Color::from(star.star_color),
                    );
                }
            }
        }
        handle_mouse_hover(
            &star_map,
            &mut global_pos,
            &mut draw,
            mouse_x,
            mouse_y,
            sec_size,
        );
        let elasped = timer.elapsed().as_secs_f64();
        draw_lines(
            &mut draw,
            vec![
                format!("nsecs: {}, {}", n_sec_x, n_sec_y),
                format!("run time seconds: {:.6}", elasped),
                format!("Zoom: {:.2} ", sec_size),
                format!("Sector: {}, {}", global_pos.x, global_pos.y),
            ],
            32,
            12,
            12,
        );
    }
}

fn handle_zoom(rl: &RaylibHandle, sec_size: f32) -> f32 {
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
    return sec_size;
}

fn handle_mouse_hover(
    star_map: &HashMap<u64, StarSystem>,
    global_pos: &mut Vector2,
    draw: &mut RaylibDrawHandle,
    mouse_x: i32,
    mouse_y: i32,
    sec_size: f32,
) {
    if let Some(star) = star_map.get(&jonk_utils::cantor_hash(
        global_pos.x as i32 + (mouse_x / sec_size as i32),
        global_pos.y as i32 + mouse_y / sec_size as i32,
    )) {
        let mut lines = vec![
            format!("Radius: {:.2}", star.radius),
            format!("Luminosity: {:.2} lums", star.luminosity),
            format!("Temp: {:.2}K", star.surface_temp),
            format!("Mass: {:.2} Solar masses", star.mass),
            format!("Planets: {}", star.planets.len()),
            format!("Color: {:?}", star.star_color),
            format!("location: {}, {}", star.location.x, star.location.y),
        ];
        for (i, pat) in star.planets.iter().enumerate() {
            lines.push(format!("Planet: {} , Mass: {}", i, pat.mass));
        }
        draw_lines(draw, lines, 32, 12, 160);
    }
}

fn draw_lines(draw: &mut RaylibDrawHandle, lines: Vec<String>, f_size: i32, s_x: i32, s_y: i32) {
    let mut start_y = s_y;
    draw.draw_rectangle(s_x, start_y, 540, f_size * lines.len() as i32, COLORS.bg);
    for s in lines {
        draw.draw_text(&s, s_x, start_y, f_size, Color::WHITE);
        start_y += f_size;
    }
}

fn handle_key_press(rl: &RaylibHandle, global_pos: &mut Vector2, sensitivity: f32) {
    if rl.is_key_down(KEY_K) {
        global_pos.y -= sensitivity;
    }
    if rl.is_key_down(KEY_L) {
        global_pos.x += sensitivity;
    }
    if rl.is_key_down(KEY_J) {
        global_pos.y += sensitivity;
    }
    if rl.is_key_down(KEY_H) {
        global_pos.x -= sensitivity;
    }
}
