use jonk_utils::Jrand;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::time::Instant;

#[derive(Debug)]
struct VecI {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Gamecolors {
    green: Color,
}

static COLORS: Gamecolors = Gamecolors {
    green: Color {
        r: 166,
        g: 227,
        b: 161,
        a: 255,
    },
};

static SCREEN_W: i32 = 960;
static SCREEN_Y: i32 = 640;
static SEC_SIZE: i32 = 16;

fn main() {
    let mut jonk_random = Jrand { seed: 0 };

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

        let n_sec_x = screen_w / SEC_SIZE;
        let n_sec_y = screen_y / SEC_SIZE;

        handle_key_press(
            &rl,
            &mut global_pos,
            SEC_SIZE as f32 * 8. * rl.get_frame_time(),
        );

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        for y in 0..n_sec_y {
            for x in 0..n_sec_x {
                let global_sec = VecI {
                    x: global_pos.x as i32 + x,
                    y: global_pos.y as i32 + y,
                };
                jonk_random.seed = jonk_utils::cantor_hash(global_sec.x, global_sec.y);
                if jonk_random.rnd_range(0, 20) == 1 {
                    let sec_to_screen = VecI {
                        x: x * SEC_SIZE,
                        y: y * SEC_SIZE,
                    };
                    draw.draw_circle(
                        sec_to_screen.x + (SEC_SIZE / 2),
                        sec_to_screen.y + (SEC_SIZE / 2),
                        6.,
                        COLORS.green,
                    );
                }
            }
        }
        draw.draw_text("Hello, Chat!!!", 12, 12, 32, Color::WHITE);
        let elasped = timer.elapsed().as_secs_f64();
        draw.draw_text(
            &format!("run time seconds: {:.6} ", elasped),
            12,
            44,
            32,
            Color::WHITE,
        );
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
