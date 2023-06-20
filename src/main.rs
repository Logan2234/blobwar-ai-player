extern crate blobwar;
// use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, MinMax};

fn main() {
    // let board = Board::load("quantum").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
<<<<<<< HEAD
    game.battle(AlphaBeta(7), Greedy());
=======
    game.battle(AlphaBeta(6), MinMax(4));
>>>>>>> 63ac43fbaf32cac0b7c4c5258cdc709769d69020
}
