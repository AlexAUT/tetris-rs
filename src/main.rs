use std::time::{Duration, Instant};
use rand::Rng;

use rand::distributions::Standard;
use rand::prelude::Distribution;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

extern crate sdl2;

const FIELD_WIDTH: u32 = 12;
const FIELD_HEIGHT: u32 = 23;
const BLOCK_SIZE: u32 = 30;
const FIELD_PADDING_X: u32 = 30;
const FIELD_PADDING_Y: u32 = 20;

struct Stone {
    local_x: i32,
    local_y: i32,
}

struct Block {
    x: i32,
    y: i32,
    stones: [Stone; 4],
}

enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Distribution<Shape> for Standard
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Shape {
        match rng.gen_range(0..=6) {
            0 => Shape::I,
            1 => Shape::J,
            2 => Shape::L,
            3 => Shape::O,
            4 => Shape::S,
            5 => Shape::T,
            _ => Shape::Z,
        }
    }
}

fn create_stones(shape: Shape) -> [Stone; 4] {
    match shape {
        Shape::I => [
            Stone {
                local_x: -2,
                local_y: 0,
            },
            Stone {
                local_x: -1,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 0,
            },
        ],
        Shape::J => [
            Stone {
                local_x: -1,
                local_y: 1,
            },
            Stone {
                local_x: -1,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 0,
            },
        ],
        Shape::L => [
            Stone {
                local_x: -1,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 1,
            },
        ],
        Shape::O => [
            Stone {
                local_x: -1,
                local_y: -1,
            },
            Stone {
                local_x: -1,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 1,
            },
        ],
        Shape::S => [
            Stone {
                local_x: -1,
                local_y: -1,
            },
            Stone {
                local_x: 0,
                local_y: -1,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 0,
            },
        ],
        Shape::T => [
            Stone {
                local_x: -1,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 0,
            },
            Stone {
                local_x: 0,
                local_y: 1,
            },
        ],
        Shape::Z => [
            Stone {
                local_x: -1,
                local_y: 1,
            },
            Stone {
                local_x: 0,
                local_y: 1,
            },
            Stone {
                local_x: 0,
                local_y: 0,
            },
            Stone {
                local_x: 1,
                local_y: 0,
            },
        ],
    }
}

fn create_new_random_block() -> Block {
    let shape: Shape = rand::random();
    create_new_block(shape)
}

fn create_new_block(shape: Shape) -> Block {
    Block {
        x: (FIELD_WIDTH / 2) as i32,
        y: 18,
        stones: create_stones(shape),
    }
}

fn draw_blocks(
    canvas: &mut WindowCanvas,
    field: &[i32; (FIELD_WIDTH * FIELD_HEIGHT) as usize],
    active_block: &Block,
) {
    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            match field[(x * FIELD_HEIGHT + y) as usize] {
                0 => canvas.set_draw_color(Color::RGB(255, 0, 0)),
                1 => canvas.set_draw_color(Color::RGB(100, 50, 50)),
                2 => canvas.set_draw_color(Color::RGB(0, 0, 255)),
                _ => {}
            }
            let rect = Rect::new(
                (FIELD_PADDING_X + x * BLOCK_SIZE) as i32,
                (FIELD_PADDING_Y + (FIELD_HEIGHT - 1 - y) * BLOCK_SIZE) as i32,
                BLOCK_SIZE,
                BLOCK_SIZE,
            );
            canvas.fill_rect(rect).unwrap();
        }
    }

    // Render active block
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    for stone in active_block.stones.iter() {
        let x: i32 = active_block.x + stone.local_x;
        let y: i32 = active_block.y + stone.local_y;
        let rect = Rect::new(
            (FIELD_PADDING_X + x as u32 * BLOCK_SIZE) as i32,
            (FIELD_PADDING_Y + (FIELD_HEIGHT - 1 - y as u32) * BLOCK_SIZE) as i32,
            BLOCK_SIZE,
            BLOCK_SIZE,
        );
        canvas.fill_rect(rect).unwrap();
    }
}

fn draw_grid(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for x in 0..FIELD_WIDTH + 1 {
        let start_point: Point = Point::new(
            (FIELD_PADDING_X + x * BLOCK_SIZE) as i32,
            (FIELD_PADDING_Y) as i32,
        );
        let end_point: Point = Point::new(
            (FIELD_PADDING_X + x * BLOCK_SIZE) as i32,
            (FIELD_PADDING_Y + FIELD_HEIGHT * BLOCK_SIZE) as i32,
        );
        canvas.draw_line(start_point, end_point).unwrap();
    }
    for y in 0..FIELD_HEIGHT + 1 {
        let start_point: Point = Point::new(
            (FIELD_PADDING_X) as i32,
            (FIELD_PADDING_Y + y * BLOCK_SIZE) as i32,
        );
        let end_point: Point = Point::new(
            (FIELD_PADDING_X + FIELD_WIDTH * BLOCK_SIZE) as i32,
            (FIELD_PADDING_Y + y * BLOCK_SIZE) as i32,
        );
        canvas.draw_line(start_point, end_point).unwrap();
    }
}

fn is_block_colliding(block: &Block, field: &[i32; (FIELD_HEIGHT * FIELD_WIDTH) as usize]) -> bool {
    for stone in block.stones.iter() {
        let x: i32 = stone.local_x + block.x;
        let y: i32 = stone.local_y + block.y;
        if field[(x * FIELD_HEIGHT as i32 + y) as usize] != 0 {
            return true;
        }
    }
    false
}

fn save_block_to_field(block: &Block, field: &mut [i32; (FIELD_HEIGHT * FIELD_WIDTH) as usize]) {
    for stone in block.stones.iter() {
        let x: i32 = stone.local_x + block.x;
        let y: i32 = stone.local_y + block.y;
        field[(x * FIELD_HEIGHT as i32 + y) as usize] = 2;
    }
}

// Returns the number of cleared rows
fn clear_full_rows(field: &mut [i32; (FIELD_HEIGHT * FIELD_WIDTH) as usize]) -> u32 {
    println!("Check Field");
    // Ignore bottom line as it acts as a border
    for y in 1..(FIELD_HEIGHT - 1) {
        println!("Check row: {}", y);
        let mut is_row_full = true;
        for x in 1..(FIELD_WIDTH - 1) {
            if field[(x * FIELD_HEIGHT + y) as usize] == 0 {
                is_row_full = false;
                println!("Row {} empty block at {}", y, x);
                break;
            }
        }
        if is_row_full {
            println!("Row {} full", y);
            // Move all rows one down
            for shift_y in y..(FIELD_HEIGHT - 2) {
                for shift_x in 1..(FIELD_WIDTH - 1) {
                    field[(shift_x * FIELD_HEIGHT + shift_y) as usize] =  field[(shift_x * FIELD_HEIGHT + (shift_y + 1)) as usize];
                }
            }
            // Clear first row
            for shift_x in 1..(FIELD_WIDTH - 1) {
                field[(shift_x * FIELD_HEIGHT + (FIELD_HEIGHT - 1)) as usize] = 0;
            }
            return 1 + clear_full_rows(field);
        }
    }
    return 0;
}

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
