use std::{array, collections::HashSet};

use itertools::Itertools;
use puzzle::Puzzle;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rules::RuleEnforcer;
use us::UnitedStatesLookup;

pub mod lookup_tables;
pub mod puzzle;
pub mod rules;
pub mod us;

fn main() {
    UnitedStatesLookup::is_within_us(0.0, 0.0);

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

    // Collect all the values of N in each puzzle kind
    let values_of_n: [Vec<usize>; 4] = array::from_fn(|group_index| {
        let mut unique_n_values = HashSet::new();

        // Go through every valid puzzle and append the value of N to the hashset
        for puzzle in &valid_puzzle_groups[group_index] {
            unique_n_values.insert(puzzle.n_digit());
        }

        unique_n_values.into_iter().collect()
    });

    // Filter out all invalid combinations of n orderings (all values of n must be unique)
    let valid_n_combos: Vec<Vec<usize>> = values_of_n
        .into_iter()
        .multi_cartesian_product()
        .filter(|n_combo| n_combo.iter().all_unique())
        .collect();

    // Filter out puzzles that do not follow the unique N rules
    let mut n_valid_puzzle_groups: [Vec<Puzzle>; 4] =
        array::from_fn(|i| Vec::with_capacity(valid_puzzle_groups[i].len()));

    for (group_index, group) in valid_puzzle_groups.into_iter().enumerate() {
        for puzzle in group {
            if valid_n_combos
                .iter()
                .any(|combination| combination[group_index] == puzzle.n_digit())
            {
                n_valid_puzzle_groups[group_index].push(puzzle);
            }
        }
    }

    println!("{:?}", valid_n_combos);

    // Output the number of valid puzzles for each kind
    println!(
        "{} Valid puzzle P permutations",
        n_valid_puzzle_groups[0].len()
    );
    println!(
        "{} Valid puzzle Q permutations",
        n_valid_puzzle_groups[1].len()
    );
    println!(
        "{} Valid puzzle R permutations",
        n_valid_puzzle_groups[2].len()
    );
    println!(
        "{} Valid puzzle S permutations",
        n_valid_puzzle_groups[3].len()
    );

    for (index, group) in n_valid_puzzle_groups.iter().enumerate() {
        for puzzle in group {
            println!(
                "Group: {}, N: {}, Longitude: {}, Latitude: {}, Numbers: {:?}",
                ['P', 'Q', 'R', 'S'][index],
                puzzle.n_digit(),
                -puzzle.longitude(),
                puzzle.latitude(),
                puzzle.numbers()
            );
        }
    }
}
