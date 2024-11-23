use std::{
    array,
    fmt::{self, Debug},
};

use position::{PuzzleDirection, PuzzlePosition};

pub mod position;

#[derive(Clone, Copy)]
pub struct Puzzle([usize; Self::NUM_OF_ELEMENTS]);

impl Puzzle {
    pub const NUM_OF_ELEMENTS: usize = 9;

    /// Convert the index to a digit within the grid
    fn index_to_digit(index: usize, digit_position: u8) -> usize {
        let divisor = 10usize.pow(digit_position as u32);

        (index / divisor) % 10
    }

    /// Get the max number of possible forms the grid can be expressed as
    pub fn max_permutations() -> u32 {
        10u32.pow(Self::NUM_OF_ELEMENTS as u32)
    }

    /// Convert a position on grid, starting at 0,0 symbolising the top left, to an array index
    #[inline]
    fn grid_position_to_index(column_index: usize, row_index: usize) -> usize {
        column_index + (row_index * 3)
    }

    /// Convert a position on grid, starting at 0,0 symbolising the top left, to the digit stored
    pub fn grid_position_to_digit(&self, column_index: usize, row_index: usize) -> usize {
        self.0[Self::grid_position_to_index(column_index, row_index)]
    }

    /// Join the list of numbers together where the first number is the most significant digit
    /// and the last number is the least significant
    fn join_digits(digits: &[usize]) -> usize {
        let mut total = 0;
        let offset = digits.len() as u32 - 1;

        for (i, &num) in digits.iter().enumerate() {
            total += 10usize.pow(offset - (i as u32)) * (num as usize);
        }

        return total;
    }

    /// Get the digits in the row at the provided offsets
    fn row(&self, index: usize, start_offset: usize, end_offset: usize) -> Vec<usize> {
        // Create start and end indexes, including the offset
        let start = Self::grid_position_to_index(start_offset, index);
        let end = Self::grid_position_to_index(3 - end_offset, index);

        self.0[start..end].to_vec()
    }

    /// Get the digits as one number in the row at the provided offsets
    fn joined_row(&self, index: usize, start_offset: usize, end_offset: usize) -> usize {
        let numbers = self.row(index, start_offset, end_offset);

        Self::join_digits(&numbers)
    }

    /// Get the digits in the column at the provided offset
    fn column(&self, index: usize, start_offset: usize, end_offset: usize) -> Vec<usize> {
        let start = Self::grid_position_to_index(index, start_offset);
        let end = Self::grid_position_to_index(index, 3 - end_offset);

        return (start..end).step_by(3).map(|i| self.0[i]).collect();
    }

    /// Get the digits as one number in the column at the provided offsets
    fn joined_column(&self, index: usize, start_offset: usize, end_offset: usize) -> usize {
        let numbers = self.column(index, start_offset, end_offset);

        Self::join_digits(&numbers)
    }

    /// Get the numbers along the provided position
    pub fn numbers_at(&self, position: PuzzlePosition) -> Vec<usize> {
        return match (position.index(), position.direction()) {
            (1, PuzzleDirection::Across) => self.row(0, 0, 0),
            (3, PuzzleDirection::Across) => self.row(1, 1, 0),
            (5, PuzzleDirection::Across) => self.row(2, 0, 0),
            (1, PuzzleDirection::Down) => self.column(0, 0, 0),
            (2, PuzzleDirection::Down) => self.column(1, 0, 1),
            (4, PuzzleDirection::Down) => self.column(2, 1, 0),
            (_, _) => unreachable!("Invalid position provided, which should be impossible"),
        };
    }

    /// Get the numbers joined together along the provided position
    pub fn joined_numbers_at(&self, position: PuzzlePosition) -> usize {
        let numbers = self.numbers_at(position);

        Self::join_digits(&numbers)
    }

    /// Get all the numbers in the grid
    pub fn numbers(&self) -> [usize; 9] {
        self.0
    }

    pub fn new(init_sequence: u32) -> Self {
        // Convert code into a grid of numbers
        let mut digits: [usize; 9] =
            array::from_fn(|i| Self::index_to_digit(init_sequence.try_into().unwrap(), i as u8));

        // Reverse the array to fix the ordering of the numbers
        digits.reverse();

        Self(digits)
    }

    /// Get the value of N in the puzzle
    pub fn n_digit(&self) -> usize {
        self.0[4]
    }

    /// Get the longitude the puzzle represents
    pub fn longitude(&self) -> f64 {
        // EF is the degrees west and GH are the corresponding minutes
        let mut fe = self.row(2, 1, 0);
        let mut hg = self.column(0, 1, 0);

        // Reverse the digits to get the right ordering of digits
        fe.reverse();
        hg.reverse();

        let ef = Self::join_digits(&fe) as f64;
        let gh = Self::join_digits(&hg) as f64;

        ef + (gh / 60.0)
    }

    /// Get the latitude the puzzle represents
    pub fn latitude(&self) -> f64 {
        // AB is the degrees north and CD are the corresponding minutes
        let ab = self.joined_row(0, 0, 1) as f64;
        let cd = self.joined_column(2, 0, 1) as f64;

        ab + (cd / 60.0)
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Puzzle")
            .field("row_one", &self.row(0, 0, 0))
            .field("row_two", &self.row(1, 0, 0))
            .field("row_three", &self.row(2, 0, 0))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle::position::PuzzlePosition;

    use super::Puzzle;

    #[test]
    fn test_numbers_at() {
        let puzzle = Puzzle::new(123456789);

        println!("{:?}", puzzle);

        // Test 1,3,5 across
        assert_eq!(
            puzzle.numbers_at(PuzzlePosition::new_across(1)),
            vec![1, 2, 3]
        );

        assert_eq!(puzzle.numbers_at(PuzzlePosition::new_across(3)), vec![5, 6]);

        assert_eq!(
            puzzle.numbers_at(PuzzlePosition::new_across(5)),
            vec![7, 8, 9]
        );

        // Test 1,2,4 down
        assert_eq!(
            puzzle.numbers_at(PuzzlePosition::new_down(1)),
            vec![1, 4, 7]
        );

        assert_eq!(puzzle.numbers_at(PuzzlePosition::new_down(2)), vec![2, 5]);

        assert_eq!(puzzle.numbers_at(PuzzlePosition::new_down(4)), vec![6, 9]);
    }

    #[test]
    fn test_joined_numbers_at() {
        let puzzle = Puzzle::new(123456789);

        println!("{:?}", puzzle);

        // Test 1,3,5 across
        assert_eq!(puzzle.joined_numbers_at(PuzzlePosition::new_across(1)), 123);

        assert_eq!(puzzle.joined_numbers_at(PuzzlePosition::new_across(3)), 56);

        assert_eq!(puzzle.joined_numbers_at(PuzzlePosition::new_across(5)), 789);

        // Test 1,2,4 down
        assert_eq!(puzzle.joined_numbers_at(PuzzlePosition::new_down(1)), 147);

        assert_eq!(puzzle.joined_numbers_at(PuzzlePosition::new_down(2)), 25);

        assert_eq!(puzzle.joined_numbers_at(PuzzlePosition::new_down(4)), 69);
    }

    #[test]
    fn test_joined_digits() {
        assert_eq!(Puzzle::join_digits(&[1, 2, 3]), 123);
    }
}
