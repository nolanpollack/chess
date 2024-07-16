use crate::BOARD_SIZE;

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        if x >= BOARD_SIZE || y >= BOARD_SIZE {
            panic!("Coordinate must be within the bounds of the board");
        }
        Coordinate { x, y }
    }
}
