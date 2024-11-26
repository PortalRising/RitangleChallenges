use std::ops::{Add, AddAssign};

/// A direction on the puzzle grid
#[derive(Clone, Copy, Debug)]
pub enum PuzzleDirection {
    /// Represents right on the grid
    Across,
    /// Represents down on the grid
    Down,
}

/// A named position on the puzzle grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PuzzleIdentifier(pub usize);

/// A named vector on the puzzle grid
#[derive(Clone, Copy, Debug)]
pub struct IdentifierVector {
    pub identifier: PuzzleIdentifier,
    pub direction: PuzzleDirection,
}

impl IdentifierVector {
    /// Create a new vector at the identifier
    pub fn new(identifier: PuzzleIdentifier, direction: PuzzleDirection) -> Self {
        Self {
            identifier,
            direction,
        }
    }
}

/// Any point on the grid
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridPosition {
    pub column: usize,
    pub row: usize,
}

impl GridPosition {
    /// Create a new position on the grid
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }
}

impl Add for GridPosition {
    type Output = GridPosition;

    fn add(self, rhs: Self) -> Self::Output {
        GridPosition::new(self.column + rhs.column, self.row + rhs.row)
    }
}

impl AddAssign for GridPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.column += rhs.column;
        self.row += rhs.row;
    }
}
