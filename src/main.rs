mod block;
mod field;
mod rendering;
mod stone;

extern crate sdl2;

use field::{
    clear_full_rows, create_new_random_block, is_block_colliding, save_block_to_field, BLOCK_SIZE,
    FIELD_HEIGHT, FIELD_PADDING_X, FIELD_PADDING_Y, FIELD_WIDTH,
};
use rendering::{draw_blocks, draw_grid};

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "Tetris",
            2 * FIELD_PADDING_X + BLOCK_SIZE * FIELD_WIDTH,
            2 * FIELD_PADDING_Y + BLOCK_SIZE * FIELD_HEIGHT,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut field = [0; (FIELD_WIDTH * FIELD_HEIGHT) as usize];
    for x in 0..FIELD_WIDTH as usize {
        field[x * FIELD_HEIGHT as usize] = 1;
    }

    for y in 0..FIELD_HEIGHT as usize {
        field[y] = 1;
        field[((FIELD_WIDTH - 1) * FIELD_HEIGHT) as usize + y] = 1
    }

    let mut active_block = create_new_random_block();

    let mut cleared_rows: u32 = 0;

    let mut last_frame = Instant::now();
    let mut since_last_tick = Duration::ZERO;
    const TICK: Duration = Duration::from_millis(250);

    'running: loop {
        let now = Instant::now();
        let frame_time = now - last_frame;
        last_frame = now;
        since_last_tick += frame_time;

        if since_last_tick >= TICK {
            since_last_tick = Duration::ZERO; // Do not substract TICK to avoid multiple ticks when lag spikes would occur
            active_block.y -= 1;

            if is_block_colliding(&active_block, &field) {
                active_block.y += 1; // Reset movement
                save_block_to_field(&active_block, &mut field);
                cleared_rows += clear_full_rows(&mut field);
                active_block = create_new_random_block();

                if is_block_colliding(&active_block, &field) {
                    println!("Game Over, cleared {} rows!", cleared_rows);
                    return;
                }
            }
        }

        // Draw
        canvas.set_draw_color(Color::RGB(100, 64, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    active_block.x -= 1;
                    if is_block_colliding(&active_block, &field) {
                        active_block.x += 1;
                    } else {
                        since_last_tick = Duration::ZERO;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    active_block.x += 1;
                    if is_block_colliding(&active_block, &field) {
                        active_block.x -= 1;
                    } else {
                        since_last_tick = Duration::ZERO;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    since_last_tick = TICK;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    // TODO ROTATE
                    for block in active_block.stones.iter_mut() {
                        block.local_x *= -1;
                        let tmp = block.local_y;
                        block.local_y = block.local_x;
                        block.local_x = tmp;
                    }
                    if is_block_colliding(&active_block, &field) {
                        for block in active_block.stones.iter_mut() {
                            let tmp = block.local_x;
                            block.local_x = -block.local_y;
                            block.local_y = tmp;
                        }
                    }
                }
                _ => {}
            }
        }

        draw_blocks(&mut canvas, &field, &active_block);
        draw_grid(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
