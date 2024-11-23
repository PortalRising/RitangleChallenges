use crate::{
    lookup_tables::LookupTables,
    puzzle::{position::PuzzlePosition, Puzzle},
};

/// The rules that a puzzle must follow to be valid as R
pub struct RRules<'a> {
    puzzle: &'a Puzzle,
}

impl<'a> RRules<'a> {
    /// Create a new rule set to be applied on a puzzle for R
    pub fn new(puzzle: &'a Puzzle) -> RRules {
        RRules { puzzle }
    }

    /// Two down is the sum of (the digit product of 1 Across) and (the digit product of 1 Down)
    fn question_eleven(&self) -> bool {
        // Get 2 Down
        let two_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(2));

        // Get 1 Across and 1 Down and find their products
        let one_across_digits: usize = self
            .puzzle
            .numbers_at(PuzzlePosition::new_across(1))
            .into_iter()
            .map(|n| n as usize)
            .product();

        let one_down_digits: usize = self
            .puzzle
            .numbers_at(PuzzlePosition::new_down(1))
            .into_iter()
            .map(|n| n as usize)
            .product();

        // Check if two down is the sum of the digit products
        two_down == (one_across_digits + one_down_digits)
    }

    /// For R, 1 Down has the same digit sum as 2 Down
    fn question_fifteen(&self) -> bool {
        // Get the two columns as digits
        let one_down_digits = self.puzzle.numbers_at(PuzzlePosition::new_down(1));
        let two_down_digits = self.puzzle.numbers_at(PuzzlePosition::new_down(2));

        // Find the sum of their digits
        let one_down_sum = one_down_digits.iter().sum::<usize>();
        let two_down_sum = two_down_digits.iter().sum::<usize>();

        one_down_sum == two_down_sum
    }

    /// For R, 4 Down is equal to 5 Across - any square
    fn question_sixteen(&self) -> bool {
        // Get 4 Down and 5 Across
        let four_down = self.puzzle.joined_numbers_at(PuzzlePosition::new_down(4));
        let five_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(5));

        // Rearrange the formula to give,
        // Any square = 5 Across - 4 Down
        let any_square = five_across - four_down;

        LookupTables::is_square(any_square)
    }

    /// For R, 3 Across is 4 Down - the digit sum of 4 Down
    fn question_nineteen(&self) -> bool {
        // Create a position for 4 Down
        let four_down_position = PuzzlePosition::new_down(4);

        // Get 3 Across and 4 Down
        let three_across = self.puzzle.joined_numbers_at(PuzzlePosition::new_across(3));
        let four_down = self.puzzle.joined_numbers_at(four_down_position);

        // Get 4 down as the sum of digits
        let four_down_sum = self
            .puzzle
            .numbers_at(four_down_position)
            .iter()
            .sum::<usize>();

        // 3 Across is 4 Down - the digit sum of 4 Down
        three_across == (four_down - four_down_sum)
    }

    /// 1 Across has the same digit sum as 1 Down
    fn question_twenty_three(&self) -> bool {
        // Get 1 Across and 1 Down as digits
        let one_across_digits = self.puzzle.numbers_at(PuzzlePosition::new_across(1));
        let one_down_digits = self.puzzle.numbers_at(PuzzlePosition::new_down(1));

        // Find the sum of their digits
        let one_down_sum = one_down_digits.iter().sum::<usize>();
        let two_down_sum = one_across_digits.iter().sum::<usize>();

        one_down_sum == two_down_sum
    }

    /// 5 Across has prime digit sum
    fn question_twenty_six(&self) -> bool {
        // Get 5 Across as digit sum
        let five_across = self
            .puzzle
            .numbers_at(PuzzlePosition::new_across(5))
            .into_iter()
            .sum::<usize>();

        LookupTables::is_prime(five_across)
    }

    /// Apply all the rules for Q and store whether it was successful
    pub fn apply(&self, is_valid: &mut bool) {
        *is_valid = self.question_eleven()
            && self.question_fifteen()
            && self.question_sixteen()
            && self.question_nineteen()
            && self.question_twenty_three()
            && self.question_twenty_six();
    }
}
