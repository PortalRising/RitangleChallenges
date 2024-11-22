use primes::PRIMES;
use puzzle::Puzzle;
use rules::RuleEnforcer;

pub mod primes;
pub mod puzzle;
pub mod rules;

fn main() {
    // Store all the valid grids that follow the rules
    let mut valid_grids: Vec<Puzzle> = Vec::with_capacity(50);

    // Go through every possible grid

    for i in 0..Puzzle::max_permutations() {
        if i % (Puzzle::max_permutations() / 50) == 0 {
            println!(
                "{}% complete",
                (i as f64 / Puzzle::max_permutations() as f64).floor() * 100.0
            );
        }

        // Convert the index into a valid puzzle
        let puzzle_attempt = Puzzle::new(i);

        // Create a rule enforcer
        let mut rule_enforcer = RuleEnforcer::new(&puzzle_attempt);

        // Apply all clues and store all valid clues
        if rule_enforcer.validate_q() {
            valid_grids.push(puzzle_attempt);
        }
    }

    // Output all valid grids
    for grid in &valid_grids {
        println!("{:?}", grid);
    }

    println!("{} Valid puzzle permutations", valid_grids.len());
}
