use crate::stone::{Stone, Shape};

pub struct Block {
    pub x: i32,
    pub y: i32,
    pub stones: [Stone; 4],
}


pub fn create_stones(shape: Shape) -> [Stone; 4] {
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
