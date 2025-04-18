use crate::constants::{SIZE, PUZZLE_SIZE};
//use rand::Rng;
use rand::seq::SliceRandom; // Importujemy SliceRandom dla metody shuffle

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]

pub struct Field {
    value: usize,
    index: usize,   
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Board {
    fields: [Field; PUZZLE_SIZE],
    zero_position: usize,
    //how_many_correct: usize,
}

impl Board {

    pub fn get_zero_element(&self) -> usize {
        return self.zero_position;
    }

    pub fn find_movable_piece(&self) -> Vec<usize> {
        let mut movable_piece = Vec::new();
        let col = self.zero_position % SIZE;
        let row = self.zero_position / SIZE;

        if row > 0 {
            movable_piece.push(self.fields[self.zero_position - SIZE].index);
        }
        if row < SIZE - 1 {
            movable_piece.push(self.fields[self.zero_position + SIZE].index);
        }
        if col > 0 {
            movable_piece.push(self.fields[self.zero_position - 1].index);
        }
        if col < SIZE - 1 {
            movable_piece.push(self.fields[self.zero_position + 1].index);
        }

        return movable_piece
    }

    pub fn new() -> Self {
        let mut fields = [Field { value: 0, index: 0 }; PUZZLE_SIZE];
        let zero_position = PUZZLE_SIZE - 1; // The position of the empty space (0)

        for i in 0..PUZZLE_SIZE {
            fields[i] = Field { value: (i + 1) % PUZZLE_SIZE, index: i };

        }

        Board { fields, zero_position }
    }

    pub fn print(&self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("    {}    ", self.fields[i * SIZE + j].value);
            }
            println!();
        }
    }

    pub fn shuffle(&mut self) {
        let mut numbers: Vec<usize> = (1..PUZZLE_SIZE).collect();
        let mut rng = rand::rng();
        numbers.shuffle(&mut rng);
        
        for i in 0..PUZZLE_SIZE - 1 {
            self.fields[i].value = numbers[i];   
        }
    }

    pub fn swap(&mut self, index1: usize, index2: usize) {
        let value1 = self.fields[index1].value;
        let value2 = self.fields[index2].value;
    
        self.fields.swap(index1, index2);
    
        if value1 == 0 {
            self.zero_position = index2;
        } else if value2 == 0 {
            self.zero_position = index1;
        }
    
        self.fields[index1].index = index1;
        self.fields[index2].index = index2;
    }

    pub fn how_many_correct(self) -> usize {
        let mut count = 0;
        for i in 0..PUZZLE_SIZE{
            if self.fields[i].index == (self.fields[i].value + 8) % PUZZLE_SIZE {
                count += 1;
            }
        }
        return count;
    }

    pub fn manhattan_distance(&self) -> usize {
        let mut distance: usize = 0;
        for i in 0..PUZZLE_SIZE {
            let value = self.fields[i].value;
            if value != 0 {
                let target_row = (value + PUZZLE_SIZE - 1) % PUZZLE_SIZE / SIZE;
                let target_col = ((value + PUZZLE_SIZE - 1) % PUZZLE_SIZE) % SIZE;
                let current_row = i / SIZE;
                let current_col = i % SIZE;
                distance += (target_row as isize - current_row as isize).abs() as usize
                    + (target_col as isize - current_col as isize).abs() as usize;
            }
        }
        return distance
    }
}