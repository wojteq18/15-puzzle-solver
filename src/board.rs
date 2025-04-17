use crate::constants::{SIZE, PUZZLE_SIZE, MOVABLE_PIECE};
//use rand::Rng;
use rand::seq::SliceRandom; // Importujemy SliceRandom dla metody shuffle

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]

pub struct Field {
    value: usize,
    index: usize,   
}

impl Field {
    pub fn value(&self) -> usize {
        self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Board {
    fields: [Field; PUZZLE_SIZE],
    zero_position: usize,
    how_many_correct: usize,
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
        let how_many_correct = 9;

        for i in 0..PUZZLE_SIZE {
            fields[i] = Field { value: (i + 1) % PUZZLE_SIZE, index: i };

        }

        Board { fields, zero_position, how_many_correct }
    }

    pub fn print(&self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("{} ", self.fields[i * SIZE + j].value());
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

    pub fn how_many_correct(&mut self) -> usize {
        let mut count = 0;
        for i in 0..PUZZLE_SIZE{
            if self.fields[i].index == (self.fields[i].value + 8) % PUZZLE_SIZE {
                count += 1;
            }
        }
        self.how_many_correct = count;
        return count;
    }

    /*pub fn fix(&mut self) {
        while self.how_many_correct != PUZZLE_SIZE - 1 {
            let mut values = Vec::new();
            let mov = self.find_movable_piece();
    
            for i in 0..mov.len() {
                self.swap(self.get_zero_element(), mov[i]);
                let y = self.how_many_correct();
                values.push(y);
                self.swap(mov[i], self.get_zero_element());
            }
    
            if let Some((best_index, _)) = values.iter().enumerate().max_by_key(|&(_, &v)| v) {
                self.swap(self.get_zero_element(), mov[best_index]);
            }
        }
    }*/
    
}