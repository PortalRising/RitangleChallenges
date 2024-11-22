use grid::Grid;
use rules::*;

pub mod grid;
pub mod primes;
pub mod rules;

fn main() {
    // Create an array of all rules to apply
    let rules: &[fn(&Grid) -> bool] = &[clue_one];

    // Store all the valid grids that follow the rules
    let mut valid_grids: Vec<Grid> = Vec::with_capacity(50);

    // Go through every possible grid
    for i in 0..Grid::max_permutations() {
        let puzzle_attempt = Grid::new(i);

        // Apply all clues
        let is_valid = rules.iter().all(|rule| rule(&puzzle_attempt));

        // Store all valid clues
        if is_valid {
            valid_grids.push(puzzle_attempt);
        }
    }

    // Output all valid grids
    for grid in &valid_grids {
        println!("{:?}", grid);
    }

    println!("{} Valid grid permutations", valid_grids.len());
}
