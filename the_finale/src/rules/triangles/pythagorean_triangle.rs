use itertools::Itertools;
use tinyvec::ArrayVec;

use crate::{
    lookup_tables::pythagorean_triples::PythagoreanTriplesTable,
    puzzle::{position::IdentifierVector, Puzzle},
    rules::PuzzleRule,
};

pub struct PythagoreanTriangleRule(pub [IdentifierVector; 3]);

impl PuzzleRule for PythagoreanTriangleRule {
    fn generate(&self, puzzle: &Puzzle, new_puzzles: &mut Vec<Puzzle>) {
        // Go though every pythagorean triple
        let triples = PythagoreanTriplesTable::triples();
        for triple in triples.iter().copied() {
            // Go through every combination of the sides
            let sides = [triple.0, triple.1, triple.2];
            for side_permutation in sides.into_iter().permutations(3) {
                // Wrap each side with their position vector
                let wrapped: Vec<(IdentifierVector, usize)> =
                    self.0.iter().copied().zip(side_permutation).collect();

                // Attempt to insert this permutation sides into the puzzle
                if let Ok(puzzle) = puzzle.try_fit_numbers(&wrapped) {
                    // This combination worked!
                    new_puzzles.push(puzzle);
                }
            }
        }
    }

    fn is_valid(&self, puzzle: &Puzzle) -> bool {
        // Get all the numbers from the puzzle
        let mut numbers = ArrayVec::<[usize; 3]>::new();
        for vector in self.0 {
            if let Some(number) = puzzle.number_at(vector) {
                numbers.push(number);
            } else {
                // This cannot be valid as not all the fields are filled
                return false;
            }
        }

        // Get through every permutation
        if PythagoreanTriplesTable::is_triple(numbers.into_inner()) {
            return true;
        }

        false
    }
}
