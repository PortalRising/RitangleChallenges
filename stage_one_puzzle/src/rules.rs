use crate::grid::Grid;

pub fn clue_one(grid: &Grid) -> bool {
    grid.numbers().iter().all(|&i| i < 3)
}
