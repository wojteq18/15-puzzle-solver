mod board;
mod constants;

use std::collections::BinaryHeap;
use board::*;

fn main() { 
    let mut board = Board::new();
    board.print();
    let mut mov = board.find_movable_piece();
    println!("Movable pieces: {:?}", mov);
}