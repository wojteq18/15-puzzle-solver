use crate::constants::{SIZE, PUZZLE_SIZE, MOVABLE_PIECE};
use crate::board::Board;
use priority_queue::PriorityQueue;
use std::collections::{HashSet};
use std::cmp::Reverse;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct State {
    board: Board,
    cost: usize, //liczba ruchów wykonanych od początku
    estimated_cost: usize, //heurystyka + cost, ustala priorytet w kolejce
}

impl State {
    pub fn new(mut board: Board, cost: usize) -> Self {
        let heuristic = board.how_many_correct();
        let estimated_cost = cost + (PUZZLE_SIZE - heuristic);
        State {
            board,
            cost,
            estimated_cost,
        }
    }
}

fn fix(board: &mut Board) {
    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();

    queue.push(State::new(board.clone(), 0), Reverse(0));
    visited.insert(board.clone());
}