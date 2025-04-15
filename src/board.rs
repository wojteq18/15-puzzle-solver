use crate::constants::{SIZE, PUZZLE_SIZE, MOVABLE_PIECE};

#[derive(Clone, Copy, Debug)]

pub struct Field {
    value: usize,
    index: usize,   
}

impl Field {
    pub fn value(&self) -> usize {
        self.value
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

pub struct Board {
    fields: [Field; PUZZLE_SIZE],
    zero_position: usize,
}

impl Board {
    pub fn find_movable_piece(&self) -> Vec<Field> {
        let mut movable_piece = Vec::new();
        let col = self.zero_position % SIZE;
        let row = self.zero_position / SIZE;

        if row > 0 {
            movable_piece.push(self.fields[self.zero_position - SIZE]);
        }
        if row < SIZE - 1 {
            movable_piece.push(self.fields[self.zero_position + SIZE]);
        }
        if col > 0 {
            movable_piece.push(self.fields[self.zero_position - 1]);
        }
        if col < SIZE - 1 {
            movable_piece.push(self.fields[self.zero_position + 1]);
        }

        return movable_piece
    }

    pub fn new() -> Self {
        let mut fields = [Field { value: 0, index: 0 }; PUZZLE_SIZE];
        let zero_position = 4; // The position of the empty space (0)

        for i in 0..PUZZLE_SIZE {
            fields[i] = Field { value: (i + 1) % PUZZLE_SIZE, index: i };

        }

        Board { fields, zero_position }
    }

    pub fn print(&self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                print!("{} ", self.fields[i * SIZE + j].value());
            }
            println!();
        }
    }
}