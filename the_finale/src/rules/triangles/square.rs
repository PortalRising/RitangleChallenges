use tinyvec::ArrayVec;

use crate::{
    lookup_tables::square::SquareTable,
    puzzle::{position::IdentifierVector, Puzzle},
    rules::PuzzleRule,
};

pub struct SumSquare(pub [IdentifierVector; 2]);

impl PuzzleRule for SumSquare {
    fn generate(&self, puzzle: &Puzzle, new_puzzles: &mut Vec<Puzzle>) {
        // Make sure that both fields are the same size
        let size_of_fields = puzzle.size_of_field(self.0[0]);
        assert_eq!(size_of_fields, puzzle.size_of_field(self.0[1]));

        // Get all squares
        let squares = SquareTable::squares();

        // Get the minimum number the square must be to have two numbers that both have the size of the fields
        let minimum_field = 10usize.pow(size_of_fields as u32 - 1);
        let minimum_square = minimum_field * 2;

        // Go through all potential squares
        for square in squares
            .into_iter()
            .copied()
            .filter(|&square| square >= minimum_square)
        {
            let max_sub = square - minimum_field + 1;

            // Go through all numbers that add to this square and fulfill the minimum number of digits
            for num_a in minimum_field..max_sub {
                let num_b = square - num_a;

                // Wrap the numbers with their positions
                let wrapped: Vec<(IdentifierVector, usize)> =
                    self.0.iter().copied().zip([num_a, num_b]).collect();

                if let Ok(new_puzzle) = puzzle.try_fit_numbers(&wrapped) {
                    new_puzzles.push(new_puzzle);
                }
            }
        }
    }

    fn is_valid(&self, puzzle: &Puzzle) -> bool {
        // Get all the numbers from the puzzle
        let mut numbers = ArrayVec::<[usize; 2]>::new();
        for vector in self.0 {
            if let Some(number) = puzzle.number_at(vector) {
                numbers.push(number);
            } else {
                // This cannot be valid as not all the fields are filled
                return false;
            }
        }

        SquareTable::is_square(numbers[0] + numbers[1])
    }
}
