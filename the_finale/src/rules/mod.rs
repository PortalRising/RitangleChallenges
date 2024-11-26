use crate::puzzle::Puzzle;

pub mod executor;
pub mod triangles;

/// A rules a puzzle can conform to
pub trait PuzzleRule {
    /// Generate all possible configurations the rule provides on the provided puzzle
    fn generate(&self, puzzle: &Puzzle, new_puzzles: &mut Vec<Puzzle>);

    /// Check if a puzzle configuration still follows this rule
    fn is_valid(&self, puzzle: &Puzzle) -> bool;
}
