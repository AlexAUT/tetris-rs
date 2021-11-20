use crate::block::{create_stones, Block, Shape};

pub const FIELD_WIDTH: u32 = 12;
pub const FIELD_HEIGHT: u32 = 23;
pub const BLOCK_SIZE: u32 = 30;
pub const FIELD_PADDING_X: u32 = 30;
pub const FIELD_PADDING_Y: u32 = 20;

pub fn create_new_random_block() -> Block {
    let shape: Shape = rand::random();
    create_new_block(shape)
}

pub fn create_new_block(shape: Shape) -> Block {
    Block {
        x: (FIELD_WIDTH / 2) as i32,
        y: 18,
        stones: create_stones(shape),
    }
}

pub fn is_block_colliding(
    block: &Block,
    field: &[i32; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
) -> bool {
    for stone in block.stones.iter() {
        let x: i32 = stone.local_x + block.x;
        let y: i32 = stone.local_y + block.y;
        if field[(x * FIELD_HEIGHT as i32 + y) as usize] != 0 {
            return true;
        }
    }
    false
}

pub fn save_block_to_field(
    block: &Block,
    field: &mut [i32; (FIELD_HEIGHT * FIELD_WIDTH) as usize],
) {
    for stone in block.stones.iter() {
        let x: i32 = stone.local_x + block.x;
        let y: i32 = stone.local_y + block.y;
        field[(x * FIELD_HEIGHT as i32 + y) as usize] = 2;
    }
}

// Returns the number of cleared rows
pub fn clear_full_rows(field: &mut [i32; (FIELD_HEIGHT * FIELD_WIDTH) as usize]) -> u32 {
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
                    field[(shift_x * FIELD_HEIGHT + shift_y) as usize] =
                        field[(shift_x * FIELD_HEIGHT + (shift_y + 1)) as usize];
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
