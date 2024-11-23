use core::fmt;
use std::fmt::Debug;

/// A struct to store whether the current puzzle is valid
/// for the 4 different puzzles
pub struct ValidationResult {
    results: [bool; 4],
}

impl ValidationResult {
    /// Create a new result that sets all results to false
    pub fn new_invalid() -> Self {
        Self {
            results: [false; 4],
        }
    }

    pub fn p_mut(&mut self) -> &mut bool {
        &mut self.results[0]
    }

    pub fn q_mut(&mut self) -> &mut bool {
        &mut self.results[1]
    }

    pub fn r_mut(&mut self) -> &mut bool {
        &mut self.results[2]
    }

    pub fn s_mut(&mut self) -> &mut bool {
        &mut self.results[3]
    }

    /// Returns the result of the validation in the order of \[P, Q, R, S\]
    pub fn results(&self) -> [bool; 4] {
        self.results
    }

    // Checks if this puzzle fits the rules for any kind of puzzle
    pub fn is_any_valid(&self) -> bool {
        self.results.iter().any(|val| *val)
    }
}

impl Debug for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidationResult")
            .field("is_p_valid", &self.results[0])
            .field("is_q_valid", &self.results[1])
            .field("is_r_valid", &self.results[2])
            .field("is_s_valid", &self.results[3])
            .finish()
    }
}
