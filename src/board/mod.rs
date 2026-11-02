use terminal_tools::canvas::{Canvas, Draw, Pos, Size};
use colored::*;

pub mod tetromino;
mod border;

pub use border::Border;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Board {
    pos: Pos,
    size: Size,
    data: Vec<bool>,
}

impl Draw for Board {
    fn draw(&self, canvas: &mut Canvas) {
        let mut chars = canvas
            .get_rows_mut(self.pos.row, self.pos.row + self.size.rows)
            .map(|row| &mut row[self.pos.col..self.pos.col + 2 * self.size.cols])
            .flatten();

        for &is_block in &self.data {
            if let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
                if is_block {
                    *c1 = "[".normal();
                    *c2 = "]".normal();
                }
            }
        }
    }
}

impl Board {
    pub fn new(pos: Pos, size: Size) -> Self {
        Self {
            pos,
            size,
            data: vec![false; size.rows * size.cols],
        }
    }
}