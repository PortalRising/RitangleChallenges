use crate::{
    lookup_tables::LookupTables,
    puzzle::{position::PuzzlePosition, Puzzle},
};

/// The rules that a puzzle must follow to be valid as P
pub struct PRules<'a> {
    puzzle: &'a Puzzle,
}

impl<'a> PRules<'a> {
    /// Create a new rule set to be applied on a puzzle for P
    pub fn new(puzzle: &'a Puzzle) -> PRules {
        PRules { puzzle }
    }

    /// 1 Down must be a prime multiple of 4 down
    fn question_four(&self) -> bool {
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
        LookupTables::is_prime(potential_prime)
    }

    /// 2 Down has a square digit sum
    fn question_eight(&self) -> bool {
        // Get 2 Down as digits
        let two_down = self.puzzle.numbers_at(PuzzlePosition::new_down(2));

        // Sum two down
        let sum = two_down.iter().sum();

        // Check if the sum is a square number
        LookupTables::is_square(sum)
    }

    /// 1 Across is a square
    fn question_ten(&self) -> bool {
        // Get 1 Across
        let one_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(1));

        // Check if 1 Across is a square
        LookupTables::is_square(one_across)
    }

    /// 3 Across is the reverse of a prime
    fn question_twelve(&self) -> bool {
        // Get 3 Across as digits
        let three_across = self.puzzle.numbers_at(PuzzlePosition::new_across(3));

        // Join them back up together but reversed (its reverse because i starts at 0 but we start with the most significant digit, therefore it becomes the first digit)
        let mut reversed_three_ac = 0;
        for (i, &num) in three_across.iter().enumerate() {
            reversed_three_ac += 10usize.pow(i as u32) * (num as usize);
        }

        // Check if reversed_three_ac a prime
        LookupTables::is_prime(reversed_three_ac)
    }

    /// 5 Across is divisible by the sum of 9 puzzle digits
    fn question_twenty_one(&self) -> bool {
        // Get 5 Across
        let five_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(5));

        // Get the sum of every digit in the puzzle
        let all_digits_sum = self.puzzle.numbers().iter().sum::<usize>();

        // Based on the clue from question 5, all_digits_sum * n = five_across, n must be greater than 1
        // "A multiple of a number must differ from the original number"

        // Therefore check if they are equal
        if five_across == all_digits_sum {
            // This cannot be valid based on the clue from Q5
            return false;
        }

        // Now if 5 Across is divisible by the sum then this is valid
        five_across % all_digits_sum == 0
    }

    /// 4 Down should be triangular
    fn question_twenty_five(&self) -> bool {
        // Get 4 Down
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        LookupTables::is_triangular(four_down)
    }

    /// Apply all the rules for Q and store whether it was successful
    pub fn apply(&self, is_valid: &mut bool) {
        *is_valid = self.question_four()
            && self.question_eight()
            && self.question_ten()
            && self.question_twelve()
            && self.question_twenty_one()
            && self.question_twenty_five();
    }
}
