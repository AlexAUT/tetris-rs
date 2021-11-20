use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

use crate::{
    block::Block,
    field::{BLOCK_SIZE, FIELD_HEIGHT, FIELD_PADDING_X, FIELD_PADDING_Y, FIELD_WIDTH},
};

pub fn draw_blocks(
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

pub fn draw_grid(canvas: &mut WindowCanvas) {
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
