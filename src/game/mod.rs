mod game_manager;

pub use game_manager::GameManager;

use terminal_tools::canvas::{Canvas, Size, Pos, Draw, TextBox};
use super::board::{Board, Border};
use std::fmt::Alignment;

use Alignment::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Game;

impl Game {
    pub fn run(&mut self) {
        let mut canvas = Canvas::from(Size::new(22, 50));
        let hold_border = Border::new(Pos::default(), Size::new(7, 14));
        let board_border = Border::new(Pos::new(0, 14), Size::new(22, 22));
        let next_border = Border::new(Pos::new(0, 36), Size::new(12, 14));

        hold_border.draw(&mut canvas);
        board_border.draw(&mut canvas);
        next_border.draw(&mut canvas);
        
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
}