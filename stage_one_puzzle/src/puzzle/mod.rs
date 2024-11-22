use std::{
    array,
    fmt::{self, Debug},
};

use position::{PuzzleDirection, PuzzlePosition};

pub mod position;

pub struct Puzzle([u8; Self::NUM_OF_ELEMENTS]);

impl Puzzle {
    pub const NUM_OF_ELEMENTS: usize = 9;

    /// Convert the index to a digit within the grid
    fn index_to_digit(index: u32, digit_position: u8) -> u8 {
        let divisor = 10u32.pow(digit_position as u32);

        return ((index / divisor) % 10) as u8;
    }

    /// Get the max number of possible forms the grid can be expressed as
    pub fn max_permutations() -> u32 {
        10u32.pow(Self::NUM_OF_ELEMENTS as u32)
    }

    /// Get the numbers in the row at the provided offsets (truncating any that are shifted off the row)
    fn row(&self, index: usize, start_offset: usize, end_offset: usize) -> Vec<u8> {
        // Create start and end indexes, including the offset
        let start = (index * 3) + start_offset;
        let end = (index * 3) + 3 - end_offset;

        self.0[start..end].to_vec()
    }

    /// Get the numbers in the column at the provided offset (truncating any that are shifted off the column)
    fn column(&self, index: usize, start_offset: usize, end_offset: usize) -> Vec<u8> {
        let start = index + (start_offset * 3);
        let end = Self::NUM_OF_ELEMENTS - (end_offset * 3);
        let mut column = Vec::with_capacity(3);

        for i in (start..end).step_by(3) {
            column.push(self.0[i]);
        }

        return column;
    }

    /// Get the numbers along the provided position
    pub fn numbers_at(&self, position: PuzzlePosition) -> Vec<u8> {
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
        let mut total = 0;

        let numbers = self.numbers_at(position);
        let offset = numbers.len() as u32 - 1;

        for (i, &num) in numbers.iter().enumerate() {
            total += 10usize.pow(offset - (i as u32)) * (num as usize);
        }

        return total;
    }

    /// Get all the numbers in the grid
    pub fn numbers(&self) -> [u8; 9] {
        self.0
    }

    pub fn new(init_sequence: u32) -> Self {
        // Convert code into a grid of numbers
        let mut digits: [u8; 9] = array::from_fn(|i| Self::index_to_digit(init_sequence, i as u8));

        // Reverse the array to fix the ordering of the numbers
        digits.reverse();

        Self(digits)
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
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
}
