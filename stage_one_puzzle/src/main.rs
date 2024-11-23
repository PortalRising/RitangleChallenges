use std::array;

use puzzle::Puzzle;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rules::RuleEnforcer;

pub mod lookup_tables;
pub mod puzzle;
pub mod rules;
pub mod us;

fn main() {
    // Go through every possible grid
    // and store all the valid grids that follow the rules
    let any_valid_puzzle: Vec<_> = (0..Puzzle::max_permutations())
        .into_par_iter()
        // Convert the index to the sequence of digits for the puzzle
        .map(|seed_sequence| Puzzle::new(seed_sequence))
        // Validate the puzzle and check if follows the rules provided
        .map(|attempt| {
            // Create a rule enforcer
            let mut rule_enforcer = RuleEnforcer::new(&attempt);

            // Apply all clues and see if its valid
            let result = rule_enforcer.apply_all_rules();

            (attempt, result)
        })
        // Filter out all puzzles that not valid for any rules
        .filter(|(_, result)| result.is_any_valid())
        .collect();

    // Filter out the puzzles into the kinds they are valid for
    let mut valid_puzzle_groups: [Vec<Puzzle>; 4] = array::from_fn(|_| Vec::with_capacity(512));

    for (puzzle, result) in any_valid_puzzle {
        for (puzzle_kind, is_valid) in result.results().into_iter().enumerate() {
            if !is_valid {
                continue;
            }

            valid_puzzle_groups[puzzle_kind].push(puzzle);
        }
    }

    // Output the number of valid puzzles for each kind
    println!(
        "{} Valid puzzle P permutations",
        valid_puzzle_groups[0].len()
    );
    println!(
        "{} Valid puzzle Q permutations",
        valid_puzzle_groups[1].len()
    );
    println!(
        "{} Valid puzzle R permutations",
        valid_puzzle_groups[2].len()
    );
    println!(
        "{} Valid puzzle S permutations",
        valid_puzzle_groups[3].len()
    );
}
