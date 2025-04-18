use crate::constants::PUZZLE_SIZE;
use crate::board::Board;
use priority_queue::PriorityQueue;
use std::collections::HashSet;
use std::cmp::Reverse;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct State {
    board: Board,
    cost: usize, //liczba ruchów wykonanych od początku
    estimated_cost: usize, //heurystyka + cost, ustala priorytet w kolejce
}

impl State {
    pub fn new(board: Board, cost: usize) -> Self {
        let heuristic = board.manhattan_distance();
        let estimated_cost = cost + heuristic;
        State {
            board,
            cost,
            estimated_cost,
        }
    }
}

pub fn fix(board: &mut Board) {
    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();

    queue.push(State::new(board.clone(), 0), Reverse(0)); //odwracamy, żeby mieć najmniejszy koszt na początku
    visited.insert(board.clone());

    while let Some((state, _)) = queue.pop() { 
        if state.board.how_many_correct() == PUZZLE_SIZE {
            println!("Found solution with cost: {}", state.cost);
            state.board.print();
            return;
        }

        for next_move in state.board.find_movable_piece() {
            let mut next_board = state.board.clone();
            next_board.swap(state.board.get_zero_element(), next_move);
            let next_state = State::new(next_board.clone(), state.cost + 1);
            if !visited.contains(&next_board) {
                visited.insert(next_board.clone());
                queue.push(next_state.clone(), Reverse(next_state.estimated_cost));
            }
        }
    }
}