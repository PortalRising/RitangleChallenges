use crate::{primes::PRIMES, puzzle::position::PuzzlePosition};

use super::Puzzle;

/// Enforces the clues as rules on the provided puzzle
pub struct RuleEnforcer<'a> {
    puzzle: &'a Puzzle,
}

impl<'a> RuleEnforcer<'a> {
    /// Create a new rule applier with a prime cache and with the puzzle to test
    pub fn new(puzzle: &'a Puzzle) -> Self {
        Self { puzzle }
    }

    /// All numbers on all puzzles must not start with 0
    fn question_five(&self) -> bool {
        // For every possible position, find the first digit and make sure its not 0
        [0, 1, 4, 5, 6]
            .iter()
            .all(|&i| self.puzzle.numbers()[i] != 0)
    }

    /// 1 Down must be a prime multiple of 4 down
    fn question_four(&mut self) -> bool {
        // Get the two columns
        let one_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(1));
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        // Check if 4 down is divisible by 1 down
        if one_down % four_down != 0 {
            // This is clearly not a prime multiple
            return false;
        }

        // Divide 4 down by 1 down
        let potential_prime = one_down / four_down;

        // Check if its a prime
        PRIMES.binary_search(&potential_prime).is_ok()
    }

    /// 1 Across must be palindromic prime
    fn question_seven(&mut self) -> bool {
        // Get 1ac digits
        let one_across_position = PuzzlePosition::new_across(1);
        let one_across_digits = self.puzzle.numbers_at(one_across_position);

        // Check if the first digit is equal to the last
        if one_across_digits[0] != one_across_digits[2] {
            return false;
        }

        let potential_prime = self.puzzle.joined_numbers_at(one_across_position);

        PRIMES.binary_search(&potential_prime).is_ok()
    }

    /// 4 Down must be a cube
    fn question_nine(&self) -> bool {
        // Get 4dn
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        // 27 and 64 are the only 2 digit cubes
        [27, 64].contains(&four_down)
    }

    fn question_fourteen(&self) -> bool {
        let one_down_position = PuzzlePosition::new_down(1);

        let one_down_digits = self.puzzle.numbers_at(one_down_position);
        let one_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(1));

        let total = one_across - one_down_digits.iter().sum::<u8>() as usize;

        let one_down = self.puzzle.joined_numbers_at(one_down_position);

        total == one_down
    }

    /// 2 Down is a prime
    fn question_seventeen(&mut self) -> bool {
        // Get 2 down
        let two_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(2));

        PRIMES.binary_search(&two_down).is_ok()
    }

    /// N (the middle element) must be number between 1 to 4
    fn question_eighteen(&self) -> bool {
        let middle = self.puzzle.numbers()[4];

        middle >= 1 && middle <= 4
    }

    /// Apply the rules for Q on the provided puzzle
    pub fn validate_q(&mut self) -> bool {
        self.question_five()
            && self.question_four()
            && self.question_seven()
            && self.question_nine()
            // && self.question_fourteen()
            && self.question_seventeen()
            && self.question_eighteen()
    }
}
