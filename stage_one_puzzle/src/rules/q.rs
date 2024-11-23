use crate::{
    lookup_tables::LookupTables,
    puzzle::{position::PuzzlePosition, Puzzle},
};

/// The rules that a puzzle must follow to be valid as Q
pub struct QRules<'a> {
    puzzle: &'a Puzzle,
}

impl<'a> QRules<'a> {
    /// Create a new rule set to be applied on a puzzle for Q
    pub fn new(puzzle: &'a Puzzle) -> QRules {
        QRules { puzzle }
    }

    /// 1 Across must be palindromic prime
    fn question_seven(&self) -> bool {
        // Get 1ac digits
        let one_across_position = PuzzlePosition::new_across(1);
        let one_across_digits = self.puzzle.numbers_at(one_across_position);

        // Check if the first digit is equal to the last
        if one_across_digits.first().unwrap() != one_across_digits.last().unwrap() {
            return false;
        }

        let potential_prime = self.puzzle.joined_numbers_at(one_across_position);

        LookupTables::is_prime(potential_prime)
    }

    /// 4 Down must be a cube
    fn question_nine(&self) -> bool {
        // Get 4dn
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        // 27 and 64 are the only 2 digit cubes
        [27, 64].contains(&four_down)
    }

    /// 1 Down must be 1 Across - the digit of sum of 1 Down
    fn question_fourteen(&self) -> bool {
        let one_down_position = PuzzlePosition::new_down(1);

        let one_down_digits = self.puzzle.numbers_at(one_down_position);
        let one_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(1));

        let total = one_across - one_down_digits.iter().sum::<usize>();

        let one_down = self.puzzle.joined_numbers_at(one_down_position);

        total == one_down
    }

    /// 2 Down is a prime
    fn question_seventeen(&self) -> bool {
        // Get 2 down
        let two_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(2));

        LookupTables::is_prime(two_down)
    }

    /// Apply all the rules for Q and store whether it was successful
    pub fn apply(&self, is_valid: &mut bool) {
        *is_valid = self.question_seven()
            && self.question_nine()
            && self.question_fourteen()
            && self.question_seventeen();
    }
}
