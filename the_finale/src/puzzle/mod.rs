use std::{
    fmt::{self, Debug, Display},
    sync::Arc,
};

use position::{GridPosition, IdentifierVector, PuzzleDirection, PuzzleIdentifier};
use tinyvec::ArrayVec;
use wall::WallDirection;

pub mod position;
pub mod wall;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FitError {
    TooSmall,
    TooLarge,
    DigitsDoNotMatch,
}

/// The shared information used by all subpuzzles
#[derive(Debug, PartialEq, Eq)]
struct SharedPuzzle {
    num_columns: usize,
    num_rows: usize,
    identifiers: hashbrown::HashMap<PuzzleIdentifier, GridPosition>,
    walls: hashbrown::HashMap<GridPosition, WallDirection>,
}

/// The puzzle to solve through backtracking
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Puzzle {
    digits: Vec<Option<u8>>,
    shared: Arc<SharedPuzzle>,
}

impl Puzzle {
    /// Create a new puzzle of a given size, rules and locations where the walls are located
    pub fn new(
        num_columns: usize,
        num_rows: usize,
        identifiers: Vec<(PuzzleIdentifier, GridPosition)>,
        walls: Vec<(GridPosition, WallDirection)>,
    ) -> Self {
        // Create a unset grid of digits
        let digits = vec![None; num_columns * num_rows];

        // Convert identifiers into hashmap
        let identifiers = identifiers.into_iter().collect();

        // Convert walls into a hashmap
        let walls = walls.into_iter().collect();

        Self {
            digits,
            shared: Arc::new(SharedPuzzle {
                num_columns,
                num_rows,
                identifiers,
                walls,
            }),
        }
    }

