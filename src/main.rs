mod board;
mod constants;
mod state;

use board::*;
use state::*;

fn main() { 
    let mut board = Board::new();
    board.shuffle();
    board.print();
    let _ = fix(&mut board);
}