use ly8r::{clock::Clock, Chip8, SCREEN_HEIGHT, SCREEN_WIDTH};

const SCALE: i32 = 15;
const WINDOW_WIDTH: i32 = (SCREEN_WIDTH as i32) * SCALE;
const WINDOW_HEIGHT: i32 = (SCREEN_HEIGHT as i32) * SCALE;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip-8".to_owned(),
        window_height: WINDOW_HEIGHT,
        window_width: WINDOW_WIDTH,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut chip8 = Chip8::default();

    chip8.load_from_file("1-chip8-logo.ch8");

    println!("{:x}", chip8.ram[0x0200]);

    let min_frame_time = 1. / 60.;
    loop {
        // dbg!(get_fps());

        clear_background(BLACK);

        chip8.clock(10);

        let screen_buf = chip8.display;

        for (i, pixel) in screen_buf.iter().enumerate() {
            if *pixel {
                let x = (i % SCREEN_WIDTH) as i32;
                let y = (i / SCREEN_WIDTH) as i32;

                draw_rectangle(
                    (x * SCALE) as f32,
                    (y * SCALE) as f32,
                    SCALE as f32,
                    SCALE as f32,
                    WHITE,
                );
            }
        }

        let current_frame_time = get_frame_time();

        next_frame().await;

        if current_frame_time < min_frame_time {
            let time_to_sleep = (min_frame_time - current_frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
    }
}

fn limit_fps_to(framerate: f32) {}
