use rand::{distributions::Standard, prelude::Distribution, Rng};

pub struct Stone {
    pub local_x: i32,
    pub local_y: i32,
}

pub struct Block {
    pub x: i32,
    pub y: i32,
    pub stones: [Stone; 4],
}

pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Distribution<Shape> for Standard {
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
