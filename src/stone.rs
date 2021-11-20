use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;

pub struct Stone {
    pub local_x: i32,
    pub local_y: i32,
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
