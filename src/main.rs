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

    let game_of_life = GameOfLife::new(20, 20, "canvas");

    if game_of_life.is_ok() {
        game_of_life.unwrap().initialize();
    } else {
        stdweb::console!(error, "Failed to initialize game of life");
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