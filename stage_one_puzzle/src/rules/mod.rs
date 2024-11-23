use p::PRules;
use q::QRules;
use r::RRules;
use result::ValidationResult;
use s::SRules;

use super::Puzzle;

mod p;
mod q;
mod r;
pub mod result;
mod s;

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

    /// N (the middle element) on all puzzles must be number between 1 to 4
    fn question_eighteen(&self) -> bool {
        let middle = self.puzzle.numbers()[4];

        middle >= 1 && middle <= 4
    }

    /// The letters A to H define a location within the continental United States as follows.
    /// AB gives the degrees North
    /// and CD the corresponding minutes;
    /// EF gives the degrees West
    /// and GH the corresponding minutes.
    /// This function converts the values of A to H to longitude and latitude then confines them to the continental US
    fn _question_twenty_one() {}

    /// Apply all the rules for all kinds of the puzzle
    pub fn apply_all_rules(&mut self) -> ValidationResult {
        // Create a invalid result
        let mut result = ValidationResult::new_invalid();

        // All puzzles must follow these rules
        let follows_base_rules = self.question_five() && self.question_eighteen();

        if !follows_base_rules {
            // It cannot follow the base rules for all puzzles
            // so its invalid for all kinds of the puzzle
            return result;
        }

        // Apply rules for all kinds of puzzle and store them in our result
        PRules::new(&self.puzzle).apply(result.p_mut());
        QRules::new(&self.puzzle).apply(result.q_mut());
        RRules::new(&self.puzzle).apply(result.r_mut());
        SRules::new(&self.puzzle).apply(result.s_mut());

        // && self.question_four()
        return result;
    }
}
