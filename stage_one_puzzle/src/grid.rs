use std::{array, fmt::{self, Debug}};

pub struct Grid([u8; Self::NUM_OF_ELEMENTS]);

impl Grid {
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

    /// Get the row of the grid at an index
    pub fn row(&self, index: usize) -> [u8; 3] {
        let start = index * 3;
        let end = start + 3;

        self.0[start..end].try_into().unwrap()
    }

    /// Get the column of the grid at an index
    pub fn column(&self, index: usize) -> [u8; 3] {
        let first = self.0[index];
        let second = self.0[index + 3];
        let third = self.0[index + 6];

        [first, second, third]
    }


    pub fn new(init_sequence: u32) -> Self {
        // Convert code into a grid of numbers
        let digits: [u8; 9] = array::from_fn(|i| Self::index_to_digit(init_sequence, i as u8));

        Self(digits)
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Grid")
            .field("row_one", &self.row(0))
            .field("row_two", &self.row(1))
            .field("row_three", &self.row(2))
            .finish()
    }
}
