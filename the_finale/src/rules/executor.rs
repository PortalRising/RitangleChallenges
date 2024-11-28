use crate::puzzle::Puzzle;

use super::PuzzleRule;

/// A partially completed puzzle
struct IncompleteArtifact {
    pub puzzle: Puzzle,
    pub next_rule: usize,
}

/// An executor that executes a series of rules to generate a reduced set of potiential valid puzzle combinations
pub struct RuleExecutor<'a> {
    base_puzzle: Puzzle,
    rules: Vec<&'a dyn PuzzleRule>,
}

impl<'a> RuleExecutor<'a> {
    /// Create a new executor with a base puzzle and a rule set
    pub fn new(base_puzzle: Puzzle, rules: Vec<&'a dyn PuzzleRule>) -> Self {
        Self { base_puzzle, rules }
    }

    /// Compute solutions to the puzzle
    pub fn compute(&mut self) -> Vec<Puzzle> {
        // TODO! Eventually parallelise this if necessary
        let mut puzzle_queue = Vec::with_capacity(16 * 1024);

        // Add the base untouched puzzle to the queue as its our starting point
        puzzle_queue.push(IncompleteArtifact {
            puzzle: self.base_puzzle.clone(),
            next_rule: 0,
        });

        // Create a buffer for all complete puzzles to go into
        let mut complete_puzzles = Vec::new();

        // Create a buffer for rules to fill
        let mut puzzle_buffer = Vec::new();

        // Go through every single puzzle in the queue
        let mut attempted_puzzles: usize = 0;
        while let Some(artifact) = puzzle_queue.pop() {
            attempted_puzzles += 1;

            // Get the next rule to update
            let current_rule = artifact.next_rule;

            // Evaluate the next rule
            self.rules[current_rule].generate(&artifact.puzzle, &mut puzzle_buffer);

            // If this was the last rule empty the puzzle buffer into the complete puzzles stack
            if current_rule == self.rules.len() - 1 {
                complete_puzzles.extend(puzzle_buffer.drain(..));
                continue;
            }

            // This was not the last rule to apply so take puzzles from puzzle buffer and convert them into artifacts
            let new_artifacts = puzzle_buffer.drain(..).map(|puzzle| IncompleteArtifact {
                puzzle,
                next_rule: current_rule + 1,
            });

            // Add the artifacts to the buffer of incomplete puzzles
            puzzle_queue.extend(new_artifacts);
        }

        // No identifier can start with zero so eliminate those puzzles
        complete_puzzles.retain(|puzzle| {
            for identifier in puzzle.identifers() {
                // Convert identifier to grid position
                let grid_position = puzzle.identifer_to_grid(*identifier);
                
                // If the digit is 0 or unfilled, this is not a valid puzzle
                if puzzle.digit_at(grid_position).unwrap_or(0) == 0 {
                    return false;
                }
            }

            true
        });

        // complete_puzzles.retain(|puzzle| self.rules.iter().all(|rule| rule.is_valid(puzzle)));

        println!("A: {}", attempted_puzzles);

        complete_puzzles
    }
}
