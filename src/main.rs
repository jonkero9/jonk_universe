use jonk_utils::Jrand;
use model::star_system::StarSystem;
use raylib::consts::KeyboardKey::*;

use raylib::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

pub mod model;

#[derive(Debug)]
struct VecI {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Gamecolors {
    green: Color,
    blue: Color,
    white: Color,
    yellow: Color,
    orange: Color,
    red: Color,
}

static COLORS: Gamecolors = Gamecolors {
    green: Color {
        r: 166,
        g: 227,
        b: 161,
        a: 255,
    },
    blue: Color {
        r: 137,
        g: 180,
        b: 250,
        a: 255,
    },
    white: Color {
        r: 166,
        g: 227,
        b: 161,
        a: 255,
    },

    yellow: Color {
        r: 166,
        g: 227,
        b: 161,
        a: 255,
    },

    orange: Color {
        r: 166,
        g: 227,
        b: 161,
        a: 255,
    },
    red: Color {
        r: 166,
        g: 227,
        b: 161,
        a: 255,
    },
};

static SCREEN_W: i32 = 960;
static SCREEN_Y: i32 = 640;

fn main() {
    let mut sec_size: f32 = 16.;
    let mut jonk_random = Jrand::new();

    let (mut rl, thread) = raylib::init()
        .vsync()
        .resizable()
        .size(SCREEN_W, SCREEN_Y)
        .title("Jspace")
        .build();

    rl.set_target_fps(75);

    let mut global_pos = Vector2 { x: 0., y: 0. };

    while !rl.window_should_close() {
        let timer = Instant::now();
        let screen_w = rl.get_screen_width();
        let screen_y = rl.get_screen_height();

        sec_size = handle_zoom(&rl, sec_size);

        let n_sec_x = screen_w / sec_size as i32;
        let n_sec_y = screen_y / sec_size as i32;

        handle_key_press(
            &rl,
            &mut global_pos,
            sec_size as f32 * 8. * rl.get_frame_time(),
        );

        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        let mut star_map: HashMap<u64, StarSystem> = HashMap::new();

        for y in 0..n_sec_y {
            for x in 0..n_sec_x {
                let global_sec = VecI {
                    x: global_pos.x as i32 + x,
                    y: global_pos.y as i32 + y,
                };
                let hash_key = jonk_utils::cantor_hash(global_sec.x, global_sec.y);
                jonk_random.seed = hash_key;

                if jonk_random.rnd_range(0, 20) == 1 {
                    let star = StarSystem::new(global_sec.x, global_sec.y);
                    star_map.insert(hash_key, star);
                    let sec_to_screen = VecI {
                        x: x * sec_size as i32,
                        y: y * sec_size as i32,
                    };
                    draw.draw_circle(
                        sec_to_screen.x + (sec_size / 2.) as i32,
                        sec_to_screen.y + (sec_size / 2.) as i32,
                        (star.radius / 2000.) * (sec_size / 2.),
                        match jonk_random.rnd_range(0, 2) == 1 {
                            true => COLORS.green,
                            false => COLORS.blue,
                        },
                    );
                }
            }
        }
        handle_mouse_hover(
            star_map,
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
                "Hello, Chat!!!",
                &format!("run time seconds: {:.6}", elasped),
                &format!("Zoom: {:.2} ", sec_size),
                &format!("Sector: {}, {}", global_pos.x, global_pos.y),
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
        return sec_size - zoom_sen;
    }
    return sec_size;
}

fn handle_mouse_hover(
    star_map: HashMap<u64, StarSystem>,
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
        draw_lines(
            draw,
            vec![
                &format!("Radius: {:.2}", star.radius),
                &format!("Luminosity: {:.2} lums", star.luminosity),
                &format!("Temp: {:.2}K", star.surface_temp),
                &format!("Mass: {:.2} Solar masses", star.mass),
                &format!("Planets: {}", star.num_of_planets),
            ],
            32,
            12,
            160,
        );
    }
}

fn draw_lines(draw: &mut RaylibDrawHandle, lines: Vec<&str>, f_size: i32, s_x: i32, s_y: i32) {
    let mut start_y = s_y;
    draw.draw_rectangle(s_x, start_y, 540, f_size * lines.len() as i32, Color::BLUE);
    for s in lines {
        draw.draw_text(s, s_x, start_y, f_size, Color::WHITE);
        start_y += f_size;
    }
}

fn handle_key_press(rl: &RaylibHandle, global_pos: &mut Vector2, sensitivity: f32) {
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
