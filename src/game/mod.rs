mod game_manager;

pub use game_manager::GameManager;

use terminal_tools::canvas::{Canvas, Size, Pos, Draw, line, TextBox};
use line::{Line, LineVariant};
use super::Board;
use std::fmt::Alignment;

use LineVariant::*;
use Alignment::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Game;

impl Game {
    pub fn run(&mut self) {
        let mut canvas = Canvas::from(Size::new(22, 50));

        Self::draw_border(Pos::default(), Size::new(7, 14), &mut canvas);
        Self::draw_border(Pos::new(0, 14), Size::new(22, 22), &mut canvas);
        Self::draw_border(Pos::new(0, 36), Size::new(12, 14), &mut canvas);

        TextBox::new(Pos::new(1, 1), 12, Center, "HOLD").draw(&mut canvas);
        TextBox::new(Pos::new(1, 37), 12, Center, "NEXT").draw(&mut canvas);

        let hold = Board::new(Pos::new(2, 1), Size::new(4, 6));
        let board = Board::new(Pos::new(1, 15), Size::new(20, 10));
        let next = Board::new(Pos::new(2, 37), Size::new(9, 6));

        hold.draw(&mut canvas);
        board.draw(&mut canvas);
        next.draw(&mut canvas);

        canvas.print_centered();
    }

    fn draw_border(pos: Pos, size: Size, canvas: &mut Canvas) {
        let last_row = size.rows - 1;
        let last_col = size.cols - 1;

        canvas.set(pos, '+');
        canvas.set(pos + Pos::new(0, last_col), '+');
        canvas.set(pos + Pos::new(last_row, 0), '+');
        canvas.set(pos + Pos::new(last_row, last_col), '+');

        let mut horiz = Line::new(pos + Pos::new(0, 1), size.cols - 2, '-', Horiz);
        let mut vert = Line::new(pos + Pos::new(1, 0), size.rows - 2, '|', Vert);

        horiz.draw(canvas);
        vert.draw(canvas);

        horiz.pos.row = pos.row + last_row;
        vert.pos.col = pos.col + last_col;

        horiz.draw(canvas);
        vert.draw(canvas);
    }
}