    /// Get all puzzle identifiers
    pub fn identifers(&self) -> hashbrown::hash_map::Keys<'_, PuzzleIdentifier, GridPosition> {
        self.shared.identifiers.keys()
    }

    /// Lookup the identifer and convert it to an actual grid position
    pub fn identifer_to_grid(&self, identifier: PuzzleIdentifier) -> GridPosition {
        *self
            .shared
            .identifiers
            .get(&identifier)
            .expect("The provided identifier should have been valid")
    }

    /// Convert grid position to index
    fn position_to_index(&self, position: GridPosition) -> usize {
        assert!(position.column < self.shared.num_columns);
        assert!(position.row < self.shared.num_rows);

        position.column + (position.row * self.shared.num_columns)
    }

    /// Lookup digit at position
    pub fn digit_at(&self, position: GridPosition) -> Option<u8> {
        let digit_index = self.position_to_index(position);

        self.digits[digit_index]
    }

    /// Check if the position ahead of us in our direction is blocked
    fn is_direction_blocked(&self, position: GridPosition, direction: PuzzleDirection) -> bool {
        // Check if there is a wall ahead of us
        if let Some(&wall) = self.shared.walls.get(&position) {
            if wall.is_blocked(direction) {
                return true;
            }
        }

        // Check if we have reached the edge
        let new_position = match direction {
            PuzzleDirection::Across => position + GridPosition::new(1, 0),
            PuzzleDirection::Down => position + GridPosition::new(0, 1),
        };

        if new_position.column >= self.shared.num_columns {
            return true;
        }

        if new_position.row >= self.shared.num_rows {
            return true;
        }

        return false;
    }

    /// Get the digits at the identifier in the given direction
    fn digits_at(&self, vector: IdentifierVector) -> ArrayVec<[Option<u8>; 3]> {
        // Convert identifer to actual grid position
        let mut position = self.identifer_to_grid(vector.identifier);

        // Begin at current position and continue in the same direction until we hit a wall or the edge
        let mut buffer = ArrayVec::new();
        for _ in 0..3 {
            // Get digit at position
            let digit = self.digit_at(position);

            // Insert current digit
            buffer.push(digit);

            // Check if the path ahead of us is blocked
            if self.is_direction_blocked(position, vector.direction) {
                // We cannot go any further
                break;
            }

            // Increment row or column dependent on what direction we are travelling in
            match vector.direction {
                PuzzleDirection::Across => position.column += 1,
                PuzzleDirection::Down => position.row += 1,
            }
        }

        buffer
    }

    /// Get the number at a named position, returns None if the number has missing digits
    pub fn number_at(&self, vector: IdentifierVector) -> Option<usize> {
        // Get the digits at the position
        let digits = self.digits_at(vector);
        let mut full_digits = ArrayVec::<[u8; 3]>::new();

        for digit in digits {
            if digit.is_none() {
                // This is not a complete field
                return None;
            }

            // Add this digit as it is valid
            full_digits.push(digit.unwrap());
        }

        // Convert the digits to a number and return
        Some(Self::digits_to_number(&full_digits))
    }

    /// Set the digits at the identifier in the given direction,
    fn set_digits_at(&mut self, vector: IdentifierVector, digits: ArrayVec<[u8; 3]>) {
        // Convert identifer to actual grid position
        let mut position = self.identifer_to_grid(vector.identifier);

        // Begin at current position and continue in the same direction until we hit a wall or the edge
        for digit in digits {
            // Convert position to index
            let digit_index = self.position_to_index(position);

            self.digits[digit_index] = Some(digit);

            // Check if the path ahead of us is blocked
            if self.is_direction_blocked(position, vector.direction) {
                // We cannot go any further
                break;
            }

            // Increment row or column dependent on what direction we are travelling in
            match vector.direction {
                PuzzleDirection::Across => position.column += 1,
                PuzzleDirection::Down => position.row += 1,
            }
        }
    }

    /// Get the size of a field inside the puzzle
    pub fn size_of_field(&self, vector: IdentifierVector) -> usize {
        // Convert identifer to actual grid position
        let mut position = self.identifer_to_grid(vector.identifier);

        // Begin at current position and continue in the same direction until we hit a wall or the edge
        let mut i = 0;
        for _ in 0..3 {
            i += 1;

            // Check if the path ahead of us is blocked
            if self.is_direction_blocked(position, vector.direction) {
                // We cannot go any further
                break;
            }

            // Increment row or column dependent on what direction we are travelling in
            match vector.direction {
                PuzzleDirection::Across => position.column += 1,
                PuzzleDirection::Down => position.row += 1,
            }
        }

        i
    }

    /// Convert the provided number into digits, any extra digits are truncated
    fn number_to_digits(number: usize) -> ArrayVec<[u8; 3]> {
        let mut divisor: usize = 1;

        while number >= divisor * 10 {
            divisor *= 10
        }

        // Begin with the most significant digit, which is placed at index 0
        let mut buffer = ArrayVec::new();
        for _ in 0..3 {
            if divisor == 0 {
                break;
            }

            buffer.push(((number / divisor) % 10) as u8);

            divisor /= 10;
        }

        buffer
    }

    /// Convert the provided digits into a number
    fn digits_to_number(digits: &[u8]) -> usize {
        let mut multiplier = 10usize.pow(digits.len() as u32 - 1);

        let mut number = 0;

        for digit in digits {
            number += *digit as usize * multiplier;

            multiplier /= 10;
        }

        number
    }

    /// Check if a number can fit within a field
    fn can_digits_fit(
        &self,
        vector: IdentifierVector,
        num_digits: ArrayVec<[u8; 3]>,
    ) -> Result<(), FitError> {
        // Get the length of the field
        let field_length = self.size_of_field(vector);

        // Check if the number is too small to be stored
        if num_digits.len() < field_length {
            return Err(FitError::TooSmall);
        }

        // Check if the number is too large to be stored
        if num_digits.len() > field_length {
            return Err(FitError::TooLarge);
        }

        // Get the currently stored digits
        let current_digits = self.digits_at(vector);

        // Check if the digits match
        for (i, digit) in current_digits.into_iter().enumerate() {
            if digit.is_none() {
                // This slot is empty so we do not need to check anything
                continue;
            }

            if digit.unwrap() != num_digits[i] {
                return Err(FitError::DigitsDoNotMatch);
            }
        }

        Ok(())
    }

    /// Try and fit a number into a field, returns a new copy of the puzzle if the number fits
    pub fn try_fit_number(
        &self,
        vector: IdentifierVector,
        number: usize,
    ) -> Result<Self, FitError> {
        // Convert the number into digits
        let num_digits = Self::number_to_digits(number);

        // Check if the number can fit
        self.can_digits_fit(vector, num_digits).map(|_| {
            // Apply digits as we do fit
            // Then clone self so we can change the digits
            let mut new_puzzle = self.clone();

            new_puzzle.set_digits_at(vector, num_digits);

            new_puzzle
        })
    }

    /// Try and fit a set of numbers into a field, returns a new copy of the puzzle if all numbers in the set fit
    pub fn try_fit_numbers(&self, numbers: &[(IdentifierVector, usize)]) -> Result<Self, FitError> {
        // Then clone self so we can change the digits
        let mut new_puzzle = self.clone();

        for (vector, number) in numbers.iter().copied() {
            // Convert the number into digits
            // let num_digits = Self::number_to_digits(number);

            // Check if the number can fit
            // self.can_digits_fit(vector, num_digits)?;

            // Apply digits as we do fit
            new_puzzle = new_puzzle.try_fit_number(vector, number)?;
        }

        Ok(new_puzzle)
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Puzzle {}x{} [",
            self.shared.num_columns, self.shared.num_rows
        )?;

        // Output all the rows
        for row_index in 0..self.shared.num_rows {
            for column_index in 0..self.shared.num_columns {
                let current_position = GridPosition::new(column_index, row_index);

                let digit_index = self.position_to_index(current_position);

                // Write digit
                match self.digits[digit_index] {
                    Some(digit) => write!(f, "{}", digit),
                    None => write!(f, "N"),
                }?;

                // Write wall
                if column_index != self.shared.num_columns - 1
                    && self.is_direction_blocked(current_position, PuzzleDirection::Across)
                {
                    write!(f, "|")?;
                } else {
                    write!(f, " ")?;
                };
            }

            writeln!(f, "")?;

            if row_index == self.shared.num_rows - 1 {
                continue;
            }

            // Write under walls
            for column_index in 0..self.shared.num_columns {
                let current_position = GridPosition::new(column_index, row_index);

                // Write wall
                if self.is_direction_blocked(current_position, PuzzleDirection::Down) {
                    write!(f, "- ")?;
                } else {
                    write!(f, "  ")?;
                };
            }

            writeln!(f, "")?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use tinyvec::array_vec;

    use super::{
        position::{GridPosition, IdentifierVector, PuzzleDirection, PuzzleIdentifier},
        wall::WallDirection,
        FitError, Puzzle,
    };

    fn create_puzzle() -> Puzzle {
        Puzzle::new(
            6,
            4,
            vec![
                (PuzzleIdentifier(1), GridPosition::new(0, 0)),
                (PuzzleIdentifier(2), GridPosition::new(2, 0)),
                (PuzzleIdentifier(3), GridPosition::new(3, 0)),
                (PuzzleIdentifier(4), GridPosition::new(4, 0)),
                (PuzzleIdentifier(5), GridPosition::new(5, 0)),
                (PuzzleIdentifier(6), GridPosition::new(0, 1)),
                (PuzzleIdentifier(7), GridPosition::new(1, 1)),
                (PuzzleIdentifier(8), GridPosition::new(3, 1)),
                (PuzzleIdentifier(9), GridPosition::new(4, 1)),
                (PuzzleIdentifier(10), GridPosition::new(0, 2)),
                (PuzzleIdentifier(11), GridPosition::new(2, 2)),
                (PuzzleIdentifier(12), GridPosition::new(5, 2)),
                (PuzzleIdentifier(13), GridPosition::new(0, 3)),
                (PuzzleIdentifier(14), GridPosition::new(3, 3)),
            ],
            vec![
                (GridPosition::new(1, 0), WallDirection::Both),
                (GridPosition::new(3, 0), WallDirection::Right),
                (GridPosition::new(4, 0), WallDirection::Down),
                (GridPosition::new(0, 1), WallDirection::Down),
                (GridPosition::new(2, 1), WallDirection::Right),
                (GridPosition::new(5, 1), WallDirection::Down),
                (GridPosition::new(0, 2), WallDirection::Right),
                (GridPosition::new(1, 2), WallDirection::Right),
                (GridPosition::new(2, 2), WallDirection::Down),
                (GridPosition::new(3, 2), WallDirection::Both),
                (GridPosition::new(4, 2), WallDirection::Right),
                (GridPosition::new(2, 3), WallDirection::Right),
            ],
        )
    }

    #[test]
    fn test_try_fit() -> Result<(), FitError> {
        let puzzle = create_puzzle();

        let across_vector = IdentifierVector {
            identifier: PuzzleIdentifier(13),
            direction: PuzzleDirection::Across,
        };

        let down_vector = IdentifierVector {
            identifier: PuzzleIdentifier(7),
            direction: PuzzleDirection::Down,
        };

        let puzzle = puzzle.try_fit_number(down_vector, 123)?;

        puzzle
            .try_fit_number(across_vector, 456)
            .expect_err("This should not be possible");

        assert_eq!(puzzle.digit_at(GridPosition::new(1, 3)), Some(3));

        Ok(())
    }

    #[test]
    fn test_digit_conversion() {
        assert_eq!(Puzzle::number_to_digits(789), array_vec!(7, 8, 9));

        assert_eq!(Puzzle::digits_to_number(&[1, 2, 3]), 123);
    }

    #[test]
    fn test_digits() {
        let mut puzzle = create_puzzle();

        let test_identifiers =
            [1, 2, 4, 6, 8, 11, 13, 14].map(|identifier| PuzzleIdentifier(identifier));

        let mut i = 100;
        let mut sequence = iter::repeat_with(|| {
            let tmp = i;
            i += 1;
            tmp
        });

        for identifier in test_identifiers {
            let grid_vector = IdentifierVector::new(identifier, PuzzleDirection::Across);

            let field_length = puzzle.size_of_field(grid_vector);

            let result = puzzle.try_fit_number(grid_vector, sequence.next().unwrap());

            if field_length == 2 {
                puzzle = puzzle.try_fit_number(grid_vector, 99).unwrap()
            } else {
                assert!(result.is_ok());
                puzzle = result.unwrap();
            }
        }

        for identifier in test_identifiers {
            let grid_vector = IdentifierVector::new(identifier, PuzzleDirection::Across);

            let field_length = puzzle.size_of_field(grid_vector);

            let digits = puzzle.digits_at(grid_vector);
            let filled_digit_count = digits.iter().filter(|x| x.is_some()).count();

            assert_eq!(filled_digit_count, field_length);
        }
    }
}
