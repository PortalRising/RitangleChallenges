#[derive(Clone, Copy, Debug)]
pub enum PuzzleDirection {
    Across,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub struct PuzzlePosition {
    index: u8,
    direction: PuzzleDirection,
}

impl PuzzlePosition {
    pub fn new(index: u8, direction: PuzzleDirection) -> Self {
        if index < 1 || index > 5 {
            panic!("The puzzle index must be between 1..5");
        }

        Self { index, direction }
    }

    pub fn new_across(index: u8) -> Self {
        Self::new(index, PuzzleDirection::Across)
    }

    pub fn new_down(index: u8) -> Self {
        Self::new(index, PuzzleDirection::Down)
    }

    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn direction(&self) -> PuzzleDirection {
        self.direction
    }
}
