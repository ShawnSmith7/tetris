use terminal_tools::canvas::Pos;

use Direction::*;

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
    pos: Pos,
    variant: Variant,
}

impl Tetromino {
    pub fn new(pos: Pos, variant: Variant) -> Self {
        Self {
            pos,
            variant,
        }
    }
}