use grid::Grid;
use rules::*;

pub mod grid;
pub mod rules;

fn main() {
    // Create an array of all rules to apply
    let rules: &[fn(&Grid) -> bool] = &[clue_one];

    // Go through every possible grid
    for i in 0..Grid::max_permutations() {
        let puzzle_attempt = Grid::new(i);

        // Apply all clues
        let is_valid = rules.iter().all(|rule| rule(&puzzle_attempt));

        println!("{:?}", puzzle_attempt);
    }
    
}
