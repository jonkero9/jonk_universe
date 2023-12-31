use jonk_utils::Jrand;
use raylib::prelude::*;

#[derive(Debug)]
struct VecI {
    x: i32,
    y: i32,
}

fn main() {
    let mut jonk_random = Jrand { seed: 0 };
    const SCREEN_W: i32 = 960;
    const SCREEN_Y: i32 = 640;
    const SEC_SIZE: i32 = 16;

    let (mut rl, thread) = raylib::init()
        .vsync()
        .size(SCREEN_W, SCREEN_Y)
        .title("Jspace")
        .build();

    rl.set_target_fps(75);

    let global_pos = VecI { x: 0, y: 0 };

    while !rl.window_should_close() {
        let screen_w = rl.get_screen_width();
        let screen_y = rl.get_screen_height();

        let n_sec_x = screen_w / SEC_SIZE;
        let n_sec_y = screen_y / SEC_SIZE;

        let mut draw = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);

        draw.draw_text("Hello, Chat!!!", 12, 12, 32, Color::WHITE);

        for y in 0..n_sec_y {
            for x in 0..n_sec_x {
                let global_sec = VecI {
                    x: global_pos.x + x,
                    y: global_pos.y + y,
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
                        Color::GREEN,
                    );
                }
            }
        }
    }
}