extern crate stdweb;

use board::*;
use canvas::*;
use cell::*;
use game_of_life::*;

mod game_of_life;
mod board;
mod cell;
mod canvas;

fn main() {
    stdweb::initialize();

    let canvas_result = Canvas::new("canvas", 500, 500);

    if canvas_result.is_ok() {
        let canvas = canvas_result.unwrap();

        canvas.fill_rect(0, 0, 100, 100, Some("#ff0000"));
        canvas.fill_rect(400, 400, 100, 100, Some("#0000ff"));
    }

    /*
    let game = GameOfLife::new(10, 10);

    for i in &game.board.cells {
        for x in i {
            print!("{:?} ", x.state);
        }
        print!("\n");
    }
    */
}