use itertools::Itertools;
use num_integer::Integer;
use tinyvec::ArrayVec;

use crate::{
    lookup_tables::{coprimes::CoprimesTable, primes::PrimeTable, triangular::TriangularTable},
    puzzle::{position::IdentifierVector, Puzzle},
    rules::PuzzleRule,
};

/// 3 Sides form a triangle whose perimeter is triangular and whose sides are each odd, composite and co-prime to one another
pub struct OddCompositeCoprimeRule(pub [IdentifierVector; 3]);

impl PuzzleRule for OddCompositeCoprimeRule {
    fn generate(&self, puzzle: &Puzzle, new_puzzles: &mut Vec<Puzzle>) {
        // Create a table of all possible trios
        let mut possible_trios = hashbrown::HashSet::<[usize; 3]>::new();

        // Go through all coprime nodes, filtering out the even ones (primes are already filtered)
        let odd_composites = CoprimesTable::composite_coprimes()
            .nodes()
            .filter(|&num| num.is_odd());

        // Go through all odd and composite numbers
        for side_a in odd_composites {
            // Get the coprimes of current number and filter out the even ones (primes are already filtered)
            let coprimes = CoprimesTable::composite_coprimes_of(side_a)
                .expect("This number should not be 1 or a prime")
                .filter(|number| number.is_odd());

            // Create a set of trios and add them to the set
            for other_sides in coprimes.permutations(2) {
                let (side_b, side_c) = (other_sides[0], other_sides[1]);

                // Make sure each side actually forms a valid triangle
                if side_a + side_b <= side_c {
                    continue;
                }

                if side_b + side_c <= side_a {
                    continue;
                }

                if side_c + side_a <= side_b {
                    continue;
                }

                // Check if the perimeter formed is triangular
                let perimeter = side_a + side_b + side_c;
                if !TriangularTable::is_triangular(perimeter) {
                    continue;
                }

                // Make sure all sides are coprime partners
                if !CoprimesTable::are_composite_coprimes(&[side_a, side_b, side_c]) {
                    continue;
                }

                // This is a valid trio
                possible_trios.insert([side_a, side_b, side_c]);
            }
        }

        // Go through all possible trios and create potential puzzles
        for trio in possible_trios.into_iter() {
            // Wrap the numbers with their positions
            let wrapped: Vec<(IdentifierVector, usize)> =
                self.0.iter().copied().zip(trio).collect();

            if let Ok(puzzle) = puzzle.try_fit_numbers(&wrapped) {
                // This combination fit into the puzzle so add it to our buffer
                new_puzzles.push(puzzle);
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

        // Check that all numbers are odd and composite
        for number in numbers {
            // Check if number is even
            if number.is_even() {
                return false;
            }

            // Check if number is a prime
            if PrimeTable::is_prime(number) {
                return false;
            }
        }

        // Check that all numbers are coprime with each other
        CoprimesTable::are_composite_coprimes(&numbers)
    }
}
