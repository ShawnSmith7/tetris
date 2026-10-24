use terminal_tools::canvas;
use canvas::{Canvas, Size, Pos, line, Draw};
use line::*;

use LineVariant::*;

fn main() {
    let mut canvas = Canvas::from(Size::new(22, 46));

    draw_border(Pos::default(), Size::new(6, 12), &mut canvas);
    draw_border(Pos::new(0, 12), Size::new(22, 22), &mut canvas);
    draw_border(Pos::new(0, 34), Size::new(11, 12), &mut canvas);

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