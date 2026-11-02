use terminal_tools::canvas::Pos;
use super::Board;

use Direction::*;
use Variant::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    CW,
    CCW,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Right,
    Up,
    Left,
    Down,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Right
    }
}

impl Orientation {
    pub fn rotate(self, direction: Direction) -> Self {
        use Orientation::*;

        match (self, direction) {
            (Right, CW) => Down,
            (Right, CCW) => Up,
            (Up, CW) => Right,
            (Up, CCW) => Left,
            (Left, CW) => Up,
            (Left, CCW) => Down,
            (Down, CW) => Left,
            (Down, CCW) => Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartialOrientation {
    Right,
    Up,
}

impl Default for PartialOrientation {
    fn default() -> Self {
        PartialOrientation::Right
    }
}

impl PartialOrientation {
    pub fn rotate(self) -> Self {
        use PartialOrientation::*;

        match self {
            Right => Up,
            Up => Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    I(PartialOrientation),
    O,
    T(Orientation),
    J(Orientation),
    L(Orientation),
    S(PartialOrientation),
    Z(PartialOrientation),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tetromino {
    pub pos: Pos,
    pub variant: Variant,
}

impl Tetromino {
    pub fn new(pos: Pos, variant: Variant) -> Self {
        Self {
            pos,
            variant,
        }
    }

    pub fn draw(&self, board: &mut Board) {

        match self.variant {
            I(PartialOrientation::Right) => {
                let idx = board.size.cols * self.pos.row + self.pos.col;
                board.data[idx - 1..=idx + 2].fill(true)
            },
            I(PartialOrientation::Up) => {
                let idx = board.size.cols * (self.pos.row - 2) + self.pos.col;
                for i in 0..4 {
                    board.data[idx + i * board.size.cols] = true;
                }
            },
            O => (),
            T(Orientation::Right) => (),
            T(Orientation::Up) => (),
            T(Orientation::Left) => (),
            T(Orientation::Down) => (),
            J(Orientation::Right) => (),
            J(Orientation::Up) => (),
            J(Orientation::Left) => (),
            J(Orientation::Down) => (),
            L(Orientation::Right) => (),
            L(Orientation::Up) => (),
            L(Orientation::Left) => (),
            L(Orientation::Down) => (),
            S(PartialOrientation::Right) => (),
            S(PartialOrientation::Up) => (),
            Z(PartialOrientation::Right) => (),
            Z(PartialOrientation::Up) => (),
        }
    }
}