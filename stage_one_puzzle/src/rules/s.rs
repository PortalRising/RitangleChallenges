use crate::{
    lookup_tables::LookupTables,
    puzzle::{position::PuzzlePosition, Puzzle},
};

/// The rules that a puzzle must follow to be valid as S
pub struct SRules<'a> {
    puzzle: &'a Puzzle,
}

impl<'a> SRules<'a> {
    /// Create a new rule set to be applied on a puzzle for S
    pub fn new(puzzle: &'a Puzzle) -> SRules {
        SRules { puzzle }
    }

    /// 1 Down is equal to the 1 Across digit sum squared + 3 Across
    fn question_thirteen(&self) -> bool {
        // Get 1 Down and 3 Across
        let one_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(1));
        let three_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(3));

        // Get the digit sum squared of 1 Across
        let one_across_squared_sum = self
            .puzzle
            .numbers_at(PuzzlePosition::new_across(1))
            .into_iter()
            .map(|n| n as usize)
            .sum::<usize>()
            .pow(2);

        one_down == (one_across_squared_sum + three_across)
    }

    /// For S, 4 Down is triangular
    fn question_fifteen(&self) -> bool {
        // Get 4 Down
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        LookupTables::is_triangular(four_down)
    }

    /// For S, 3 Across is Any Triangular - 4 Down
    fn question_sixteen(&self) -> bool {
        // Get 3 Across and 4 Down
        let three_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(3));
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        // Rearrenge the equation to form
        // Any Triangular = 3 Across + 4 Down
        let any_triangular = three_across + four_down;

        LookupTables::is_triangular(any_triangular)
    }

    /// For S, 2 Down is the reverse of 3 Across
    fn question_nineteen(&self) -> bool {
        // Get 2 Down and 3 Across as digits
        let two_down_digits = self.puzzle.numbers_at(PuzzlePosition::new_down(2));
        let mut three_across_digits = self.puzzle.numbers_at(PuzzlePosition::new_across(3));

        // Reverse 3 Across
        three_across_digits.reverse();

        // Now compare
        two_down_digits == three_across_digits
    }

    /// 5 Across is the sum of 1 Down, 3 Across, and 4 Down
    fn question_twenty_four(&self) -> bool {
        // Get all rows and columns
        let five_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(5));
        let one_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(1));
        let three_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(3));
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));

        // 5ac = 1dn + 3ac + 4dn
        five_across == (one_down + three_across + four_down)
    }

    /// Apply all the rules for Q and store whether it was successful
    pub fn apply(&self, is_valid: &mut bool) {
        *is_valid = self.question_thirteen()
            && self.question_fifteen()
            && self.question_sixteen()
            && self.question_nineteen()
            && self.question_twenty_four();
    }
}
