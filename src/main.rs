mod board;
mod constants;

use std::collections::BinaryHeap;
use board::*;

fn main() { 
    let mut board = Board::new();
    //board.print();
    board.shuffle();
    board.print();
    let mut mov = board.find_movable_piece();
    println!("Movable pieces: {:?}", mov);
    println!("{}", board.get_zero_element());
    board.swap(board.get_zero_element(), mov[0]);
    board.print();
    let mov1 = board.find_movable_piece();
    println!("Movable pieces: {:?}", mov1);
    println!("{}", board.get_zero_element());
}