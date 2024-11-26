use super::position::PuzzleDirection;

/// The direction of the wall from the current tile
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WallDirection {
    Right,
    Down,
    /// The wall is both to the right and down of the current position
    Both,
}

impl WallDirection {
    /// Check the direction is blocked by this wall
    pub fn is_blocked(&self, direction: PuzzleDirection) -> bool {
        return match (self, direction) {
            (WallDirection::Right, PuzzleDirection::Across) => true,
            (WallDirection::Right, PuzzleDirection::Down) => false,
            (WallDirection::Down, PuzzleDirection::Across) => false,
            (WallDirection::Down, PuzzleDirection::Down) => true,
            (WallDirection::Both, _) => true,
        };
    }
}
