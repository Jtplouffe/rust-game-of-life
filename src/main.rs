extern crate stdweb;
use stdweb::js;

mod game_of_life;
mod board;
mod cell;
mod canvas;

use game_of_life::*;
use board::*;
use cell::*;
use canvas::*;

// https://github.com/KappaDistributive/rs2048

fn main() {
    stdweb::initialize();

    let game = GameOfLife::new(10, 10, );

    for i in &game.board.cells {
        for x in i {
            print!("{:?} ", x.state);
        }
        print!("\n");
    }
}