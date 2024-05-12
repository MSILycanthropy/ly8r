use std::env;

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
    let argv: Vec<_> = env::args().collect();
    if argv.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }

    let mut chip8 = Chip8::default();

    chip8.load_from_file(&argv[1]);

    let min_frame_time = 1. / 60.;
    loop {
        clear_background(BLACK);

        chip8.reset_keys();

        let keys = get_keys_down()
            .iter()
            .map(map_key)
            .filter(|o| o.is_some())
            .flatten()
            .collect::<Vec<usize>>();

        for key in keys {
            chip8.set_key_pressed(key)
        }

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

fn map_key(key: &KeyCode) -> Option<usize> {
    match key {
        KeyCode::Key1 => Some(0x1),
        KeyCode::Key2 => Some(0x2),
        KeyCode::Key3 => Some(0x3),
        KeyCode::Key4 => Some(0xC),
        KeyCode::Q => Some(0x4),
        KeyCode::W => Some(0x5),
        KeyCode::E => Some(0x6),
        KeyCode::R => Some(0xD),
        KeyCode::A => Some(0x7),
        KeyCode::S => Some(0x8),
        KeyCode::D => Some(0x9),
        KeyCode::F => Some(0xE),
        KeyCode::Z => Some(0xA),
        KeyCode::X => Some(0x0),
        KeyCode::C => Some(0xB),
        KeyCode::V => Some(0xF),
        _ => None,
    }
}
