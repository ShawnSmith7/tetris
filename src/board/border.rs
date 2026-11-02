use terminal_tools::canvas::{Pos, Size, Draw, Canvas};
use terminal_tools::canvas::line::Line;
use terminal_tools::canvas::line::LineVariant::{Horiz, Vert};
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Border {
    pub pos: Pos,
    pub size: Size,
}

impl Border {
    pub fn new(pos: Pos, size: Size) -> Self {
        Self {
            pos,
            size,
        }
    }
}

impl Draw for Border {
    fn draw(&self, canvas: &mut Canvas) {
        let last_row = self.size.rows - 1;
        let last_col = self.size.cols - 1;

        canvas.set(self.pos, "+".normal());
        canvas.set(self.pos + Pos::new(0, last_col), "+".normal());
        canvas.set(self.pos + Pos::new(last_row, 0), "+".normal());
        canvas.set(self.pos + Pos::new(last_row, last_col), "+".normal());

        let mut horiz = Line::new(self.pos + Pos::new(0, 1), self.size.cols - 2, "-".normal(), Horiz);
        let mut vert = Line::new(self.pos + Pos::new(1, 0), self.size.rows - 2, "|".normal(), Vert);

        horiz.draw(canvas);
        vert.draw(canvas);

        horiz.pos.row = self.pos.row + last_row;
        vert.pos.col = self.pos.col + last_col;

        horiz.draw(canvas);
        vert.draw(canvas);
    }
}